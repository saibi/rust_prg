use clap::Parser;

/// Simple command line parser example
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path to monitor
    #[arg(short, long)]
    path: std::path::PathBuf,

    /// Monitoring interval in seconds
    #[arg(short, long, default_value_t = 1)]
    interval: u32,

    /// Monitoring duration in seconds
    #[arg(short, long, default_value_t = 60)]
    duration: u32,
}

fn main() {
    let args = Args::parse();

    println!(
        "Monitoring {} for {} seconds",
        args.path.display(),
        args.duration
    );

    monitor_path(&args);
}

fn monitor_path(args: &Args) {
    for _ in 0..args.duration / args.interval {
        std::thread::sleep(std::time::Duration::from_secs(args.interval as u64));
        if args.path.exists() {
            println!("{} exists", args.path.display());
            handler(&args.path);
        } else {
            println!("{} does not exist", args.path.display());
        }
    }
}

fn handler(path: &std::path::PathBuf) {
    let contents = std::fs::read_to_string(path).unwrap();
    println!("{}", contents);
    std::fs::remove_file(path).unwrap();

    // get first line
    let first_line = contents.lines().next().unwrap();
    println!("First line: {}", first_line);
    // first line format
    // # command hash
    let mut iter = first_line.split_whitespace();
    iter.next(); // ignore # symbol
    let cmd = iter.next().unwrap();
    let hash = iter.next().unwrap();

    println!("Command: {}", cmd);
    println!("Hash: {}", hash);

    let remain_contents = contents.lines().skip(1).collect::<Vec<_>>().join("\n");
    println!("Remain contents: {}", remain_contents);
}
