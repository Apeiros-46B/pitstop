use clap::Parser;

use app::App;
use ipc::{IpcConnection, IpcMsg};
use util::Fallible;

mod app;
mod components;
mod ipc;
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
	Open {
		/// A query to preemptively enter into the search bar.
		#[arg(short, long)]
		query: Option<String>,

		// /// dmenu mode (take menu entries from stdin)
		// #[arg(long)]
		// dmenu: bool,
	},
}

fn main() -> Fallible {
	let cli = Cli::parse();
	match &cli.cmd {
		Commands::Start => App::default().start(),
		Commands::Quit => IpcConnection::connect()?.send(IpcMsg::C2SQuit),
		Commands::Open { query } => {
			let mut conn = IpcConnection::connect()?;
			conn.send(IpcMsg::C2SOpenWindow(query.clone()))?;

			// conn.send(IpcMsg::C2SOpenWindow {
			// 	query: query.clone(),
			// 	dmenu: *dmenu,
			// })?;

			// if !*dmenu {
			// 	return Ok(());
			// }
			// conn.send(IpcMsg::C2SDmenuPopulate(
			// 	std::io::stdin().lock().read_until.map(Result::unwrap).collect(),
			// ))?;

			// let mut buf = String::with_capacity(128);
			// if let IpcMsg::S2CDmenuResult(res) = conn.recv(&mut buf)? {
			// 	println!("{res}")
			// }

			Ok(())
		},
	}
}
