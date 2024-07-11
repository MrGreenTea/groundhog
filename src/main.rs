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
        println!("Run {}", i);
        println!("--------------");
        // run command and only show output if it fails
        let command = args.command.replace("{}", i.to_string().as_str());
        let r = std::process::Command::new(&args.shell)
            .arg("-c")
            .arg(command)
            .output();

        if !args.only_count_successes || r.is_ok() {
            i += 1;
        }

        if let Ok(r) = r {
            if !r.status.success() || args.show_output {
                println!("{}", String::from_utf8(r.stdout).unwrap());
            }
            if !r.status.success() && !args.ignore_failures {
                // exit with error if we don't ignore failures
                // and print error message to stderr
                eprintln!("Command failed with {}", r.status);
                std::process::exit(1);
            }
        }
    }
}
