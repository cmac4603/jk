use clap::Parser;

mod cfg;
mod clients;
mod cmds;
mod tools;

#[derive(Parser)]
enum Cli {
    /// Pull rqeuest commands
    #[command(subcommand)]
    Pr(cmds::pr::PrCommand),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = cfg::JkConfig::get()?;
    match Cli::parse() {
        Cli::Pr(pr) => println!("{}", pr.run_cmd(config).await?),
    }
    Ok(())
}
