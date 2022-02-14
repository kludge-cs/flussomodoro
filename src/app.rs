use std::io::{stdout, Error, Stdout};

use clap::Parser;
use crossterm::{
	event::KeyEvent,
	terminal::{disable_raw_mode, enable_raw_mode},
};
use notify_rust::Notification;
use tui::{backend::CrosstermBackend, Terminal};

use crate::{counter::Counter, keys::*};

type CrossTerminal = Terminal<CrosstermBackend<Stdout>>;

#[derive(Default, Parser)]
#[clap(author, version, about)]
pub struct AppOpts {
	#[clap(short, long)]
	pub focus_time: Option<u16>,
	#[clap(short, long)]
	pub clover_break_bonus: Option<u16>,
	#[clap(short, long)]
	pub notify: bool,
}

pub struct App {
	pub counter: Counter,
	terminal: CrossTerminal,
}

impl App {
	pub fn new(counter: Counter) -> Self {
		App {
			counter,
			terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
		}
	}

	pub fn with_opts(opts: &AppOpts) -> Self {
		App {
			counter: Counter::with_opts(&opts),
			terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
		}
	}

	pub fn setup_term(mut self) -> Result<Self, Error> {
		enable_raw_mode()?;
		self.terminal.clear()?;
		Ok(self)
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
			(_, _) => false,
		}
	}

	pub fn render(&mut self) {
		let counter = &self.counter;
		self.terminal
			.draw(|frame| {
				frame.render_widget(counter.to_table(), frame.size());
			})
			.expect("Rendering failed.");
	}
}

impl Drop for App {
	fn drop(&mut self) {
		disable_raw_mode().unwrap();
		self.terminal.clear().unwrap();
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
				notif.body("Your break expired and your pom has reset :(")
			}
			AppNotification::CloverComplete => {
				notif.body("Clover complete! Great job!")
			}
			AppNotification::PomComplete => {
				notif.body("Pom complete! Keep it up!")
			}
		};
		notif
	}
}
