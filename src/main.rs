use clap::Parser;

use app::App;
use ipc::{client_send, IpcMsg};
use util::Fallible;

mod app;
mod components;
mod ipc;
mod style;
mod util;

#[derive(clap_derive::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	cmd: Commands,
}

#[derive(clap_derive::Subcommand)]
enum Commands {
	Start,
	Quit,
	OpenWindow {
		/// A query to preemtively enter into the search bar.
		#[arg(short, long)]
		query: Option<String>
	},
}

fn main() -> Fallible {
	let cli = Cli::parse();
	match &cli.cmd {
		Commands::Start => App::default().start(),
    Commands::Quit => client_send(IpcMsg::Quit),
    Commands::OpenWindow { query } => client_send(IpcMsg::OpenWindow(query.clone())),
	}
}
