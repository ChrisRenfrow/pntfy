use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
/// A tool for notifying when a command fails or succeeds
pub struct Cli {
    /// The command to monitor
    ///
    /// Complex commands featuring pipes and redirections should
    /// be run with `sh -c` for intended results.
    pub command: String,
    #[arg(short, long)]
    // TODO: Unimplemented
    // /// Suppress displaying ASCII QR code
    // no_qr: bool,
    #[arg(short, long)]
    /// Use a custom notification topic
    ///
    /// [default: (generated UUIDv4)]
    pub topic: Option<String>,
    /// The ntfy server url
    #[arg(long, default_value = "http://ntfy.sh")]
    pub ntfy_server: Option<String>,
    /// Request that the ntfy server disables caching messages
    ///
    /// Caching is used to facilitate redelivery to disconnected
    /// subscribed clients, can be useful in poor network scenarios.
    ///
    /// Documentation: https://docs.ntfy.sh/publish/#message-caching
    #[arg(long, short = 'c')]
    pub no_cache: bool,
    /// Request that the ntfy server disables forwarding messages
    /// to Firebase
    ///
    /// This is to improve Android notification delivery speed and
    /// battery optimization.
    ///
    /// Documentation: https://docs.ntfy.sh/publish/#disable-firebase
    #[arg(long, short = 'f')]
    pub no_firebase: bool,
}

impl Cli {
    pub fn get_args() -> Self {
        Self::parse()
    }
}
