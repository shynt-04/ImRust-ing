use crate::{terminal::Terminal, document::Document, row::Row};
use termion::event::Key;            
use termion::color;
use std::time::{Duration, Instant};
use std::env;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(224,255,255);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(139, 0, 139);

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Copy, Clone)]
pub enum SearchDirection {
    Forward,
    Backward,
}

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self { text: message, time: Instant::now() }
    }
}

pub struct Editor {
    quit_sign: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    confirm_quit: bool, // Check unsaved changes
}

impl Editor {
    pub fn init() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut init_status = String::from("Command: CTRL-Q <Quit> | CTRL-S <Save> | CTRL-F <Find>");
        let document = if args.len() > 1 {
            let file_name = &args[1];
            let doc = Document::open(&file_name);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                init_status = format!("ERROR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };
        Self { 
            quit_sign: false, 
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
            status_message: StatusMessage::from(init_status),
            confirm_quit: false,
        }
    }
    
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {            
                die(error);            
            }            
            if self.quit_sign {            
                break;            
            }            
            if let Err(error) = self.process_keypress() {            
                die(error);            
            }
        }
    }

    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ", |_, _, _|{}).unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted".to_string());
                return;
            }
            self.document.file_name = new_name;
        }

        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved".to_string());
        } else {
            self.status_message = StatusMessage::from("An error has occurred while saving".to_string());
        }
    }

    fn search(&mut self) {
        let old_position = self.cursor_position.clone();
        let mut direction = SearchDirection::Forward;
        // Closures
        let query = self
                .prompt("Search (ESC <cancel> | Arrows <navigate>): ",
                    |editor, key, query: &String|{
                    let mut moved = false;
                    match key {
                        Key::Right | Key::Down => {
                            direction = SearchDirection::Forward;
                            editor.move_cursor(Key::Right);
                            moved = true;
                        },
                        Key::Left | Key::Up => {
                            direction = SearchDirection::Backward;

                        },
                        _ => direction = SearchDirection::Forward,
                    }
                    if let Some(position) = editor.document.find(&query, &editor.cursor_position, direction) {
                        editor.cursor_position = position;
                        editor.scroll();
                    } else if moved {
                        editor.move_cursor(Key::Left);
                    }
                }).unwrap_or(None);
        if query.is_none() {
            self.cursor_position = old_position;
            self.scroll();
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => {
                if self.confirm_quit == false && self.document.is_dirty() {
                    self.status_message = StatusMessage::from("File has unsaved changes. Press Ctrl-Q to confirm quit".to_string());
                    self.confirm_quit = true;
                    return Ok(());
                }
                self.quit_sign = true;
            },
            Key::Ctrl('s') => self.save(),
            Key::Ctrl('f') => self.search(),
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            },
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            },
            Key::Up |
            Key::Down |
            Key::Left |
            Key::Right |
            Key::PageUp |
            Key::PageDown => self.move_cursor(pressed_key),
            _ => ()
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position {x, y} = self.cursor_position;
        let height = self.terminal.size().height as usize;
        let width = self.terminal.size().width as usize;
        let mut offset = &mut self.offset;

        if  y < offset.y {
            offset.y = y
        } else if y > offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if  x < offset.x {
            offset.x = x
        } else if x > offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    pub fn move_cursor(&mut self, key: Key) {
        let Position{mut x, mut y} = self.cursor_position;
        let terminal_height = self.terminal.size().height as usize;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len() 
        } else {
            0
        };
        
        match key {
            Key::Up => {
                y = y.saturating_sub(1)
            },
            Key::Down => {
                if y + 1 < height {
                    y = y.saturating_add(1)
                }
            },
            Key::Left => {
                if x > 0 {
                    x = x - 1;
                } else if y > 0 {
                    y = y - 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                } 
            },
            Key::Right => {
                if x < width {
                    x = x + 1;
                } else if y < height {
                    y = y + 1;
                    x = 0;
                }
            },
            Key::PageUp => {
                y = if y > terminal_height {
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            },
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y.saturating_add(terminal_height)
                } else {
                    height
                }
            },
            _ => ()
        }

        width = if let Some(row) = self.document.row(y) {
            row.len() 
        } else {
            0
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position {x, y};

    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.quit_sign {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start: usize = self.offset.x;
        let end = start.saturating_add(width);
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        for terminal_row in 0..self.terminal.size().height  {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(self.offset.y.saturating_add(terminal_row as usize)) {
                self.draw_row(row);
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            "(modified)"
        } else {
            ""
        };
        let mut filename = "[No name]".to_string();
        if let Some(name) = &self.document.file_name {
            filename = name.clone();
            filename.truncate(20);
        }
        status = format!("{} - {} lines {}", filename, self.document.len(), modified_indicator);
        let line_indicator = format!(
            "{}/{}",
            std::cmp::min(self.cursor_position.y.saturating_add(1), self.document.len()),
            self.document.len()
        );
        let len = line_indicator.len() + status.len();
        status.push_str(&" ".repeat(width.saturating_sub(len)));
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }

    // displays a prompt in the status bar, and lets the user input a line of text after the prompt
    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, std::io::Error> 
        where C: FnMut(&mut Self, Key, &String)
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            let key = Terminal::read_key()?;
            match key {
                Key::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len().saturating_sub(1));
                    }
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    result.push(c);
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => ()
            }
            callback(self, key, &result);
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }

}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}