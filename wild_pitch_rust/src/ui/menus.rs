use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum MenuType {
    Main,
    NewGame,
    LoadGame,
    Settings,
    GameMenu,
    TeamSelection,
    PlayerManagement,
    Statistics,
    Quit,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub action: MenuAction,
    pub enabled: bool,
    pub shortcut: Option<char>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuAction {
    NewGame,
    LoadGame,
    SaveGame,
    Settings,
    Statistics,
    TeamManagement,
    PlayerManagement,
    Resume,
    MainMenu,
    Quit,
    SubMenu(MenuType),
    Custom(String),
}

impl MenuItem {
    pub fn new(label: String, action: MenuAction) -> Self {
        Self {
            label,
            action,
            enabled: true,
            shortcut: None,
        }
    }

    pub fn with_shortcut(mut self, shortcut: char) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn display_text(&self) -> String {
        if let Some(shortcut) = self.shortcut {
            format!("[{}] {}", shortcut, self.label)
        } else {
            self.label.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub menu_type: MenuType,
    pub title: String,
    pub items: Vec<MenuItem>,
    pub selected_index: usize,
    pub is_active: bool,
}

impl Menu {
    pub fn new(menu_type: MenuType, title: String) -> Self {
        Self {
            menu_type,
            title,
            items: Vec::new(),
            selected_index: 0,
            is_active: false,
        }
    }

    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    pub fn add_items(&mut self, items: Vec<MenuItem>) {
        self.items.extend(items);
    }

    pub fn get_selected_item(&self) -> Option<&MenuItem> {
        self.items.get(self.selected_index)
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.items.len().saturating_sub(1);
        }
        self.skip_disabled_items();
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.items.len().saturating_sub(1) {
            self.selected_index += 1;
        } else {
            self.selected_index = 0;
        }
        self.skip_disabled_items();
    }

    fn skip_disabled_items(&mut self) {
        let start_index = self.selected_index;
        loop {
            if let Some(item) = self.items.get(self.selected_index) {
                if item.enabled {
                    break;
                }
            }
            
            if self.selected_index < self.items.len().saturating_sub(1) {
                self.selected_index += 1;
            } else {
                self.selected_index = 0;
            }
            
            // Prevent infinite loop if all items are disabled
            if self.selected_index == start_index {
                break;
            }
        }
    }

    pub fn handle_shortcut(&mut self, key: char) -> Option<&MenuAction> {
        for (index, item) in self.items.iter().enumerate() {
            if item.enabled && item.shortcut == Some(key) {
                self.selected_index = index;
                return Some(&item.action);
            }
        }
        None
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Create menu block
        let block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL)
            .border_style(if self.is_active {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            });

        // Create list items
        let list_items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let style = if index == self.selected_index {
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else if !item.enabled {
                    Style::default().fg(Color::Gray)
                } else {
                    Style::default().fg(Color::White)
                };

                let text = if index == self.selected_index {
                    format!("> {}", item.display_text())
                } else {
                    format!("  {}", item.display_text())
                };

                ListItem::new(text).style(style)
            })
            .collect();

        let list = List::new(list_items)
            .block(block)
            .highlight_style(Style::default().bg(Color::Blue));

        frame.render_widget(list, area);
    }
}

pub struct MenuManager {
    menus: Vec<Menu>,
    current_menu: usize,
    menu_stack: Vec<usize>,
}

impl MenuManager {
    pub fn new() -> Self {
        let mut manager = Self {
            menus: Vec::new(),
            current_menu: 0,
            menu_stack: Vec::new(),
        };

        manager.create_default_menus();
        manager
    }

