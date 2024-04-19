use clap::Parser;

#[derive(Parser, Clone)]
pub struct Config {
    #[clap(short, long, env)]
    pub port: u16,
}
