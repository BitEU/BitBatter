use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone, PartialEq)]
pub enum DialogType {
    Confirmation,
    Information,
    Warning,
    Error,
    Input,
    Selection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DialogResult {
    Ok,
    Cancel,
    Yes,
    No,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Dialog {
    pub dialog_type: DialogType,
    pub title: String,
    pub message: String,
    pub buttons: Vec<String>,
    pub selected_button: usize,
    pub is_visible: bool,
    pub input_text: String,
    pub options: Vec<String>,
    pub selected_option: usize,
}

impl Dialog {
    pub fn confirmation(title: String, message: String) -> Self {
        Self {
            dialog_type: DialogType::Confirmation,
            title,
            message,
            buttons: vec!["Yes".to_string(), "No".to_string()],
            selected_button: 0,
            is_visible: false,
            input_text: String::new(),
            options: Vec::new(),
            selected_option: 0,
        }
    }

    pub fn information(title: String, message: String) -> Self {
        Self {
            dialog_type: DialogType::Information,
            title,
            message,
            buttons: vec!["OK".to_string()],
            selected_button: 0,
            is_visible: false,
            input_text: String::new(),
            options: Vec::new(),
            selected_option: 0,
        }
    }

    pub fn warning(title: String, message: String) -> Self {
        Self {
            dialog_type: DialogType::Warning,
            title,
            message,
            buttons: vec!["OK".to_string(), "Cancel".to_string()],
            selected_button: 0,
            is_visible: false,
            input_text: String::new(),
            options: Vec::new(),
            selected_option: 0,
        }
    }

    pub fn error(title: String, message: String) -> Self {
        Self {
            dialog_type: DialogType::Error,
            title,
            message,
            buttons: vec!["OK".to_string()],
            selected_button: 0,
            is_visible: false,
            input_text: String::new(),
            options: Vec::new(),
            selected_option: 0,
        }
    }

    pub fn input(title: String, message: String) -> Self {
        Self {
            dialog_type: DialogType::Input,
            title,
            message,
            buttons: vec!["OK".to_string(), "Cancel".to_string()],
            selected_button: 0,
            is_visible: false,
            input_text: String::new(),
            options: Vec::new(),
            selected_option: 0,
        }
    }

    pub fn selection(title: String, message: String, options: Vec<String>) -> Self {
        Self {
            dialog_type: DialogType::Selection,
            title,
            message,
            buttons: vec!["OK".to_string(), "Cancel".to_string()],
            selected_button: 0,
            is_visible: false,
            input_text: String::new(),
            options,
            selected_option: 0,
        }
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<DialogResult> {
        if !self.is_visible {
            return None;
        }

        match key_event.code {
            KeyCode::Left => {
                if matches!(self.dialog_type, DialogType::Selection) {
                    // Navigate options
                    if self.selected_option > 0 {
                        self.selected_option -= 1;
                    } else {
                        self.selected_option = self.options.len().saturating_sub(1);
                    }
                } else {
                    // Navigate buttons
                    if self.selected_button > 0 {
                        self.selected_button -= 1;
                    } else {
                        self.selected_button = self.buttons.len().saturating_sub(1);
                    }
                }
                None
            },
            KeyCode::Right => {
                if matches!(self.dialog_type, DialogType::Selection) {
                    // Navigate options
                    if self.selected_option < self.options.len().saturating_sub(1) {
                        self.selected_option += 1;
                    } else {
                        self.selected_option = 0;
                    }
                } else {
                    // Navigate buttons
                    if self.selected_button < self.buttons.len().saturating_sub(1) {
                        self.selected_button += 1;
                    } else {
                        self.selected_button = 0;
                    }
                }
                None
            },
            KeyCode::Up => {
                if matches!(self.dialog_type, DialogType::Selection) {
                    if self.selected_option > 0 {
                        self.selected_option -= 1;
                    } else {
                        self.selected_option = self.options.len().saturating_sub(1);
                    }
                }
                None
            },
            KeyCode::Down => {
                if matches!(self.dialog_type, DialogType::Selection) {
                    if self.selected_option < self.options.len().saturating_sub(1) {
                        self.selected_option += 1;
                    } else {
                        self.selected_option = 0;
                    }
                }
                None
            },
            KeyCode::Enter => {
                self.hide();
                Some(self.get_result())
            },
            KeyCode::Esc => {
                self.hide();
                Some(DialogResult::Cancel)
            },
            KeyCode::Char(c) => {
                if matches!(self.dialog_type, DialogType::Input) {
                    self.input_text.push(c);
                }
                None
            },
            KeyCode::Backspace => {
                if matches!(self.dialog_type, DialogType::Input) {
                    self.input_text.pop();
                }
                None
            },
            _ => None,
        }
    }

    fn get_result(&self) -> DialogResult {
        match self.dialog_type {
            DialogType::Confirmation => {
                match self.selected_button {
                    0 => DialogResult::Yes,
                    1 => DialogResult::No,
                    _ => DialogResult::Cancel,
                }
            },
            DialogType::Information | DialogType::Error => DialogResult::Ok,
            DialogType::Warning => {
                match self.selected_button {
                    0 => DialogResult::Ok,
                    _ => DialogResult::Cancel,
                }
            },
            DialogType::Input => {
                match self.selected_button {
                    0 => DialogResult::Custom(self.input_text.clone()),
                    _ => DialogResult::Cancel,
                }
            },
            DialogType::Selection => {
                match self.selected_button {
                    0 => {
                        if let Some(option) = self.options.get(self.selected_option) {
                            DialogResult::Custom(option.clone())
                        } else {
                            DialogResult::Cancel
                        }
                    },
                    _ => DialogResult::Cancel,
                }
            },
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if !self.is_visible {
            return;
        }

        // Calculate dialog size based on content
        let dialog_width = (self.message.len() + 10).min(60).max(30) as u16;
        let dialog_height = self.calculate_height();
        
        let dialog_area = Self::centered_rect(dialog_width, dialog_height, area);

        // Clear the background
        frame.render_widget(Clear, dialog_area);

        // Create the dialog layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(1),    // Content
                Constraint::Length(3), // Buttons
            ])
            .split(dialog_area);

        // Render title
        let title_style = match self.dialog_type {
            DialogType::Error => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            DialogType::Warning => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            DialogType::Information => Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            _ => Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        };

        let title_block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL)
            .border_style(title_style);

        frame.render_widget(title_block, chunks[0]);

        // Render content
        let content_area = chunks[1];
        self.render_content(frame, content_area);

        // Render buttons
        self.render_buttons(frame, chunks[2]);
    }

    fn render_content(&self, frame: &mut Frame, area: Rect) {
        match self.dialog_type {
            DialogType::Input => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(1),    // Message
                        Constraint::Length(3), // Input field
                    ])
                    .split(area);

                // Message
                let message_paragraph = Paragraph::new(self.message.clone())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Left);
                frame.render_widget(message_paragraph, chunks[0]);

