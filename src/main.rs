use clap::Parser;

mod server_starter;
mod version;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(index = 1)]
    path_to_jar: String,
    #[arg(long, short)]
    world_name: Option<String>,
}

fn main() {
    let args = Cli::parse();
    server_starter::start_server(&args.path_to_jar[..], "1");
}
