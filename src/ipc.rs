use std::io::{BufRead, BufReader, Write};

use futures::{channel::mpsc, SinkExt, StreamExt};
use interprocess::local_socket::{
	prelude::*, GenericFilePath, ListenerOptions, Stream
};
use serde::{Deserialize, Serialize};

use crate::{app::Msg, util::Fallible};

#[cfg(not(windows))]
const IPC_NAME: &str = "/tmp/pitstop_ipc_channel.sock";
#[cfg(windows)]
const IPC_NAME: &str = r"\\.\pipe\/tmp/pitstop_ipc_channel.sock";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IpcMsg {
	Quit,
	OpenWindow(Option<String>),
}

pub fn client_send(imsg: IpcMsg) -> Fallible {
	let name = IPC_NAME.to_fs_name::<GenericFilePath>()?;
	let conn = Stream::connect(name)?;
	let mut conn = BufReader::new(conn);

	conn
		.get_mut()
		.write_all(ron::to_string(&imsg)?.as_bytes())?;

	Ok(())
}

pub fn server_cleanup_ipc() {
	println!("cleaning up socket");
	std::fs::remove_file(IPC_NAME).unwrap()
}

// called from iced subscription
pub async fn server_listen_ipc(mut output: mpsc::Sender<Msg>) -> Fallible {
	let (tx, mut rx) = mpsc::channel(1);
	output.send(Msg::IpcReady(tx)).await?;

	let name = IPC_NAME.to_fs_name::<GenericFilePath>()?;
	let listener = match ListenerOptions::new().name(name).create_sync() {
		#[cfg(not(windows))]
		Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
			anyhow::bail!(
				"the socket file '{IPC_NAME}' is occupied. please check if
				there are other running instances of pitstop using this socket."
			);
		},
		x => x?,
	};

	let mut buf = String::with_capacity(128);

	for conn in listener.incoming() {
		let mut conn = BufReader::new(conn?);
		conn.read_line(&mut buf)?;
		output
			.send(ron::from_str::<IpcMsg>(&buf).map(Msg::Ipc)?)
			.await?;
		buf.clear();
		rx.select_next_some().await; // recv ping
	}

	Ok(())
}

// called from iced subscription
#[cfg(not(windows))]
pub async fn server_listen_exit_hook(mut output: mpsc::Sender<Msg>) -> Fallible {
	use std::sync::{self, atomic};

	let running = sync::Arc::new(sync::atomic::AtomicBool::new(true));
	let r = running.clone();

	ctrlc::set_handler(move || r.store(false, atomic::Ordering::SeqCst))?;

	while running.load(atomic::Ordering::SeqCst) {
		use futures_time as ftime;
		// TODO: when the atomicbool becomes false, instantly exit
		// -> consider condvar? idk might not work in async
		ftime::task::sleep(ftime::time::Duration::from_secs(5)).await;
	}
	// signal the iced runtime to exit once ctrl+c hooked
	output.send(Msg::Ipc(IpcMsg::Quit)).await?;

	Ok(())
}
