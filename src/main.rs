use std::process::Command;
use std::time::Instant;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use colored::*;

struct Config {
    posix_format: bool,
    output_file: Option<String>,
    append: bool,
    program: String,
    args: Vec<String>,
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut config = Config {
        posix_format: false,
        output_file: None,
        append: false,
        program: String::new(),
        args: Vec::new(),
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-p" => config.posix_format = true,
            "-o" => {
                i += 1;
                if i < args.len() {
                    config.output_file = Some(args[i].clone());
                } else {
                    return Err("Missing filename after -o".to_string());
                }
            },
            "-a" => config.append = true,
            _ => {
                config.program = args[i].clone();
                config.args = args[i+1..].to_vec();
                break;
            }
        }
        i += 1;
    }

    if config.program.is_empty() {
        Err("No program specified".to_string())
    } else {
        Ok(config)
    }
}

fn format_output(duration: f64, posix: bool) -> String {
    if posix {
        format!("{}\n{}\n{}\n",
            format!("{} {:.2}", "real".bold().truecolor(161, 178, 255), duration),
            format!("{} 0.00", "user".bold().truecolor(209, 161, 255)),
            format!("{} 0.00", "sys".bold().truecolor(224, 255, 161)))
    } else {
        format!("\n{}\n{}\n{}\n",
            format!("{}\t0m{:.3}s", "real".bold().truecolor(161, 178, 255), duration),
            format!("{}\t0m0.000s", "user".bold().truecolor(209, 161, 255)),
            format!("{}\t0m0.000s", "sys".bold().truecolor(224, 255, 161)))
    }
}

fn main() -> std::io::Result<()> {
    let config = match parse_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", "Error:".bright_red().bold());
            eprintln!("  {}", e.red());
            eprintln!("{}", "Usage:".bright_yellow().bold());
            eprintln!("  {} [-pa] [-o FILE] PROG ARGS", "time".cyan());
            eprintln!("  Run PROG, display resource usage when it exits");
            eprintln!("    {}  POSIX output format", "-p".green());
            eprintln!("    {}  Write result to FILE", "-o FILE".green());
            eprintln!("    {}  Append (else overwrite)", "-a".green());
            std::process::exit(1);
        }
    };

    println!("{} {}", "Running:".bright_yellow().bold(), config.program.cyan());

    let start = Instant::now();

    let status = Command::new(&config.program)
        .args(&config.args)
        .status()?;

    let duration = start.elapsed().as_secs_f64();

    let output = format_output(duration, config.posix_format);

    if let Some(file_path) = config.output_file {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(config.append)
            .open(&file_path)?;
        write!(file, "{}", output)?;
        println!("{} {}", "Results written to:".bright_green().bold(), file_path.bright_cyan());
    } else {
        println!("{}", output);
    }

    println!("{} {}", "Command exited with status:".bright_magenta().bold(), status.to_string().white().bold());

    Ok(())
}