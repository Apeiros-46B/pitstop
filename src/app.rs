use futures::{channel::mpsc::Sender, SinkExt};
use iced::{window, Subscription, Task};

use crate::{ipc::IpcMsg, util::Fallible};

#[derive(Default)]
pub struct App {
	x: i32,
	tx: Option<Sender<()>>,
	win: Option<window::Id>,
}

#[derive(Clone, Debug)]
pub enum Msg {
	IpcReady(Sender<()>),
	Ipc(IpcMsg),
	CloseWindow(window::Id),
	Dummy,
}

// elm
impl App {
	pub fn start(self) -> Fallible {
		iced::daemon("pitstop", Self::update, Self::view)
			.subscription(Self::subscription)
			.run_with(|| (self, Task::none()))?;
		Ok(())
	}

	fn update(&mut self, msg: Msg) -> Task<Msg> {
		match msg {
			Msg::IpcReady(tx) => {
				self.tx = Some(tx);
			},
			Msg::Ipc(imsg) => {
				self.ipc_ping();
				match imsg {
					IpcMsg::Quit => return iced::exit(),
					IpcMsg::OpenWindow(str) => {
						// TODO: set search bar content
						return self.open_window();
					},
					IpcMsg::Delta(x) => {
						self.x += x;
					},
				}
			},
			Msg::CloseWindow(id) => self.on_window_close(id),
			Msg::Dummy => {},
		}

		Task::none()
	}

	fn view(&self, _: window::Id) -> iced::Element<Msg> {
		iced::widget::text(self.x.to_string()).into()
	}

	fn subscription(&self) -> Subscription<Msg> {
		Subscription::batch(vec![
			self.ipc_subscription(),
			window::close_events().map(Msg::CloseWindow),
		])
	}
}

// ipc
impl App {
	fn ipc_subscription(&self) -> Subscription<Msg> {
		Subscription::run(|| {
			iced::stream::try_channel(100, crate::ipc::server_listen_ipc)
		})
		.map(Result::unwrap)
	}

	// must ping after receiving any ipc message,
	// otherwise the ipc Subscription stops executing
	fn ipc_ping(&mut self) {
		if let Some(tx) = &mut self.tx {
			futures::executor::block_on(tx.send(())).unwrap();
		}
	}
}

// window management
impl App {
	fn open_window(&mut self) -> Task<Msg> {
		if self.win.is_none() {
			let (id, task) = window::open(window::Settings {
				size: iced::Size::new(400.0, 200.0),
				position: window::Position::Centered,
				visible: true,
				resizable: false,
				decorations: false,
				transparent: false,
				level: window::Level::AlwaysOnTop,
				platform_specific: window::settings::PlatformSpecific {
					application_id: "pitstop".to_string(),
					override_redirect: false,
				},
				// exit_on_close_request: false,
				exit_on_close_request: true,
				..Default::default()
			});
			self.win = Some(id);
			task.map(|_| Msg::Dummy)
		} else {
			Task::none()
		}
	}

	fn on_window_close(&mut self, close_id: window::Id) {
		if let Some(id) = self.win {
			if id == close_id {
				self.win = None;
				dbg!(self.win);
			}
		}
	}
}
