use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "levianaut")]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server {
        #[arg(long, default_value = "127.0.0.1:8096")]
        addr: String,
    },
}

pub async fn run() -> std::io::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Server { addr } => levianaut_server::run(&addr).await?,
    }

    Ok(())
}
