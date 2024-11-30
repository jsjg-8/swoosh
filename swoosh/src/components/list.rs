use std::collections::HashSet;

use color_eyre::Result;
use crossterm::event::KeyModifiers;
use ratatui::{
    layout::{ Constraint, Rect },
    style::{ Color, Modifier, Style, Stylize },
    text::Span,
    widgets::{ Block, Borders, Cell, Clear, Row, Table, TableState },
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    components::{ value::{ ImageInfo, ImageStatus }, Component },
    config::{ key_event_to_string, parse_key_sequence, Config },
    tui::Event,
};

#[derive(Default)]
pub struct ImageList {
    pub image_data: Vec<ImageInfo>,
    pub table_state: TableState,
    selected_indices: HashSet<usize>,
    last_selection: Option<usize>,
    config: Config,
    action_tx: Option<UnboundedSender<Action>>,
}

impl ImageList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_image(&mut self, image_info: ImageInfo) {
        self.image_data.push(image_info);
        if self.table_state.selected().is_none() && !self.image_data.is_empty() {
            self.table_state.select(Some(0));
        }
    }

    pub fn extend_selection(&mut self, new_index: usize) {
        if let Some(last) = self.last_selection {
            let start = last.min(new_index);
            let end = last.max(new_index);
            for i in start..=end {
                self.selected_indices.insert(i);
            }
        } else {
            self.selected_indices.insert(new_index);
        }
        self.last_selection = Some(new_index);
    }

    pub fn remove_image(&mut self, index: usize) {
        if index < self.image_data.len() {
            self.image_data.remove(index);
            if self.image_data.is_empty() {
                self.table_state.select(None);
            } else if let Some(selected) = self.table_state.selected() {
                if selected >= self.image_data.len() {
                    self.table_state.select(Some(self.image_data.len() - 1));
                }
            }
        }
    }

    pub fn remove_selected_images(&mut self) {
        // Sort selected indices in descending order to avoid index issues after removal
        let mut selected_indices = self.selected_indices.clone().into_iter().collect::<Vec<_>>();
        selected_indices.sort_by(|a, b| b.cmp(a)); // Sort in descending order to avoid index issues after removal

        for i in self.selected_indices.clone() {
            self.remove_image(i);
        }
        self.table_state.select(None); // Clear selection after removal
    }

    fn update_image_status(&mut self, index: usize, status: ImageStatus) {
        if let Some(image) = self.image_data.get_mut(index) {
            image.status = status;
        }
    }

    fn clear_images(&mut self) {
        self.image_data.clear();
        self.table_state.select(None);
    }
}

