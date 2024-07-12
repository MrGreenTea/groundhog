use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use clap::Parser;

/// Simple program that repeats a command
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// command to run
    #[arg(short, long)]
    command: String,

    /// Number of times to repeat
    #[arg(short, long, default_value_t = 1)]
    times: u8,

    /// Ignore failures and continue
    #[arg(short, long, default_value_t = false)]
    ignore_failures: bool,

    /// Show output of each run
    #[arg(short, long, default_value_t = false)]
    show_output: bool,

    /// Only count succesfull runs
    #[arg(long, default_value_t = false)]
    only_count_successes: bool,

    /// Shell to use
    #[arg(long, default_value_t = String::from("sh"))]
    shell: String,
}

fn main() {
    let args = Args::parse();

    let mut i = 1;
    while i <= args.times {
        println!("\nRun {i}");
        println!("--------------");
        // run command and only show output if it fails
        let command = args.command.replace("{}", i.to_string().as_str());
        let command_run = Command::new(&args.shell)
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match command_run {
            Ok(mut cmd) => {
                let stdout_lines = {
                    let stdout = cmd.stdout.as_mut().expect("Failed to open stdout");
                    let stdout_reader = BufReader::new(stdout);
                    stdout_reader.lines()
                };
                let stderr_lines = {
                    let stderr = cmd.stderr.as_mut().expect("Failed to open stderr");
                    let stderr_reader = BufReader::new(stderr);
                    stderr_reader.lines()
                };

                for line in stdout_lines {
                    println!("{}", line.unwrap());
                }

                for line in stderr_lines {
                    eprintln!("{}", line.unwrap());
                }

                match cmd.wait() {
                    Ok(status) => {
                        if !status.success() && !args.ignore_failures {
                            // exit with error if we don't ignore failures
                            // and print error message to stderr
                            eprintln!("Command failed with {status}");
                            std::process::exit(1);
                        }
                        if status.success() || !args.only_count_successes {
                            i += 1;
                        }
                    }
                    Err(err) => {
                        eprintln!("Error running command: {err}");
                    }
                }
            }
            Err(err) => {
                eprintln!("Error running command: {err}");
            }
        }
    }
}
