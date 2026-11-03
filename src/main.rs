use anyhow::Result;
use clap::Parser;
use dashboard::dashboard;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    production: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = dashboard::Config { 
        production_mode: cli.production, 
        ..Default::default() 
    };
    let mut component = dashboard::new(config);
    
    component.initialize()?;
    component.start()?;
    
    println!("SigmaOS-dashboard running");
    Ok(())
}
