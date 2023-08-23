use crate::{terminal::Terminal, document::Document, row::{Row, self}};
use termion::event::Key;            
use std::env;

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    quit_sign: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn init() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            Document::open(&args[1]).unwrap_or_default()
        } else {
            Document::default()
        };
        Self { 
            quit_sign: false, 
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
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

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.quit_sign = true,
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
        let height = self.document.len();
        let width = if let Some(row) = self.document.row(y) {
            row.len() 
        } else {
            0
        };
        
        match key {
            Key::Up => {
                y = y.saturating_sub(1)
            },
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            },
            Key::Left => {
                x = x.saturating_sub(1)
            },
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1)
                }
            },
            Key::PageUp => {
                y = 0
            },
            Key::PageDown => {
                y = height
            },
            _ => ()
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
        let end = start + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        for terminal_row in 0..self.terminal.size().height - 1  {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else {
                println!("~\r");
            }
        }
    }


}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}