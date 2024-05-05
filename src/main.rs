use std::{
    io::{BufRead, BufReader},
    process::{exit, Command, Stdio},
    thread,
};

use uuid::Uuid;

mod cli;
mod notifier;

use cli::Cli;
use notifier::Notifier;

fn main() {
    let args = Cli::get_args();

    let server = args.clone().ntfy_server.unwrap();
    let topic = args.clone().topic.unwrap_or(Uuid::new_v4().to_string());
    let request_url = format!("{server}/{topic}");
    let command = shell_words::split(&args.command).expect("problem splitting command");

    let notifier = match Notifier::new(&request_url, Some(args.clone().into())) {
        Ok(notifier) => notifier,
        Err(err) => {
            eprintln!("Error parsing URL: {}", err);
            exit(1);
        }
    };

    println!(
        "Running {}...\nClick this url to subscribe to alerts: {}\n---------------",
        &args.command, &request_url
    );

    let mut child = match Command::new(&command[0])
        .args(&command[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            eprintln!("[pntfy] Couldn't launch command! {}\n\tExiting...", err);
            exit(1);
        }
    };

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

    let stderr_handle = thread::spawn({
        let notifier = notifier.clone();
        move || {
            for line in stderr_reader.lines().map_while(Result::ok) {
                last_stderr = line.clone();
                if let Err(err) = notifier.notify(format!("New error: {last_stderr}").as_str()) {
                    eprintln!("[pntfy] {}", err);
                }
                eprintln!("{}", line);
            }
            last_stderr
        }
    });

    let last_stdout = stdout_handle.join().expect("Thread panicked");
    let last_stderr = stderr_handle.join().expect("Thread panicked");

    match if child.wait().expect("Failed to wait on child").success() {
        println!("[pntfy] Command completed successfully.");
        notifier.notify(format!("Command successful! Last message: {last_stdout}").as_str())
    } else {
        eprintln!("[pntfy] Command failed!");
        notifier.notify(format!("Command failed! Last error: {last_stderr}").as_str())
    } {
        Ok(_res) => println!("[pntfy] Sent notification."),
        Err(err) => eprintln!("[pntfy] {err}"),
    }
}
