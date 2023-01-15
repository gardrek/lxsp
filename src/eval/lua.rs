use crate::eval::eval_err;
use crate::eval::EvalError;
use crate::eval::LispEnv;
use crate::lisp::LispValue;

use std::process::Command;
use std::sync::Arc;

pub fn run(mut child: std::process::Child) -> Result<LispValue, EvalError> {
    let error_status = child.wait().expect("failed to wait on child");

    let code = error_status.code();

    let error_code = match code {
        Some(i) => LispValue::Integer(i.into()),
        None => LispValue::nil(),
    };

    // list!(error_code, "unimplemented")
    Ok(LispValue::List(Arc::new([
        error_code.into(),
        "unimplemented".into(),
    ])))
}

pub fn _run_lua_source(src: &str) -> Result<LispValue, EvalError> {
    let child = Command::new("lua")
        .arg("-e")
        .arg(&src)
        .spawn()
        .expect("Lua failed to start");

    run(child)
}

pub fn run_lua_file(filename: &str) -> Result<LispValue, EvalError> {
    let child = Command::new("lua")
        .arg(&filename)
        .spawn()
        .expect("Lua failed to start");

    run(child)
}

pub fn run_lua_file_from_lisp_args(
    args: &[LispValue],
    env: &LispEnv,
) -> Result<LispValue, EvalError> {
    if args.len() != 1 {
        return Err(eval_err("[lua] Wrong number of arguments"));
    }

    let sym = env.eval(&args[0])?;

    let lib_name = sym
        .get_symbol()
        .ok_or(eval_err("argument to lua not a symbol"))?;

    let filename = get_lua_lib_filename(lib_name);

    let result = run_lua_file(&filename)?;

    Ok(result)
}

fn get_lua_lib_filename(lib_name: &str) -> String {
    format!("src/{}.lua", lib_name)
}
