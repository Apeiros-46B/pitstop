use futures::{channel::mpsc, SinkExt};
use iced::{stream, window, Subscription, Task};

use crate::{
	components::{Search, SearchMsg}, ipc::{self, IpcMsg}, util::Fallible
};

#[derive(Default)]
pub struct App {
	win: Option<window::Id>,
	ipc_tx: Option<mpsc::Sender<()>>,

	search: crate::components::Search,
}

#[derive(Clone, Debug)]
pub enum Msg {
	IpcReady(mpsc::Sender<()>),
	Ipc(IpcMsg),
	CloseWindow(window::Id),

	Search(SearchMsg),

	Dummy,
}

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
				self.ipc_tx = Some(tx);
			},
			Msg::Ipc(imsg) => {
				self.ipc_ping();
				match imsg {
					IpcMsg::Quit => {
						ipc::server_cleanup_ipc();
						return iced::exit();
					},
					IpcMsg::OpenWindow(str) => {
						self.search = Search::with_text(&str.unwrap_or_default());
						return self.open_window();
					},
				}
			},
			Msg::CloseWindow(id) => self.on_window_close(id),

			Msg::Search(smsg) => self.search.update(smsg),

			Msg::Dummy => {},
		}

		Task::none()
	}

	fn view(&self, _: window::Id) -> iced::Element<Msg> {
		// TODO
		self.search.view().map(Msg::Search)
	}

	fn subscription(&self) -> Subscription<Msg> {
		Subscription::batch(vec![
			self.ipc_subscription(),
			window::close_events().map(Msg::CloseWindow),
			#[cfg(not(windows))]
			self.exit_subscription(),
		])
	}
}

// ipc
impl App {
	fn ipc_subscription(&self) -> Subscription<Msg> {
		Subscription::run(|| iced::stream::try_channel(100, ipc::server_listen_ipc))
			.map(Result::unwrap)
	}

	// must ping after receiving any ipc message,
	// otherwise the ipc Subscription stops executing
	fn ipc_ping(&mut self) {
		if let Some(tx) = &mut self.ipc_tx {
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

#[cfg(not(windows))]
impl App {
	fn exit_subscription(&self) -> Subscription<Msg> {
		Subscription::run(|| stream::try_channel(100, ipc::server_listen_exit_hook))
			.map(Result::unwrap)
	}
}
