mod app;
mod config;
mod k8s;
mod model;
mod ui;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "r9s", version, about = "Kubernetes TUI manager")]
struct Cli {
    /// Kubernetes namespace to start in
    #[arg(short, long)]
    namespace: Option<String>,

    /// Path to kubeconfig file
    #[arg(long, env = "KUBECONFIG")]
    kubeconfig: Option<String>,

    /// Kubernetes context to use
    #[arg(long)]
    context: Option<String>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// Run in headless (non-interactive) mode
    #[arg(long)]
    headless: bool,

    /// Initial command to run (e.g. "pods", "deploy", "svc")
    #[arg(long)]
    command: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up file-based logging so TUI output is not polluted.
    let log_dir = config::log_dir();
    std::fs::create_dir_all(&log_dir)?;
    let file_appender = tracing_appender::rolling::daily(&log_dir, "r9s.log");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_new(&cli.log_level).unwrap_or_else(|_| EnvFilter::new("info")))
        .with_writer(file_appender)
        .with_ansi(false)
        .init();

    tracing::info!("r9s starting");

    // Build the Kubernetes client.
    let client = k8s::create_client(cli.kubeconfig.as_deref(), cli.context.as_deref()).await?;

    // Bootstrap application state.
    let mut application = app::App::new(client, cli.namespace);

    if cli.headless {
        tracing::info!("running in headless mode");
        return Ok(());
    }

    // Enter the TUI event loop.
    ui::run(&mut application).await?;

    tracing::info!("r9s exiting");
    Ok(())
}