impl Component for ImageList {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>> {
        if let Some(Event::Key(key)) = event {
            let keybindings = self.config.keybindings.get(&crate::app::Mode::Home);
            if let Some(keybindings) = keybindings {
                if let Some(action) = keybindings.get(&vec![key]) {
                    match action {
                        Action::Down => {
                            if let Some(selected) = self.table_state.selected() {
                                if selected + 1 < self.image_data.len() {
                                    self.table_state.select(Some(selected + 1));
                                    if self.selected_indices.len() > 1 {
                                        self.selected_indices.clear();
                                        self.last_selection = None;
                                    }
                                }
                            } else if !self.image_data.is_empty() {
                                self.table_state.select(Some(0));
                                self.last_selection = self.table_state.selected();
                            }
                        }
                        Action::Up => {
                            if let Some(selected) = self.table_state.selected() {
                                if selected > 0 {
                                    self.table_state.select(Some(selected - 1));
                                    if self.selected_indices.len() > 1 {
                                        self.selected_indices.clear();
                                        self.last_selection = None;
                                    }
                                }
                            } else if !self.image_data.is_empty() {
                                self.table_state.select(Some(self.image_data.len() - 1));
                                self.last_selection = self.table_state.selected();
                            }
                        }
                        Action::ShiftUp => {
                            if let Some(selected) = self.table_state.selected() {
                                if selected > 0 {
                                    self.extend_selection(selected);
                                    self.table_state.select(Some(selected - 1));
                                }
                            }
                        }
                        Action::ShiftDown => {
                            if let Some(selected) = self.table_state.selected() {
                                if selected + 1 < self.image_data.len() {
                                    self.extend_selection(selected);
                                    self.table_state.select(Some(selected + 1));
                                }
                            }
                        }
                        Action::Select => {
                            if let Some(selected) = self.table_state.selected() {
                                if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    // Shift-click selection
                                    if let Some(last) = self.last_selection {
                                        let start = last.min(selected);
                                        let end = last.max(selected);
                                        for i in start..=end {
                                            self.selected_indices.insert(i);
                                        }
                                    }
                                } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                                    // Ctrl-click selection (toggle)
                                    if self.selected_indices.contains(&selected) {
                                        self.selected_indices.remove(&selected);
                                    } else {
                                        self.selected_indices.insert(selected);
                                    }
                                } else {
                                    // Normal click selection (clear previous selections)
                                    self.selected_indices.clear();
                                    self.selected_indices.insert(selected);
                                }
                                self.last_selection = Some(selected);
                            }
                        }
                        Action::Delete => {
                            if !self.selected_indices.is_empty() {
                                self.remove_selected_images();
                                self.table_state.select(
                                    Some(self.selected_indices.clone().into_iter().min().unwrap())
                                );
                                self.selected_indices.clear();
                            } else if let Some(selected) = self.table_state.selected() {
                                self.remove_image(selected);
                                self.table_state.select(Some(selected)); // Clear selection after removal
                            }
                        }

                        _ => {}
                    }
                }
            }
        }
        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ClearImages => self.clear_images(),
            Action::UpdateImageStatus { index, status } => {
                self.update_image_status(index, status);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame, area: Rect, _focused: bool) -> Result<()> {
        f.render_widget(Clear, area);

        let rows = self.image_data
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let status_style = match item.status {
                    ImageStatus::Queued => Style::default().fg(Color::Gray),
                    ImageStatus::Converting => Style::default().fg(Color::Yellow),
                    ImageStatus::Completed => Style::default().fg(Color::Green),
                    ImageStatus::Error(_) => Style::default().fg(Color::Red),
                };
                let mut row_style = Style::default();
                if self.table_state.selected() == Some(i) {
                    row_style = row_style.on_cyan();
                }
                if self.selected_indices.contains(&i) {
                    row_style = row_style.bg(Color::LightBlue); // Or another visual cue
                }
                Row::new(
                    vec![
                        Cell::from(item.filename.clone()),
                        Cell::from(item.size.clone()),
                        Cell::from(Span::styled(item.status.to_string(), status_style))
                    ]
                ).style(row_style)
            });
        let block_style = {
            if _focused {
                self.config.styles
                    .get(&crate::app::Mode::Home)
                    .and_then(|x| x.get("focused")) // Style for focused block
                    .copied()
                    .unwrap_or_default()
            } else {
                self.config.styles
                    .get(&crate::app::Mode::Home)
                    .and_then(|x| x.get("default"))
                    .copied()
                    .unwrap_or_default()
            }
        };

        let table = Table::new(
            rows,
            &[Constraint::Percentage(70), Constraint::Percentage(15), Constraint::Percentage(15)]
        )
            .header(
                Row::new(vec!["Filename", "Size", "Status"])
                    .style(
                        self.config.styles
                            .get(&crate::app::Mode::Home)
                            .and_then(|x| x.get("table_header"))
                            .copied()
                            .unwrap_or_default()
                            .add_modifier(Modifier::BOLD)
                    )
                    .bottom_margin(1)
            )
            .block(Block::default().borders(Borders::ALL).title("Images").border_style(block_style))
            .row_highlight_style(
                self.config.styles
                    .get(&crate::app::Mode::Home)
                    .and_then(|x| x.get("highlighted"))
                    .copied()
                    .unwrap_or_default()
            );

        f.render_stateful_widget(table, area, &mut self.table_state);
        Ok(())
    }
}
