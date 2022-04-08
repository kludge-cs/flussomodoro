use std::io::{stdout, Error, Stdout};

use crossterm::{
	event::KeyEvent,
	terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{counter::Counter, keys::*};

type CrossTerminal = Terminal<CrosstermBackend<Stdout>>;

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

	pub fn start(&mut self) {
		self.counter.start();
		self.render();
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
