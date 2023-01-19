use terminal::{error, Action, Attribute, Clear, Color, Retrieved, Value};

pub struct Session {
    pub terminal: terminal::Terminal<std::io::Stdout>,
}

impl Session {
    pub fn new() -> Session {
        let terminal = terminal::stdout();

        let s = Session { terminal };

        s.init()
    }

    pub fn wait_for_event(&self) -> Result<terminal::Retrieved, Box<dyn std::error::Error>> {
        Ok(self.terminal.get(terminal::Value::Event(None))?)
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

        self.terminal
            .act(Action::ClearTerminal(Clear::All))
            .unwrap();

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

pub fn _main() -> error::Result<()> {
    let terminal = terminal::stdout();

    // perform an single action.
    terminal.act(Action::ClearTerminal(Clear::All))?;

    // batch multiple actions.
    for i in 0..100 {
        terminal.batch(Action::MoveCursorTo(0, i))?;
    }

    // execute batch.
    terminal.flush_batch()?;

    // get an terminal value.
    if let Retrieved::TerminalSize(x, y) = terminal.get(Value::TerminalSize)? {
        println!("x: {}, y: {}", x, y);
    }

    Ok(())
}

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
