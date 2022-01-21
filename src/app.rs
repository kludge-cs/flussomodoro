use std::io::{stdout, Error, Stdout};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::{backend::CrosstermBackend, terminal::CompletedFrame, Terminal};

use crate::counter::Counter;

type CrossTerminal = Terminal<CrosstermBackend<Stdout>>;

pub struct App {
	counter: Counter,
	terminal: CrossTerminal,
}

impl App {
	pub fn new(counter: Counter) -> Self {
		App {
			counter,
			terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
		}
	}

	pub fn setup_term(mut self) -> Result<Self, Error> {
		enable_raw_mode()?;
		self.terminal.clear()?;
		Ok(self)
	}

	pub fn handle_msg(&mut self, msg: AppMessage) {
		match msg {
			AppMessage::Start => {
				self.counter.start();
				self.render().unwrap();
			}
			AppMessage::Render => {
				self.render().unwrap();
			}
			AppMessage::ToggleActive => self.counter.toggle_active(),
			AppMessage::ToggleBreak => self.counter.toggle_break(),
			AppMessage::Work => self.counter.work(),
		}
	}

	pub fn handle_msgs(&mut self, msgs: Vec<AppMessage>) {
		msgs.iter().for_each(|msg| self.handle_msg(*msg));
	}

	pub fn render(&mut self) -> Result<CompletedFrame, Error> {
		let counter = &self.counter;
		self.terminal.draw(|frame| {
			frame.render_widget(counter.to_table(), frame.size());
		})
	}
}

impl Drop for App {
	fn drop(&mut self) {
		disable_raw_mode().unwrap();
		self.terminal.clear().unwrap();
	}
}

#[derive(Clone, Copy)]
pub enum AppMessage {
	Start,
	Work,
	ToggleActive,
	ToggleBreak,
	Render,
}