                // Input field
                let input_block = Block::default()
                    .title("Input")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow));

                let input_paragraph = Paragraph::new(self.input_text.clone())
                    .block(input_block)
                    .style(Style::default().fg(Color::White));

                frame.render_widget(input_paragraph, chunks[1]);
            },
            DialogType::Selection => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(2), // Message
                        Constraint::Min(1),    // Options
                    ])
                    .split(area);

                // Message
                let message_paragraph = Paragraph::new(self.message.clone())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Left);
                frame.render_widget(message_paragraph, chunks[0]);

                // Options
                let options_block = Block::default()
                    .title("Options")
                    .borders(Borders::ALL);

                let list_items: Vec<ListItem> = self.options
                    .iter()
                    .enumerate()
                    .map(|(index, option)| {
                        let style = if index == self.selected_option {
                            Style::default()
                                .bg(Color::Blue)
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };

                        let text = if index == self.selected_option {
                            format!("> {}", option)
                        } else {
                            format!("  {}", option)
                        };

                        ListItem::new(text).style(style)
                    })
                    .collect();

                let options_list = List::new(list_items).block(options_block);
                frame.render_widget(options_list, chunks[1]);
            },
            _ => {
                // Simple message display
                let message_paragraph = Paragraph::new(self.message.clone())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center);
                frame.render_widget(message_paragraph, area);
            },
        }
    }

    fn render_buttons(&self, frame: &mut Frame, area: Rect) {
        let button_constraints: Vec<Constraint> = self.buttons
            .iter()
            .map(|_| Constraint::Min(10))
            .collect();

        let button_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(button_constraints)
            .split(area);

        for (index, button_text) in self.buttons.iter().enumerate() {
            if let Some(chunk) = button_chunks.get(index) {
                let style = if index == self.selected_button {
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let button_block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(style);

                let button_paragraph = Paragraph::new(button_text.clone())
                    .block(button_block)
                    .alignment(Alignment::Center)
                    .style(style);

                frame.render_widget(button_paragraph, *chunk);
            }
        }
    }

    fn calculate_height(&self) -> u16 {
        let base_height = 6; // Title + buttons
        let content_height = match self.dialog_type {
            DialogType::Input => 5,
            DialogType::Selection => (self.options.len() as u16 + 3).min(10),
            _ => (self.message.len() / 50 + 2) as u16,
        };
        (base_height + content_height).min(20)
    }

    fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length((r.height.saturating_sub(height)) / 2),
                Constraint::Length(height),
                Constraint::Length((r.height.saturating_sub(height)) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((r.width.saturating_sub(width)) / 2),
                Constraint::Length(width),
                Constraint::Length((r.width.saturating_sub(width)) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

pub struct DialogManager {
    current_dialog: Option<Dialog>,
}

impl DialogManager {
    pub fn new() -> Self {
        Self {
            current_dialog: None,
        }
    }

    pub fn show_dialog(&mut self, mut dialog: Dialog) {
        dialog.show();
        self.current_dialog = Some(dialog);
    }

    pub fn hide_dialog(&mut self) {
        self.current_dialog = None;
    }

    pub fn has_dialog(&self) -> bool {
        self.current_dialog.as_ref().map_or(false, |d| d.is_visible)
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<DialogResult> {
        if let Some(ref mut dialog) = self.current_dialog {
            if let Some(result) = dialog.handle_key_event(key_event) {
                self.current_dialog = None;
                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if let Some(ref dialog) = self.current_dialog {
            dialog.render(frame, area);
        }
    }
}