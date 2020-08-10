/*
cargo.toml:
structopt="*"
*/

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "astryx")]
struct Opt {
    /// Command
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// start a server
    Serve {
        /// Input file
        #[structopt(parse(from_os_str))]
        file: PathBuf,
        port: Option<u32>,
    },
    /// build the project
    Build {
        /// Input file
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    New,
}

pub fn main() {
    let opt = Opt::from_args();

    match opt.command {
        Command::Serve{ file, port } => server::start(file, port.unwrap_or(8888)),
        Command::Build{ .. } => Ok(()),
        Command::New => new_project(),
    };
}
