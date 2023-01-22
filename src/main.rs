mod cli;
mod env;
mod eval;
mod parse;
mod scan;
mod tests;
mod value;

//~ use std::io::Read;
//~ use std::io::Write;
use std::error::Error;

use cli::CommandHistory;
use cli::Session;
use terminal::Action;
use terminal::Color;
use terminal::KeyModifiers;
use terminal::Retrieved;

use clap::Parser;
use once_cell::sync::OnceCell;

use env::LispEnv as LispEnv;
use value::Value as LispValue;

pub static BASE_ENV: OnceCell<LispEnv<'_>> = OnceCell::new();

pub static STD_ENV: OnceCell<LispEnv<'_>> = OnceCell::new();

pub fn parse_eval(source: &str, env: &LispEnv) -> Result<LispValue, Box<dyn Error>> {
    Ok(env.eval(&parse_string(&source)?)?)
}

pub fn parse_eval_and_macro_pass(source: &str, env: &LispEnv) -> Result<LispValue, Box<dyn Error>> {
    let result = parse_string(source)?;
    let passed = env.macro_eval(&result)?;
    Ok(env.eval(&passed)?)
    //Ok(passed)
}

pub fn parse_string(source: &str) -> Result<LispValue, Box<dyn Error>> {
    let scanner = scan::Scanner::new(source);

    let tokens = scanner.collect::<Vec<_>>();

    let (parsed_exp, _rest) = parse::parse(&tokens)?;

    Ok(parsed_exp)
}

/*
fn input_line() -> String {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    line
}
*/

fn add_lib<'e, 's>(base_env: &'e LispEnv, src: &'s str) -> Result<LispEnv<'e>, Box<dyn Error>> {
    let pairs = parse_string(&src)?;

    Ok(base_env.new_inner_from_pairs(&pairs)?)
}

fn print_prompt(session: &cli::Session) -> Result<(), Box<dyn Error>> {
    session
        .terminal
        .act(Action::SetForegroundColor(Color::Blue))?;

    print!("λ ");

    session
        .terminal
        .act(Action::SetForegroundColor(Color::Reset))?;

    Ok(())
}

fn print_error(session: &cli::Session, error: &str) -> Result<(), Box<dyn Error>> {
    session
        .terminal
        .act(Action::SetForegroundColor(Color::Red))?;

    print!("{}", error);

    session
        .terminal
        .act(Action::SetForegroundColor(Color::Reset))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::ArgStruct::parse();

    if args.use_old_repl {
        return old_main();
    }

    BASE_ENV.set(LispEnv::default()).unwrap();

    let base_env = BASE_ENV.get().unwrap();

    let env = if args.nostd {
        base_env
    } else {
        let src = std::fs::read_to_string("lisb/std.l")?;
        //~ let src = std::fs::read_to_string("lisb/lam.l")?;

        let std_inner = add_lib(&base_env, &src)?;

        STD_ENV.set(std_inner).unwrap();

        STD_ENV.get().unwrap()
    };

    let built_ins = base_env.sorted_list();

    print!("Built-ins: ");
    for s in &built_ins {
        print!("{} ", s);
    }
    println!();

    let std_bindings = env.sorted_list();

    print!("Standard Library: ");
    for s in &std_bindings {
        print!("{} ", s);
    }
    println!();

    println!();
    println!("---");
    println!();

    let session = Session::new();

    ////////////////////////////////////////////////////////////////
    // Main Loop                                                  //
    ////////////////////////////////////////////////////////////////

    let mut rl = rustyline::Editor::<()>::new();

    rl.history_mut().set_max_len(500);

    'main: loop {
        //print_prompt(&session)?;

        session
            .terminal
            .act(Action::SetForegroundColor(Color::Blue))?;

        //print!("{}", error);
        let readline = rl.readline("λ \x1b[0m");

        session
            .terminal
            .act(Action::SetForegroundColor(Color::Reset))?;

        match readline {
            Ok(raw_line) => {
                let line = raw_line.trim();

                if line.is_empty() {
                    continue;
                    //} else if line == "exit" {
                    //    break 'main Ok(());
                }

                let result = parse_eval_and_macro_pass(&line, env);
                match result {
                    Ok(res) => {
                        print!("\r => {}\r\n", res);
                        if res == LispValue::Symbol("exit".to_string()) {
                            break 'main Ok(());
                        }
                    }
                    Err(e) => print_error(&session, &format!("\r[ERROR] {}\r\n", e))?,
                }

                rl.history_mut().add(line);
            }
            Err(_) => break 'main Ok(()),
        }
    }
}

