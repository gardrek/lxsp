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

    fn init(mut self) -> Session {
        self.reset_style().unwrap();

        self.terminal
            .act(Action::ClearTerminal(Clear::CurrentLine))
            .unwrap();

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
        self.terminal
            .act(Action::ClearTerminal(Clear::CurrentLine))
            .unwrap();

        self.reset_style().unwrap();

        // other actions that reset stuff
        /*
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
*/