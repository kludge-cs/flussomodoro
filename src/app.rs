use clap::Parser;
use crossterm::event::KeyEvent;
use notify_rust::Notification;

use crate::{
	counter::Counter,
	keys::*,
	terminal::Terminal,
	ui::{AppPage, Page},
};

#[derive(Clone, Default, Parser)]
#[clap(author, version, about)]
pub struct AppOpts {
	#[clap(short, long)]
	/// Number of seconds per focus session
	pub focus_time: Option<u16>,
	#[clap(short, long)]
	/// Number of seconds of bonus break awarded for completing 4 focus
	/// sessions
	pub clover_break_bonus: Option<u16>,
	#[clap(short, long)]
	/// Whether or not to send notifications
	pub notify: bool,
	/// Whether or not to use ASCII art instead of gauges
	#[clap(short, long)]
	pub ascii: bool,
}

#[derive(Default)]
pub struct App {
	pub counter: Counter,
	pub page: AppPage,
	pub opts: AppOpts,
}

impl App {
	pub fn new(counter: Counter) -> Self {
		App { counter, ..Default::default() }
	}

	pub fn with_opts(opts: &AppOpts) -> Self {
		App {
			counter: Counter::with_opts(opts),
			opts: opts.clone(),
			..Default::default()
		}
	}

	pub fn handle_key_event(&mut self, event: KeyEvent) -> bool {
		match (event.modifiers, event.code) {
			SIGINT | QUIT => true,
			PAUSE => {
				self.counter.work_state_mut().toggle_active();
				false
			}
			BREAK => {
				self.counter.work_state_mut().toggle_break();
				false
			}
			HELP => {
				self.page = self.page.toggle_help();
				false
			}
			VI_DOWN => {
				self.page.scroll_by(1);
				false
			}
			VI_UP => {
				self.page.scroll_by(-1);
				false
			}
			(_, _) => false,
		}
	}

	pub fn draw_with(&self, terminal: &mut Terminal) {
		terminal.0.draw(|f| self.page.render(f.size(), f, self)).unwrap();
	}
}

pub enum AppNotification {
	BreakAlmostOver,
	BreakOver,
	PomComplete,
	CloverComplete,
}

impl From<AppNotification> for Notification {
	fn from(msg: AppNotification) -> Notification {
		let mut notif = Notification::new();
		notif.summary("Flussomodoro").auto_icon();
		match msg {
			AppNotification::BreakAlmostOver => {
				notif.body("Your break is almost over! Get ready!")
			}
			AppNotification::BreakOver => {
				notif.body("Your break expired and your session has reset :(")
			}
			AppNotification::CloverComplete => {
				notif.body("Clover complete! Great job!")
			}
			AppNotification::PomComplete => {
				notif.body("Session complete! Keep it up!")
			}
		};
		notif
	}
}
