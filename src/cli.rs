// CLI //

use clap::Parser;

#[derive(Parser)]
#[command(name = "lxsp")]
//#[command(author = "Nonymous A. <admin@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
pub struct ArgStruct {
    #[arg(long)]
    pub nostd: bool,
    #[arg(long)]
    pub load: Vec<String>,
}

// TUI //

use terminal::{Action, Attribute, Clear, Color, Retrieved, Value};

pub struct Session {
    pub terminal: terminal::Terminal<std::io::Stdout>,
}

impl Session {
    pub fn new() -> Session {
        let terminal = terminal::stdout();

        let s = Session { terminal };

        s.init()
    }

    pub fn wait_for_event(
        &self,
        wait: Option<std::time::Duration>,
    ) -> Result<terminal::Retrieved, Box<dyn std::error::Error>> {
        Ok(self.terminal.get(terminal::Value::Event(wait))?)
    }

    pub fn get_cursor_position(&self) -> Result<terminal::Retrieved, Box<dyn std::error::Error>> {
        Ok(self.terminal.get(terminal::Value::CursorPosition)?)
    }

    fn init(mut self) -> Session {
        self.reset_style().unwrap();

        self.terminal
            .act(Action::ClearTerminal(Clear::CurrentLine))
            .unwrap();

        self.terminal.act(Action::EnableRawMode).unwrap();

        self
    }

    pub fn reset_style(&mut self) -> Result<(), terminal::error::ErrorKind> {
        self.terminal
            .act(Action::SetForegroundColor(Color::Reset))?;

        self.terminal
            .act(Action::SetBackgroundColor(Color::Reset))?;

        self.terminal.act(Action::SetAttribute(Attribute::Reset))?;

        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.reset_style().unwrap();

        self.terminal.act(Action::DisableRawMode).unwrap();

        if let Retrieved::TerminalSize(_w, h) = self.terminal.get(Value::TerminalSize).unwrap() {
            //self.terminal.act(Action::ScrollUp(h)).unwrap();
            for _i in 0..h {
                print!("\r\n");
            }
        }

        //self.terminal.act(Action::ClearTerminal(Clear::All)).unwrap();

        // actions that reset stuff
        /*
            self.reset_style()
            ClearTerminal(Clear::All)
            ShowCursor,
            DisableRawMode,
            LeaveAlternateScreen,
            DisableMouseCapture,
        */
    }
}

#[derive(Debug, Default)]
pub struct CommandHistory {
    index: usize,
    front_buffer: String,
    back_buffer: Option<String>,
    ordered_commands: Vec<String>,
    command_set: Vec<String>,
}

impl CommandHistory {
    pub fn new() -> CommandHistory {
        CommandHistory::default()
    }

    pub fn push(&mut self, cmd: String) {
        let ordered_index = self.ordered_commands.iter().position(|s| s == &cmd);
        if let Some(index) = ordered_index {
            // remove old instance so there's only one at a time
            self.ordered_commands.remove(index);
        } else {
            // add to the set, which could be in any order
            self.command_set.push(cmd.clone());
        }
        self.ordered_commands.push(cmd);
    }

    fn _remove(&mut self, _cmd: &str) {
        todo!()
    }

    pub fn move_up(&mut self, front_buffer: &mut String) {
        let len = self.ordered_commands.len();
        if len > 0 {
            match &self.back_buffer {
                None => {
                    self.index = len - 1;
                    self.back_buffer = Some(front_buffer.clone());
                    *front_buffer = self.get(self.index).clone();
                }
                Some(_) => {
                    if self.index > 0 {
                        self.index -= 1;

                        *front_buffer = self.get(self.index).clone();
                    }
                }
            }
        }
    }

    pub fn move_down(&mut self, front_buffer: &mut String) {
        if self.back_buffer.is_none() {
            return;
        }
        let len = self.ordered_commands.len();
        if len > 0 {
            if self.index < len - 1 {
                self.index += 1;

                *front_buffer = self.get(self.index).clone();
            } else {
                let s = self.back_buffer.take().unwrap();
                //self.back_buffer = None;

                *front_buffer = s.clone();
            }
        }
    }

    fn get(&mut self, index: usize) -> &String {
        &self.ordered_commands[index]
    }

    pub fn reset_buffers(&mut self, front_buffer: &mut String) {
        front_buffer.clear();
        if let Some(s) = self.back_buffer.take() {
            *front_buffer = s;
        }
    }
}

/*
#[derive(Parser)]
#[command(name = "lxsp")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli1 {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}


use crossterm::terminal;
use std::io;
use std::io::Read;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}
/* */


fn main() {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {
        /* add the following */
        let character = buf[0] as char;
        if character.is_control() {
            println!("{}\r", character as u8)
        } else {
            println!("{}\r", character)
        }
        /* end */
    }
}


*/
