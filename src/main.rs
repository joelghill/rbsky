use clap::Parser;
use rbsky::{client::BlueSkyClient, xrpc::com::atproto::session::SessionManager};
use reqwest::blocking::Client;
use std::env;

#[derive(Parser, Debug)]
struct AuthArgs {
    /// User handle. Username, email, did
    #[arg(short, long, required = true)]
    handle: String,
}

/// CLI for managing BlueSky accounts
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Command to execute against account
    #[command(subcommand)]
    command: Commands,

    /// BlueSky server host
    #[arg(short, long, default_value_t = String::from("https://bsky.social"))]
    server: String,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    /// Account-management in ATP services.
    Account {
        #[command(subcommand)]
        option: AccountOptions,
    },
    /// Manage handles (aka usernames) in ATP.
    Handle,
    /// Session management in ATP.
    Session,
}

/// Account Commands
#[derive(Debug, clap::Subcommand)]
enum AccountOptions {
    /// Get information about an account.
    Get {
        /// User handle. Username, email, did
        #[arg(short = 'H', long, required = false)]
        handle: String,
    },
    /// Create a bsky account
    Create,
    /// Create a bsky invite code
    InviteCode,
}

fn main() {
    let http = Client::new();

    let env_handle = match env::var("ATPROTO_HANDLE") {
        Ok(val) => val,
        Err(_) => String::from(""),
    };

    let env_password = match env::var("ATPROTO_PASSWORD") {
        Ok(val) => val,
        Err(_) => String::from(""),
    };

    let args = Args::parse();
    let bsky_client: BlueSkyClient = BlueSkyClient::new(&args.server, &http);

    let session_result = match args.command {
        Commands::Account { option } => match option {
            AccountOptions::Get { handle } => {
                if !handle.is_empty() {
                    bsky_client
                        .com
                        .atproto
                        .session
                        .create(&handle, &env_password)
                } else {
                    bsky_client
                        .com
                        .atproto
                        .session
                        .create(&env_handle, &env_password)
                }
            }
            AccountOptions::Create => todo!(),
            AccountOptions::InviteCode => todo!(),
        },
        Commands::Handle => todo!(),
        Commands::Session => todo!(),
    };

    match session_result {
        Ok(s) => println!("{:#?}", s),
        Err(e) => match e {
            rbsky::xrpc::Error::Xrpc(e) => println!("XRPC Protocol Error: {:#?}", e),
            rbsky::xrpc::Error::Http(e) => println!("HTTP Request Error: {:#?}", e),
        },
    };
}
