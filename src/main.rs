use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread,
};

use reqwest::blocking::Client;
use uuid::Uuid;

mod cli;

use cli::Cli;

fn main() {
    let args = Cli::get_args();

    let server = args.ntfy_server.unwrap();
    let topic = args.topic.unwrap_or(Uuid::new_v4().to_string());
    let request_url = format!("{server}/{topic}");

    let err_notification = {
        let request_url = request_url.clone();
        move |msg: String| {
            Client::new()
                .post(&request_url)
                .body(format!("New error: {msg}"))
                .send()
        }
    };
    let success_notification = {
        let request_url = request_url.clone();
        move |msg: String| {
            Client::new()
                .post(&request_url)
                .body(format!("Command successful! Last message: {msg}"))
                .send()
        }
    };
    let fail_notification = {
        let request_url = request_url.clone();
        move |msg: String| {
            Client::new()
                .post(&request_url)
                .body(format!("Command failed! Last error: {msg}"))
                .send()
        }
    };

    let command = shell_words::split(&args.command).expect("problem splitting command");

    println!(
        "Running `{:?}`...\nClick this url to subscribe to alerts: {}\n---------------",
        &args.command, &request_url
    );

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
            err_notification(line.clone()).expect("Err sending notification");
            eprintln!("{}", line);
        }
        last_stderr
    });

    let last_stdout = stdout_handle.join().expect("Thread panicked");
    let last_stderr = stderr_handle.join().expect("Thread panicked");

    match if child.wait().expect("Failed to wait on child").success() {
        println!("[pntfy] Command completed successfully.");
        success_notification(last_stdout)
    } else {
        eprintln!("[pntfy] Command failed!");
        fail_notification(last_stderr)
    } {
        Ok(_res) => println!("[pntfy] Sent notification."),
        Err(_err) => eprintln!("[pntfy] Failed to send notification!"),
    }
}
