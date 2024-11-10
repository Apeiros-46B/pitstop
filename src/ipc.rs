use std::io::{BufRead, BufReader, Write};

use futures::{channel::mpsc, SinkExt, StreamExt};
use interprocess::local_socket::{
	prelude::*, GenericFilePath, ListenerOptions, Stream, ToFsName,
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
	Delta(i32),
}

pub async fn server_listen_ipc(mut output: mpsc::Sender<Msg>) -> Fallible {
	let (tx, mut rx) = mpsc::channel(1);
	output.send(Msg::IpcReady(tx)).await?;

	let name = IPC_NAME.to_fs_name::<GenericFilePath>()?;
	let listener = ListenerOptions::new().name(name).create_sync()?;

	let mut buf = String::with_capacity(128);

	for mut conn in listener
		.incoming()
		.filter_map(Result::ok)
		.map(BufReader::new)
	{
		conn.read_line(&mut buf)?;
		output
			.send(ron::from_str::<IpcMsg>(&buf).map(Msg::Ipc)?)
			.await?;
		buf.clear();
		rx.select_next_some().await; // recv ping
	}

	Ok(())
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