    fn create_default_menus(&mut self) {
        // Main Menu
        let mut main_menu = Menu::new(MenuType::Main, "Wild Pitch - Main Menu".to_string());
        main_menu.add_items(vec![
            MenuItem::new("New Game".to_string(), MenuAction::SubMenu(MenuType::NewGame))
                .with_shortcut('n'),
            MenuItem::new("Load Game".to_string(), MenuAction::LoadGame)
                .with_shortcut('l'),
            MenuItem::new("Settings".to_string(), MenuAction::SubMenu(MenuType::Settings))
                .with_shortcut('s'),
            MenuItem::new("Statistics".to_string(), MenuAction::SubMenu(MenuType::Statistics))
                .with_shortcut('t'),
            MenuItem::new("Quit".to_string(), MenuAction::Quit)
                .with_shortcut('q'),
        ]);
        main_menu.is_active = true;
        self.menus.push(main_menu);

        // New Game Menu
        let mut new_game_menu = Menu::new(MenuType::NewGame, "New Game Setup".to_string());
        new_game_menu.add_items(vec![
            MenuItem::new("Quick Start".to_string(), MenuAction::NewGame)
                .with_shortcut('q'),
            MenuItem::new("Team Selection".to_string(), MenuAction::SubMenu(MenuType::TeamSelection))
                .with_shortcut('t'),
            MenuItem::new("Player Management".to_string(), MenuAction::SubMenu(MenuType::PlayerManagement))
                .with_shortcut('p'),
            MenuItem::new("Back to Main Menu".to_string(), MenuAction::MainMenu)
                .with_shortcut('b'),
        ]);
        self.menus.push(new_game_menu);

        // Settings Menu
        let mut settings_menu = Menu::new(MenuType::Settings, "Game Settings".to_string());
        settings_menu.add_items(vec![
            MenuItem::new("Difficulty Level".to_string(), MenuAction::Custom("difficulty".to_string()))
                .with_shortcut('d'),
            MenuItem::new("Game Options".to_string(), MenuAction::Custom("options".to_string()))
                .with_shortcut('o'),
            MenuItem::new("Display Settings".to_string(), MenuAction::Custom("display".to_string()))
                .with_shortcut('i'),
            MenuItem::new("Audio Settings".to_string(), MenuAction::Custom("audio".to_string()))
                .with_shortcut('a'),
            MenuItem::new("MLB Data Analysis".to_string(), MenuAction::Custom("mlb_analysis".to_string()))
                .with_shortcut('m'),
            MenuItem::new("Back to Main Menu".to_string(), MenuAction::MainMenu)
                .with_shortcut('b'),
        ]);
        self.menus.push(settings_menu);

        // Game Menu (for in-game options)
        let mut game_menu = Menu::new(MenuType::GameMenu, "Game Menu".to_string());
        game_menu.add_items(vec![
            MenuItem::new("Resume Game".to_string(), MenuAction::Resume)
                .with_shortcut('r'),
            MenuItem::new("Save Game".to_string(), MenuAction::SaveGame)
                .with_shortcut('s'),
            MenuItem::new("Load Game".to_string(), MenuAction::LoadGame)
                .with_shortcut('l'),
            MenuItem::new("Settings".to_string(), MenuAction::SubMenu(MenuType::Settings))
                .with_shortcut('e'),
            MenuItem::new("Main Menu".to_string(), MenuAction::MainMenu)
                .with_shortcut('m'),
            MenuItem::new("Quit".to_string(), MenuAction::Quit)
                .with_shortcut('q'),
        ]);
        self.menus.push(game_menu);

        // Statistics Menu
        let mut stats_menu = Menu::new(MenuType::Statistics, "Statistics".to_string());
        stats_menu.add_items(vec![
            MenuItem::new("Player Stats".to_string(), MenuAction::Custom("player_stats".to_string()))
                .with_shortcut('p'),
            MenuItem::new("Team Stats".to_string(), MenuAction::Custom("team_stats".to_string()))
                .with_shortcut('t'),
            MenuItem::new("League Leaders".to_string(), MenuAction::Custom("leaders".to_string()))
                .with_shortcut('l'),
            MenuItem::new("Game History".to_string(), MenuAction::Custom("history".to_string()))
                .with_shortcut('h'),
            MenuItem::new("Back to Main Menu".to_string(), MenuAction::MainMenu)
                .with_shortcut('b'),
        ]);
        self.menus.push(stats_menu);
    }

    pub fn get_current_menu(&self) -> &Menu {
        &self.menus[self.current_menu]
    }

    pub fn get_current_menu_mut(&mut self) -> &mut Menu {
        &mut self.menus[self.current_menu]
    }

    pub fn navigate_to_menu(&mut self, menu_type: MenuType) -> bool {
        for (index, menu) in self.menus.iter().enumerate() {
            if menu.menu_type == menu_type {
                self.menu_stack.push(self.current_menu);
                self.menus[self.current_menu].is_active = false;
                self.current_menu = index;
                self.menus[self.current_menu].is_active = true;
                return true;
            }
        }
        false
    }

    pub fn go_back(&mut self) -> bool {
        if let Some(previous_menu) = self.menu_stack.pop() {
            self.menus[self.current_menu].is_active = false;
            self.current_menu = previous_menu;
            self.menus[self.current_menu].is_active = true;
            true
        } else {
            false
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<MenuAction> {
        match key_event.code {
            KeyCode::Up => {
                self.get_current_menu_mut().move_up();
                None
            },
            KeyCode::Down => {
                self.get_current_menu_mut().move_down();
                None
            },
            KeyCode::Enter => {
                if let Some(item) = self.get_current_menu().get_selected_item() {
                    Some(item.action.clone())
                } else {
                    None
                }
            },
            KeyCode::Esc => {
                if self.go_back() {
                    None
                } else {
                    Some(MenuAction::Quit)
                }
            },
            KeyCode::Char(c) => {
                if let Some(action) = self.get_current_menu_mut().handle_shortcut(c) {
                    Some(action.clone())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    pub fn process_action(&mut self, action: MenuAction) -> Option<MenuAction> {
        match action {
            MenuAction::SubMenu(menu_type) => {
                self.navigate_to_menu(menu_type);
                None
            },
            MenuAction::MainMenu => {
                // Clear menu stack and go to main menu
                self.menu_stack.clear();
                self.menus[self.current_menu].is_active = false;
                self.current_menu = 0;
                self.menus[self.current_menu].is_active = true;
                None
            },
            _ => Some(action), // Return the action for the application to handle
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        self.get_current_menu().render(frame, area);
    }

    pub fn show_menu_overlay(&self, frame: &mut Frame, area: Rect) {
        // Calculate centered area for menu overlay
        let popup_area = Self::centered_rect(60, 50, area);
        
        // Clear the area
        frame.render_widget(Clear, popup_area);
        
        // Render the menu
        self.render(frame, popup_area);
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }

    pub fn is_main_menu(&self) -> bool {
        self.menus[self.current_menu].menu_type == MenuType::Main
    }

    pub fn get_current_menu_type(&self) -> &MenuType {
        &self.menus[self.current_menu].menu_type
    }
}