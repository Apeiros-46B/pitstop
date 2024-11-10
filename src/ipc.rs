use std::io::{BufRead, BufReader, Write};

use futures::{channel::mpsc, SinkExt, StreamExt};
use interprocess::local_socket::{
	prelude::*, GenericNamespaced, ListenerOptions, Stream,
};
use serde::{Deserialize, Serialize};

use crate::{app::Msg, util::Fallible};

const IPC_NAME: &str = "pitstop.sock";
// #[cfg(not(windows))]
// const IPC_NAME: &str = "/tmp/pitstop_ipc_channel.sock";
// #[cfg(windows)]
// const IPC_NAME: &str = r"\\.\pipe\/tmp/pitstop_ipc_channel.sock";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IpcMsg {
	Quit,
	OpenWindow(Option<String>),
	Delta(i32),
}

pub async fn server_listen_ipc(mut output: mpsc::Sender<Msg>) -> Fallible {
	let (tx, mut rx) = mpsc::channel(1);
	output.send(Msg::IpcReady(tx)).await?;

	let name = IPC_NAME.to_ns_name::<GenericNamespaced>()?;
	let listener = ListenerOptions::new().name(name).create_sync()?;

	let mut buf = String::with_capacity(128);

	for conn in listener.incoming() {
		let mut conn = BufReader::new(conn?);
		conn.read_line(&mut buf)?;
		output
			.send(ron::from_str::<IpcMsg>(dbg!(&buf)).map(Msg::Ipc)?)
			.await?;
		buf.clear();
		rx.select_next_some().await; // recv ping
	}

	Ok(())
}

pub fn client_send(imsg: IpcMsg) -> Fallible {
	let name = IPC_NAME.to_ns_name::<GenericNamespaced>()?;
	let conn = Stream::connect(name)?;
	let mut conn = BufReader::new(conn);

	conn
		.get_mut()
		.write_all(dbg!(ron::to_string(&imsg)?).as_bytes())?;

	Ok(())
}
