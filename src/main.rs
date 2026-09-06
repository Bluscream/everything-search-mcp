//! everything-search-mcp — filename search via a Voidtools Everything HTTP server.

use std::sync::Arc;

use clap::Parser;
use mcp_toolkit::ServerOptions;

use everything_search_mcp::tools::{Endpoint, SearchTools};

#[derive(Parser, Debug)]
#[command(
    name = "everything-search-mcp",
    version,
    about = "Filename search through a Voidtools Everything HTTP server"
)]
struct Cli {
    #[command(flatten)]
    server: ServerOptions,

    /// Host running Everything's HTTP server.
    #[arg(long, default_value = "127.0.0.1", env = "EVERYTHING_HTTP_HOST")]
    everything_host: String,

    /// Port Everything's HTTP server listens on.
    #[arg(long, default_value_t = 14680, env = "EVERYTHING_HTTP_PORT")]
    everything_port: u64,

    /// Username, if Everything's HTTP server requires authentication.
    #[arg(long, env = "EVERYTHING_HTTP_USERNAME")]
    everything_username: Option<String>,

    /// Password for that user. Prefer the environment variable over a flag,
    /// which would otherwise be visible in the process list.
    #[arg(long, env = "EVERYTHING_HTTP_PASSWORD", hide_env_values = true)]
    everything_password: Option<String>,
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    let endpoint = Endpoint {
        host: cli.everything_host,
        port: cli.everything_port,
        username: cli.everything_username,
        password: cli.everything_password,
    };
    let group = Arc::new(SearchTools::new(endpoint));

    match mcp_toolkit::run("everything", env!("CARGO_PKG_VERSION"), group, cli.server).await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("everything-search-mcp: {err}");
            std::process::ExitCode::FAILURE
        }
    }
}
