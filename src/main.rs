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
}

fn main() {
    let args = Args::parse();

    for i in 1..args.times + 1 {
        println!("Run {}", i);
        println!("--------------");
        // run command and only show output if it fails
        let command = args.command.replace("{}", i.to_string().as_str());
        let r = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output();

        if let Ok(r) = r {
            if !r.status.success() || args.show_output {
                println!("{}", String::from_utf8(r.stdout).unwrap());
            }
            if !r.status.success() && !args.ignore_failures {
                println!("{}", String::from_utf8(r.stderr).unwrap());
                std::process::exit(1);
            }
        }
    }
}
