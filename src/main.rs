use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread,
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// A tool for notifying when a command fails or succeeds
struct Cli {
    command: String,
    #[arg(short, long)]
    /// Suppress displaying ASCII QR code
    no_qr: bool,
    #[arg(short, long)]
    /// Use a custom notification topic [default: (generated UUID)]
    topic: Option<String>,
    /// The ntfy server url
    #[arg(long, default_value = "https://ntfy.sh")]
    ntfy_server: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let command = shell_words::split(&args.command).expect("problem splitting command");

    println!("{:?}", command);

    let mut child = Command::new(&command[0])
        .args(&command[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stderr = child.stderr.take().expect("Failed to open stderr");

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let mut last_stdout = String::new();
    let mut last_stderr = String::new();

    let stdout_handle = thread::spawn(move || {
        for line in stdout_reader.lines().map_while(Result::ok) {
            last_stdout = line.clone();
            println!("{}", line);
        }
        last_stdout
    });

    let stderr_handle = thread::spawn(move || {
        for line in stderr_reader.lines().map_while(Result::ok) {
            last_stderr = line.clone();
            eprintln!("{}", line);
        }
        last_stderr
    });

    let status = child.wait().expect("Failed to wait on child");

    let last_stdout = stdout_handle.join().expect("Thread panicked");
    let last_stderr = stderr_handle.join().expect("Thread panicked");

    println!("Child process exited {}", status);
    println!("last output line:\n{}", last_stdout);
    println!("last error line received:\n{}", last_stderr);
}
