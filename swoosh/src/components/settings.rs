// src/ui/settings.rs

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget}
};
use bitflags::bitflags;


bitflags! {
    pub struct TransformFlags: u32 {
        const RESIZE = 0b00000001;
        const ROTATE = 0b00000010;
        const FLIP = 0b00000100;
        const BLUR = 0b00001000;
        const UNSHARPEN = 0b00010000;
        const CROP = 0b00100000;
        const FILTER3X3 = 0b01000000;
        const BRIGHTEN = 0b10000000;
        const CONTRAST = 0b100000000;
        const HUEROTATE = 0b1000000000;
    }
}



pub struct SettingsPanel<'a> {
    pub transform_flags: TransformFlags,
    pub items: Vec<(&'a str, TransformFlags)>,
    pub resize_width: u32,
    pub resize_height: u32,
    pub preserve_aspect_ratio: bool,

    // Rotations
    pub rotate_degrees: i32,

    // Flips
    pub flip_horizontal: bool,
    pub flip_vertical: bool,

    // Blur and Unsharpen
    pub blur_sigma: f32,
    pub unsharpen_sigma: f32,
    pub unsharpen_threshold: i32,
    pub crop_x: u32,
    pub crop_y: u32,
    pub crop_width: u32,
    pub crop_height: u32,
    pub filter3x3_kernel: [f32; 9],
    pub brighten_value: i32,
    pub contrast_value: f32,
    pub huerotate_value: i32,
}

pub struct SettingsPanelWidget<'a> {
    settings: &'a mut SettingsPanel<'a>
}


impl<'a> SettingsPanel<'a>  {
    pub fn new() -> Self {
        SettingsPanel {
            transform_flags: TransformFlags::empty(),
            items: vec![
                ("Resize", TransformFlags::RESIZE),
                ("Rotate", TransformFlags::ROTATE),
                ("Flip", TransformFlags::FLIP),
                ("Blur", TransformFlags::BLUR),
                ("Unsharpen", TransformFlags::UNSHARPEN),
                ("Crop", TransformFlags::CROP),
                ("Filter 3x3", TransformFlags::FILTER3X3),
                ("Brighten", TransformFlags::BRIGHTEN),
                ("Contrast", TransformFlags::CONTRAST),
                ("Hue Rotate", TransformFlags::HUEROTATE),
                ],
            resize_width: 800,
            resize_height: 600,
            preserve_aspect_ratio: true,
            rotate_degrees: 90,
            flip_horizontal: false,
            flip_vertical: false,
            blur_sigma: 1.0,
            unsharpen_sigma: 1.0,
            unsharpen_threshold: 1,
            crop_x: 0,
            crop_y: 0,
            crop_width: 100,
            crop_height: 100,
            filter3x3_kernel: [0.0; 9],
            brighten_value: 0,
            contrast_value: 0.0,
            huerotate_value: 0,
        }
    }



    pub fn render(&'a mut self) -> SettingsPanelWidget<'a> {
        SettingsPanelWidget {
            settings: self,
        }
    }
}



impl<'a> StatefulWidget for SettingsPanelWidget<'a> {
    type State = ListState;

    fn render(mut self, area: Rect, buf: &mut ratatui::buffer::Buffer, state: &mut Self::State) {
        let items: Vec<ListItem> = self.settings
            .items
            .iter()
            .map(|(name, flag)| {
                let mut spans = vec![Span::raw(format!("{} ", name))];
                if self.settings.transform_flags.contains(*flag) {
                    spans.push(Span::styled("[x]", Style::default().fg(Color::Green))); // Indicate enabled
                } else {
                    spans.push(Span::styled("[ ]", Style::default().fg(Color::Gray))); // Indicate disabled
                }
                ListItem::new(Line::from(spans))
            })
            .collect();
        let list = List::new(items) // existing list widget creation
            .block(Block::default().borders(Borders::ALL).title("Transformations"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> ");

        let mut settings_text = Vec::new(); // Start with an empty Vec


        // Add settings text only for ENABLED transformations
        if let Some(selected_index) = state.selected() {
            let (_, selected_flag) = self.settings.items[selected_index];

            if self.settings.transform_flags.contains(selected_flag) { // Only if enabled
                match selected_flag {
                    TransformFlags::RESIZE => {
                        settings_text.push(Line::from(vec![Span::raw(format!("Width: {}", self.settings.resize_width))]));
                        settings_text.push(Line::from(vec![Span::raw(format!("Height: {}", self.settings.resize_height))]));
                        settings_text.push(Line::from(vec![Span::raw(format!("Preserve Aspect Ratio: {}", self.settings.preserve_aspect_ratio))]));
                    }
                    TransformFlags::ROTATE => {
                        settings_text.push(Line::from(vec![Span::raw(format!("Degrees: {}", self.settings.rotate_degrees))]));
                    }
                    TransformFlags::FLIP => {
                        settings_text.push(Line::from(vec![Span::raw(format!("Horizontal: {}", self.settings.flip_horizontal))]));
                        settings_text.push(Line::from(vec![Span::raw(format!("Vertical: {}", self.settings.flip_vertical))]));
                    }
                    TransformFlags::BLUR => {
                        settings_text.push(Line::from(vec![Span::raw(format!("Sigma: {}", self.settings.blur_sigma))]));
                    }
                    TransformFlags::UNSHARPEN => {
                        settings_text.push(Line::from(vec![Span::raw(format!("Sigma: {}", self.settings.unsharpen_sigma))]));
                        settings_text.push(Line::from(vec![Span::raw(format!("Threshold: {}", self.settings.unsharpen_threshold))]));
                    }
                    TransformFlags::CROP => {
                        settings_text.push(Line::from(vec![Span::raw(format!("X: {}", self.settings.crop_x))]));
                        settings_text.push(Line::from(vec![Span::raw(format!("Y: {}", self.settings.crop_y))]));
                        settings_text.push(Line::from(vec![Span::raw(format!("Width: {}", self.settings.crop_width))]));
                        settings_text.push(Line::from(vec![Span::raw(format!("Height: {}", self.settings.crop_height))]));
                    }
                    TransformFlags::FILTER3X3 => {
                        settings_text.extend(self.settings.filter3x3_kernel.iter().enumerate().map(|(i, &val)| {
                            Line::from(vec![Span::raw(format!("Kernel[{}]: {}", i, val))])
                        }));
                    }
                    TransformFlags::BRIGHTEN => {
                        settings_text.push(Line::from(vec![Span::raw(format!("Value: {}", self.settings.brighten_value))]));
                    }
                    TransformFlags::CONTRAST => {
                        settings_text.push(Line::from(vec![Span::raw(format!("Value: {}", self.settings.contrast_value))]));
                    }
                    TransformFlags::HUEROTATE => {
                        settings_text.push(Line::from(vec![Span::raw(format!("Value: {}", self.settings.huerotate_value))]));
                    }
                    _ => {}
                }
            }
        }


        let settings_paragraph = if settings_text.is_empty() {
            Paragraph::new("Select a transformation to configure").style(Style::default().fg(Color::Gray))
        } else {
            Paragraph::new(settings_text).style(Style::default().fg(Color::White))
        };


        let settings_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(6)].as_ref())
            .split(area);

        
    }
}