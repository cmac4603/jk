use clap::Parser;

mod cfg;
mod clients;
mod cmds;
mod tools;

#[derive(Parser)]
enum Cli {
    /// Dependabot commands
    #[command(subcommand)]
    Dependabot(cmds::dependabot::ManageDependabot),
    /// Pull request commands
    #[command(subcommand)]
    Pr(cmds::pr::PrCommand),
    /// Self-update
    Update,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = cfg::JkConfig::get()?;
    match Cli::parse() {
        // Cli::Dependabot(d) => println!("{}", d.run_cmd(config).await?),
        Cli::Dependabot(d) => d.run_cmd(config).await?,
        Cli::Pr(pr) => println!("{}", pr.run_cmd(config).await?),
        Cli::Update => println!("{}", cmds::update::update().await?),
    }
    Ok(())
}
