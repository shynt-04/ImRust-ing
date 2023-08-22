use crate::terminal::Terminal;
use termion::event::Key;            

pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Editor {
    quit_sign: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn init() -> Self {
        Self { 
            quit_sign: false, 
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position { x: 0, y: 0 },
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
            _ => {
                if let Key::Char(c) = pressed_key {
                    print!("{c}")
                }
            },
        }
        Ok(())
    }

    pub fn move_cursor(&mut self, key: Key) {
        let Position{mut x, mut y} = self.cursor_position;
        match key {
            Key::Up => {
                y = y.saturating_sub(1)
            },
            Key::Down => {
                y = y.saturating_add(1)
            },
            Key::Left => {
                x = x.saturating_sub(1)
            },
            Key::Right => {
                x = x.saturating_add(1)
            },
            Key::PageUp => {
                y = 0
            },
            Key::PageDown => {
                y = self.terminal.size().height
            },
            _ => ()
        }
        self.cursor_position = Position {x, y};

    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.quit_sign {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&Position { x: 0, y: 0 });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1  {
            Terminal::clear_current_line();
            println!("~\r")
        }
    }

}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}