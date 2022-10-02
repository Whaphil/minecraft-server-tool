use clap::Parser;

mod server_starter;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(index = 1)]
    version: String,
    #[arg(long, short)]
    world_name: Option<String>,
}

fn main() {
    let args = Cli::parse();
    server_starter::start_server("1.19.1", "1");
}
