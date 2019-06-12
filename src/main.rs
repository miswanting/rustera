use std::{env, io, io::Write};
mod engine;
const VERSION: &'static str = "0.1.0-190612";
const AKA: &'static str = "Dead Start";

/// 程序入口
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // 直接启动
        // 启动引擎
        help();
        engine::engine::test();
    } else {
        // 参数启动
        // println!("{:?}", args);
        if args[1] == "--V" || args[1] == "--version" {
            version()
        } else if args[1] == "itp" {
            itp()
        } else if args[1] == "run" {
            run()
        } else {
            help()
        }
    }
}
fn cover() {
    println!("RustEra v{} aka \"{}\"", VERSION, AKA);
    println!("Type \"help\", \"copyright\", \"credits\" or \"license\" for more information.");
}
fn version() {
    println!("RustEra v{} aka \"{}\"", VERSION, AKA);
}
fn itp() {
    cover();
    loop {
        let input = get_input_line();
        // print!("{}", input);
        let input = input.trim();
        // print!("{}", input == "help");
        io::stdout().flush().unwrap();
        // match
        if input == "help" {
            itp_help()
        } else if input == "exit" || input == "quit" {
            print!("EXIT {}", 0);
            io::stdout().flush().unwrap();
            break;
        }
        // input.split(' ');
    }
}
fn run() {}

fn get_input_line() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // print!("{}", input);
    // io::stdout().flush().unwrap();
    return input;
}
fn help() {
    println!(
        "RustEra Game Engine

USAGE:
    rustera [COMMANDS] [ARGUMENTS]
    rustera [OPTIONS] [ARGUMENTS]

OPTIONS:
    -V, --version  Print version info and exit
    -h, --help     Prints help information

COMMANDS:
    itp            Run the Interpreter
    debug          Run Game in DEBUG mode.
    run            Run Game"
    );
}
fn itp_help() {
    println!("RustEra Game Script Interpreter");
}
