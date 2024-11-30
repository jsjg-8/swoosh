use std::path::PathBuf;

use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{ layout::{ Constraint, Direction, Layout }, prelude::Rect };
use serde::{ Deserialize, Serialize };
use tokio::sync::mpsc;
use tracing::{ debug, info };

use crate::{
    action::Action,
    components::{ list::ImageList, value::ImageInfo, options::OptionsPanel, Component },
    config::Config,
    tui::{ Event, Tui },
};

pub struct App {
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
    focused_component: Focus,
    left_panel_percentage: u16,
    image_list: ImageList,
    options_panel: OptionsPanel,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)] // New: Focus enum
pub enum Focus {
    ImageList,
    OptionsPanel, // Add when OptionsPanel is implemented
}

impl Default for App {
    fn default() -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Self {
            config: Config::default(),
            tick_rate: 24.0,
            frame_rate: 60.0,
            should_quit: false,
            should_suspend: false,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
            focused_component: Focus::ImageList,
            left_panel_percentage: 60,
            image_list: ImageList::new(),
            options_panel: OptionsPanel::new(),
        }
    }
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64, images: Option<Vec<PathBuf>>) -> Result<Self> {
        let mut app = App {
            tick_rate,
            frame_rate,
            config: Config::new()?,
            // options_panel: OptionsPanel::new(),
            ..Default::default()
        };
        if let Some(images) = images {
            for image_path in images {
                if let Ok(image_info) = ImageInfo::new(image_path) {
                    app.image_list.add_image(image_info);
                } else {
                    // Handle invalid image paths (e.g., display an error message)
                }
            }
        }
        Ok(app)
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?.mouse(true).tick_rate(self.tick_rate).frame_rate(self.frame_rate);
        tui.enter()?;

        self.image_list.register_action_handler(self.action_tx.clone())?;
        self.image_list.register_config_handler(self.config.clone())?;
        self.image_list.init(tui.size()?)?;

        let action_tx = self.action_tx.clone();
        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::ClearScreen)?;
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        }
        if let Some(action) = self.image_list.handle_events(Some(event.clone()))? {
            action_tx.send(action)?;
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.config.keybindings.get(&self.mode) else {
            return Ok(());
        };
        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Got action: {action:?}");
                action_tx.send(action.clone())?;
            }
            _ => {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    info!("Got action: {action:?}");
                    action_tx.send(action.clone())?;
                }
            }
        }
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            if action != Action::Tick && action != Action::Render {
                debug!("{action:?}");
            }
            match action {
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                }
                Action::Quit => {
                    self.should_quit = true;
                }
                Action::Suspend => {
                    self.should_suspend = true;
                }
                Action::Resume => {
                    self.should_suspend = false;
                }
                Action::ClearScreen => tui.terminal.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                _ => {}
            }
            if let Some(action) = self.image_list.update(action.clone())? {
                self.action_tx.send(action)?;
            }
        }
        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(self.left_panel_percentage),
                    Constraint::Percentage(100 - self.left_panel_percentage),
                ])
                .split(frame.area());

            let focused = self.focused_component == Focus::ImageList;
            self.image_list
                .draw(frame, chunks[0], self.focused_component == Focus::ImageList)
                .unwrap();
            self.options_panel.draw(frame, chunks[1], focused).unwrap();

            // Render other components (OptionsPanel, FpsCounter) with focus information
            // ...
        })?;
        Ok(())
    }
}
