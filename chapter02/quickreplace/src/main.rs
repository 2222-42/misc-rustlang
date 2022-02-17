use std::fs;

use regex::Regex;
use text_colorizer::Colorize;

fn print_usage() {
    eprintln!(
        "{} - change occurrences of aonestring into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickrepalce <target> <replacement> <INPUT> <OUTPUT>");
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, repalcement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, repalcement).to_string())
}

fn main() {
    let args = parse_args();
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(err) => {
            eprintln!(
                "{} failed to read from '{}': {:?}",
                "Error".red().bold(),
                args.filename,
                err
            );
            std::process::exit(1);
        }
    };

    let replace_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replace_data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write to '{}': {:?}",
                "Error".red().bold(),
                args.output,
                e
            );
            std::process::exit(1);
        }
    };
    println!("{:?}", args);
}
