use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// A tool for notifying when a command fails or succeeds
pub struct Cli {
    pub command: String,
    #[arg(short, long)]
    // TODO: Unimplemented
    // /// Suppress displaying ASCII QR code
    // no_qr: bool,
    #[arg(short, long)]
    /// Use a custom notification topic [default: (generated UUID)]
    pub topic: Option<String>,
    /// The ntfy server url
    #[arg(long, default_value = "http://ntfy.sh")]
    pub ntfy_server: Option<String>,
}

impl Cli {
    pub fn get_args() -> Self {
        Self::parse()
    }
}
