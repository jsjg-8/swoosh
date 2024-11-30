use color_eyre::Result;
use ratatui::{
    layout::{ Constraint, Direction, Layout, Rect },
    style::{ Color, Modifier, Style },
    text::Span,
    widgets::{ Block, Borders, Clear },
    Frame,
};

use crate::{ action::Action, components::Component, config::Config, tui::Event };

#[derive(Default)]
pub struct OptionsPanel {
    config: Config,
}

impl OptionsPanel {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for OptionsPanel {
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()> {
        f.render_widget(Clear, area);
        let block_style = if focused {
            self.config.styles
                .get(&crate::app::Mode::Home)
                .and_then(|x| x.get("focused"))
                .copied()
                .unwrap_or_default()
        } else {
            self.config.styles
                .get(&crate::app::Mode::Home)
                .and_then(|x| x.get("default"))
                .copied()
                .unwrap_or_default()
        };

        let panel = Block::default()
            .borders(Borders::ALL)
            .style(block_style)
            .title(Span::styled("Options", Style::default().add_modifier(Modifier::BOLD)));
        f.render_widget(panel.clone(), area);

        // Placeholder for options
        let options_area = panel.inner(area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(options_area);

        // Placeholder for Output Format
        let output_format_block = Block::default().title("Output Format").borders(Borders::ALL);
        f.render_widget(output_format_block, chunks[0]);

        // Placeholder for Resize Options
        let resize_options_block = Block::default().title("Resize Options").borders(Borders::ALL);
        f.render_widget(resize_options_block, chunks[1]);

        // Placeholder for Quality/Compression
        let quality_block = Block::default().title("Quality/Compression").borders(Borders::ALL);
        f.render_widget(quality_block, chunks[2]);

        Ok(())
    }

    fn update(&mut self, _action: Action) -> Result<Option<Action>> {
        Ok(None)
    }

    fn handle_events(&mut self, _event: Option<Event>) -> Result<Option<Action>> {
        Ok(None)
    }
}
