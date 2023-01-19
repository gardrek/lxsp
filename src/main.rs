mod cli;
mod eval;
mod lisp;
mod parse;
mod scan;
mod tests;

use clap::Parser;
use once_cell::sync::OnceCell;

use eval::LispEnv;
use lisp::Value as LispValue;

pub static BASE_ENV: OnceCell<LispEnv<'_>> = OnceCell::new();

pub static STD_ENV: OnceCell<LispEnv<'_>> = OnceCell::new();

pub fn parse_eval(source: &str, env: &LispEnv) -> Result<LispValue, Box<dyn std::error::Error>> {
    Ok(env.eval(&parse_string(&source)?)?)
}

pub fn parse_string(source: &str) -> Result<LispValue, Box<dyn std::error::Error>> {
    let scanner = scan::Scanner::new(source);

    let tokens = scanner.collect::<Vec<_>>();

    let (parsed_exp, _rest) = parse::parse(&tokens)?;

    Ok(parsed_exp)
}

fn input_line() -> String {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    line
}

fn add_lib<'e, 's>(
    base_env: &'e LispEnv,
    src: &'s str,
) -> Result<LispEnv<'e>, Box<dyn std::error::Error>> {
    let pairs = parse_string(&src)?;

    Ok(base_env.new_inner_from_pairs(&pairs)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //~ use std::io::Read;
    //~ use std::io::Write;
    use terminal::{Action, Color};

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

    use cli::Session;
    use terminal::Retrieved;

    let session = Session::new();

    /*
    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1
        && buf != [b'q']
        && buf != [3]
    {
        let character = buf[0] as char;
        print!("len: {}\r\n", buf.len());
        if character.is_control() {
            print!("control {}\r\n", character as u8)
        } else {
            print!("char {}\r\n", character)
        }
    }
    */

    let mut command_buffer = String::new();
    let mut cursor_position = (0u16, 0u16);

    'main: loop {
        match session.wait_for_event()? {
            Retrieved::Event(Some(event)) => {
                match event {
                    terminal::Event::Key(terminal::KeyEvent { code, modifiers }) => {
                        use terminal::KeyCode::*;
                        let _modifiers = modifiers;
                        match code {
                            Esc => break 'main Ok(()),
                            Enter => {
                                print!("{}\r\n", command_buffer);
                                
                                let line = command_buffer.clone();
                        
                                if line.trim().is_empty() {
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
                                    Err(e) => print!("[ERROR] {}\r\n", e),
                                }
                                command_buffer.clear()
                            },
                            Char(c) => command_buffer.push(c),
                            _ => (),
                        }
                    }
                    _ => {
                        //todo!()
                    }
                }
                //print!("{:?}\r\n", event);
            }
            Retrieved::Event(None) => todo!(),
            _ => (),
        };

        if let Retrieved::CursorPosition(x, y) = session.get_cursor_position()? {
            cursor_position = (x, y)
        };
        session.terminal.act(terminal::Action::MoveCursorTo(0, 0))?;
        session.terminal.act(terminal::Action::ClearTerminal(terminal::Clear::CurrentLine))?;
        print!("{}", command_buffer);
        session.terminal.act(terminal::Action::MoveCursorTo(0, cursor_position.1))?;
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
