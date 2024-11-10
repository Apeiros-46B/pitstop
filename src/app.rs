use futures::{channel::mpsc, SinkExt};
use iced::{advanced, stream, window, Event, Subscription, Task};

use crate::{
	components::{ListMode, ListMsg, Search, SearchMsg},
	ipc::{self, IpcMsg},
	util::Fallible,
};

#[derive(Default)]
pub struct App {
	win: Option<window::Id>,
	ipc_tx: Option<mpsc::Sender<IpcMsg>>,

	list: crate::components::List,
	search: crate::components::Search,
}

#[derive(Clone, Debug)]
pub enum Msg {
	IpcReady(mpsc::Sender<IpcMsg>),
	Ipc(IpcMsg),
	CloseWindow,
	WindowClosedExternally(window::Id),

	List(ListMsg),
	Search(SearchMsg),
	Focus,

	Dummy,
}

impl App {
	pub fn start(mut self) -> Fallible {
		#[cfg(not(windows))]
		{
			// cleanup socket on panics
			// (cleanup on clean exit is handled by App::exit_subscription)
			use std::panic;
			let hook = panic::take_hook();
			panic::set_hook(Box::new(move |info| {
				ipc::server_cleanup_socket();
				hook(info);
			}));
		}

		self.list.set_mode(ListMode::Omni);

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
			Msg::Ipc(imsg) => match imsg {
				IpcMsg::C2SQuit => {
					self.ping();
					ipc::server_cleanup_socket();
					return iced::exit();
				},
				IpcMsg::C2SOpenWindow(query) => {
					self.ping();
					self.search = Search::with_text(&query.unwrap_or_default());
					return self.open_window();
				},
				_ => panic!("server received server->client signal"),
			},
			Msg::CloseWindow => return self.close_window(),
			Msg::WindowClosedExternally(id) => self.on_window_close(id),

			Msg::List(lmsg) => return self.list.update(lmsg),
			Msg::Search(smsg) => self.search.update(smsg),
			// Msg::Focus => return self.search.focus(),
			Msg::Focus => {},

			Msg::Dummy => {},
		}
		Task::none()
	}

	fn view(&self, _: window::Id) -> iced::Element<Msg> {
		iced::widget::column![self.search.view(), self.list.view().map(Msg::List),]
			.into()
	}

	fn subscription(&self) -> Subscription<Msg> {
		Subscription::batch(vec![
			self.ipc_subscription(),
			window::close_events().map(Msg::WindowClosedExternally),
			iced::event::listen().map(Self::handle_events),
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
	fn ping(&mut self) {
		self.send_to_ipc_thread(IpcMsg::InternalPing);
	}

	fn send_to_ipc_thread(&mut self, imsg: IpcMsg) {
		if let Some(tx) = &mut self.ipc_tx {
			futures::executor::block_on(tx.send(imsg)).unwrap();
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
			task.map(|_| Msg::Dummy).chain(advanced::widget::operate(
				advanced::widget::operation::focusable::focus_next(),
			))
		} else {
			Task::none()
		}
	}

	fn close_window(&mut self) -> Task<Msg> {
		if let Some(id) = self.win {
			self.on_window_close(id);
			window::close(id)
		} else {
			Task::none()
		}
	}

	fn on_window_close(&mut self, close_id: window::Id) {
		if let Some(id) = self.win {
			if id == close_id {
				self.win = None;
			}
		}
	}
}

// key handling
impl App {
	// handles miscellaneous keypresses that are not captured by the Search input
	// TODO: configurable binds
	fn handle_events(evt: Event) -> Msg {
		use iced::keyboard;

		match evt {
			Event::Keyboard(keyboard::Event::KeyPressed {
				key: keyboard::Key::Character(s),
				modifiers,
				..
			}) => match s.parse::<usize>() {
				Ok(n) if n > 0 && n < 10 && (modifiers.control() ^ modifiers.alt()) => {
					// Ctrl+N = select N
					// Alt+N = confirm N
					dbg!(Msg::List(ListMsg::Nth {
						n: n - 1,
						confirm: modifiers.alt(),
					}))
				},
				_ => Msg::Focus,
			},
			_ => Msg::Focus,
		}
	}
}

// exit hook
#[cfg(not(windows))]
impl App {
	fn exit_subscription(&self) -> Subscription<Msg> {
		Subscription::run(|| stream::try_channel(100, ipc::server_listen_exit_hook))
			.map(Result::unwrap)
	}
}
