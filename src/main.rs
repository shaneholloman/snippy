use clap::{Parser, Subcommand};
use snippy::{
    copy_files_to_clipboard, watch_clipboard, ClipboardCopierConfig, ClipboardWatcherConfig,
};
use tracing_subscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    cmd: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
enum SubCommands {
    Copy(CopyArgs),
    Watch(WatchArgs),
}

#[derive(Parser, Debug, Clone)]
struct CopyArgs {
    #[arg(required = true)]
    files: Vec<String>,
    #[arg(short = 'm', long, default_value = "false")]
    no_markdown: bool,
    #[arg(short = 'l', long, default_value = None)]
    line_number: Option<usize>,
    #[arg(short = 'p', long, default_value = "|")]
    prefix: String,
    #[arg(short = 'M', long, default_value = "gpt-4o")]
    model: String,
    #[arg(short = 's', long, default_value = "false")]
    no_stats: bool,
    #[arg(long, default_value = "MarkdownHeading")]
    filename_format: Option<String>,
    #[arg(long, default_value = "# Relevant Code\n")]
    pub first_line: String,
}

#[derive(Parser, Debug, Clone)]
struct WatchArgs {
    #[arg(short = 'x', long)]
    watch_path: Option<String>,
    #[arg(short = 'i', long, default_value_t = 1000)]
    interval_ms: u64,
    #[arg(long, default_value = "# Relevant Code")]
    pub first_line: String,
}

#[tokio::main]
async fn main() {
    let cli_args = CliArgs::parse();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    match cli_args.cmd {
        SubCommands::Copy(args) => {
            let copier_config = ClipboardCopierConfig {
                no_markdown: args.no_markdown,
                line_number: args.line_number,
                prefix: args.prefix.clone(),
                model: args.model.clone(),
                no_stats: args.no_stats,
                filename_format: args
                    .filename_format
                    .clone()
                    .unwrap_or_else(|| "None".to_owned()),
                first_line: args.first_line,
            };
            if let Err(e) = copy_files_to_clipboard(copier_config, args.files).await {
                eprintln!("Error copying files to clipboard: {}", e);
            }
        }
        SubCommands::Watch(args) => {
            let watcher_config = ClipboardWatcherConfig {
                interval_ms: args.interval_ms,
                watch_path: args.watch_path,
                first_line: args.first_line,
            };
            if let Err(e) = watch_clipboard(watcher_config).await {
                eprintln!("Error watching clipboard: {}", e);
            }
        }
    }
}
