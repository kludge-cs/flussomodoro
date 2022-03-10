#![feature(once_cell)]
#![feature(mixed_integer_ops)]

pub mod app;
pub mod counter;
pub mod terminal;
mod ui;

mod keys {
	use crossterm::event::{KeyCode, KeyModifiers};

	pub type KeyPair = (KeyModifiers, KeyCode);

	pub const QUIT: KeyPair = (KeyModifiers::NONE, KeyCode::Char('q'));
	pub const SIGINT: KeyPair = (KeyModifiers::CONTROL, KeyCode::Char('c'));
	pub const PAUSE: KeyPair = (KeyModifiers::NONE, KeyCode::Char('p'));
	pub const BREAK: KeyPair = (KeyModifiers::NONE, KeyCode::Char('b'));
	pub const HELP: KeyPair = (KeyModifiers::NONE, KeyCode::Char('h'));
	pub const VI_DOWN: KeyPair = (KeyModifiers::NONE, KeyCode::Char('j'));
	pub const VI_UP: KeyPair = (KeyModifiers::NONE, KeyCode::Char('k'));
}