fn old_main() -> Result<(), Box<dyn Error>> {
    let args = cli::ArgStruct::parse();

    BASE_ENV.set(LispEnv::default()).unwrap();

    let base_env = BASE_ENV.get().unwrap();

    let env = if args.nostd {
        base_env
    } else {
        let src = std::fs::read_to_string("lisb/std.l")?;
        //~ let src = std::fs::read_to_string("lisb/lam.l")?;

        let std_inner = add_lib(&base_env, &src)?;

        STD_ENV.set(std_inner).unwrap();

        STD_ENV.get().unwrap()
    };

    let built_ins = base_env.sorted_list();

    print!("Built-ins: ");
    for s in &built_ins {
        print!("{} ", s);
    }
    println!();

    let std_bindings = env.sorted_list();

    print!("Standard Library: ");
    for s in &std_bindings {
        print!("{} ", s);
    }
    println!();

    println!();
    println!("---");
    println!();

    let session = Session::new();

    let mut command_buffer = String::new();
    let mut cursor_index = 0;
    let mut cursor_position = (0u16, 0u16);
    let mut command_history = CommandHistory::new();

    print_prompt(&session)?;

    // this won't work because it mutably borrows stuff for the whole loop i guess
    // maybe some sort of internal mutability is in order?
    /*let reset_prompt = || -> Result<(), Box<dyn Error>> {
        command_buffer.clear();
        print_prompt(&session)?;
        Ok(())
    };*/

    'main: loop {
        //~ let result = session.wait_for_event(Some(std::time::Duration::from_millis(5)));
        let result = session.wait_for_event(None);
        let ok = if let Ok(ok) = result {
            ok
        } else {
            result?;
            break 'main Ok(());
        };
        match ok {
            Retrieved::Event(Some(event)) => {
                match event {
                    terminal::Event::Key(terminal::KeyEvent { code, modifiers }) => {
                        use terminal::KeyCode::*;
                        match code {
                            Esc => break 'main Ok(()),
                            Backspace => {
                                if command_buffer.len() > cursor_index && cursor_index > 0 {
                                    let ch = command_buffer.remove(cursor_index - 1);

                                    if let Ok(Retrieved::CursorPosition(x, y)) =
                                        session.get_cursor_position()
                                    {
                                        session.terminal.act(Action::ClearTerminal(
                                            terminal::Clear::UntilNewLine,
                                        ))?;
                                        session.terminal.act(Action::MoveCursorTo(x, y))?;
                                        print!("{}", &command_buffer[(cursor_index - 1)..]);
                                        cursor_index -= ch.len_utf8();
                                        session.terminal.act(Action::MoveCursorTo(x - 1, y))?;
                                    } else {
                                        panic!()
                                    }
                                }
                            }
                            Enter => {
                                //print!("{}\r\n", command_buffer);
                                print!("\r\n");

                                let line = command_buffer.clone();

                                let line = line.trim();

                                if line.trim().is_empty() {
                                    command_buffer.clear();
                                    print_prompt(&session)?;
                                    continue;
                                } else if line.trim() == "exit" {
                                    break 'main Ok(());
                                }

                                let result = parse_eval(&line, env);
                                match result {
                                    Ok(res) => {
                                        print!(" => {}\r\n", res);
                                        if res == LispValue::Symbol("exit".to_string()) {
                                            break 'main Ok(());
                                        }
                                    }
                                    Err(e) => print_error(&session, &format!("[ERROR] {}\r\n", e))?,
                                }

                                command_history.push(line.to_string());

                                //print!("{:?}\r\n", command_history);

                                command_history.reset_buffers(&mut command_buffer);

                                cursor_index = 0;
                                command_buffer.clear();
                                print_prompt(&session)?;
                            }
                            Up | Down => {
                                print!("\r"); // carriage retrun
                                session
                                    .terminal
                                    .act(Action::ClearTerminal(terminal::Clear::UntilNewLine))?;
                                match code {
                                    Up => command_history.move_up(&mut command_buffer),
                                    Down => command_history.move_down(&mut command_buffer),
                                    _ => unreachable!(),
                                }
                                print_prompt(&session)?;
                                print!("{}", command_buffer);
                                cursor_index = command_buffer.len(); // just past the end
                            }
                            Left | Right => {
                                if let Ok(Retrieved::CursorPosition(x, y)) =
                                    session.get_cursor_position()
                                {
                                    match code {
                                        Left => {
                                            if cursor_index > 0 {
                                                cursor_index -= 1;
                                                session
                                                    .terminal
                                                    .act(Action::MoveCursorTo(x - 1, y))?;
                                            }
                                        }
                                        Right => {
                                            if cursor_index < command_buffer.len() {
                                                cursor_index += 1;
                                                session
                                                    .terminal
                                                    .act(Action::MoveCursorTo(x + 1, y))?;
                                            }
                                        }
                                        _ => unreachable!(),
                                    };
                                };
                            }
                            Char(ch) => {
                                if modifiers.is_empty() {
                                    //if cursor_index < command_buffer.len() {
                                    command_buffer.insert(cursor_index, ch);
                                    if let Ok(Retrieved::CursorPosition(x, y)) =
                                        session.get_cursor_position()
                                    {
                                        session.terminal.act(Action::MoveCursorTo(x, y))?;
                                        print!("{}", &command_buffer[cursor_index..]);
                                        cursor_index += ch.len_utf8();
                                        session.terminal.act(Action::MoveCursorTo(x + 1, y))?;
                                    }
                                    /*} else {
                                        cursor_index += ch.len_utf8();
                                        command_buffer.push(ch);
                                        print!("{}", ch);
                                    }*/
                                } else if modifiers.contains(KeyModifiers::CONTROL) {
                                    let mut modifiers = modifiers.clone();
                                    modifiers.remove(KeyModifiers::CONTROL);
                                    if modifiers.is_empty() {
                                        match ch {
                                            'c' | 'd' => break 'main Ok(()),
                                            _ => (),
                                        }
                                    } else {
                                        // control+shift and control+alt an such can go here
                                    }
                                } else {
                                }
                            }
                            _ => {
                                session
                                    .terminal
                                    .act(Action::ClearTerminal(terminal::Clear::CurrentLine))?;
                                session
                                    .terminal
                                    .act(Action::MoveCursorTo(0, cursor_position.1))?;
                                print!("{:?}\r\n", event);
                                print_prompt(&session)?;
                                print!("{}", command_buffer);
                            }
                        }
                    }
                    _ => {
                        //todo!()
                    }
                }
            }
            Retrieved::Event(None) => (), // timed out, so we don't do anythihng this "frame"
            _ => (),
        };

        // NOTE: this silently just leaves the cursor position as old values
        // if the read fails
        if let Ok(Retrieved::CursorPosition(x, y)) = session.get_cursor_position() {
            cursor_position = (x, y)
        };
        //~ session.terminal.act(Action::MoveCursorTo(0, 0))?;
        //~ session.terminal.act(Action::ClearTerminal(terminal::Clear::CurrentLine))?;
        //~ session.terminal.act(Action::MoveCursorTo(0, cursor_position.1))?;
        //print!("{}", command_buffer);
    }

    /*
    'main: loop {
        session
            .terminal
            .act(Action::SetForegroundColor(Color::Blue))?;
        print!("λ ");
        session
            .terminal
            .act(Action::SetForegroundColor(Color::Reset))?;

        std::io::stdout().flush()?;
        let line = input_line();

        if line.trim().is_empty() {
            continue;
        } else if line.trim() == "exit" {
            break Ok(());
        }

        let result = parse_eval(&line, env);
        match result {
            Ok(res) => {
                println!("=> {}", res);
                if res == LispValue::Symbol("exit".to_string()) {
                    break 'main Ok(());
                }
            }
            Err(e) => println!("[ERR] {}", e),
        }
    }
    */
}

/*
#[rustfmt::skip]
const ALPHABET: [&str; 64] = [
     " ", "0", ")", // $1
          "1", "'", // $2
          "2",       "a", "g",
               ",", // $3
          "3",       "b", "h",
                     "f", "i", "j",
    "_9",      "(", // $4
          "4",       "c", "k", "d", "l", "m",
    "_8",            "e", "n", "o",
    "_7",            "p",
    "_6",
    "_5",
    "_@",      "@", // $5
           "5",
           "6",      "q",
           "7",      "r", "s",
    "_e",  "8",      "t", "u",
    "_d",            "v",
    "_c",
    "_4",
    "_(",  "9",      "w", "x",
    "_f",            "y",
    "_b",
    "_3",
    "_,",            "z",
    "_a",
    "_2",
    "_'",
    "_1",
    "_)",
    "_0",
   "DEL",
];
*/
//const fn calcuate_reverse_alphabet() -> [&'static str; 64] {}
