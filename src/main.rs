use clap::Parser;

mod server_starter;
mod version;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(index = 1)]
    version_name: Option<String>,
    #[arg(long, short)]
    world_name: Option<String>,
}

fn main() {
    let versions = version::load_versions(Some(String::from("./versions.json")));
    let args = Cli::parse();
    let version_name = match args.version_name {
        Some(name) => name,
        None => match versions.len() {
            0 => String::from("newest"),
            _ => String::from(&versions[0].name),
        },
    };
    server_starter::start_server(&version_name, "1");
}
