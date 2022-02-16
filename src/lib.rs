pub mod app;
pub mod counter;
pub mod terminal;

mod keys {
	use crossterm::event::{KeyCode, KeyModifiers};

	pub type KeyPair = (KeyModifiers, KeyCode);

	pub const QUIT: KeyPair = (KeyModifiers::NONE, KeyCode::Char('q'));
	pub const SIGINT: KeyPair = (KeyModifiers::CONTROL, KeyCode::Char('c'));
	pub const PAUSE: KeyPair = (KeyModifiers::NONE, KeyCode::Char('p'));
	pub const BREAK: KeyPair = (KeyModifiers::NONE, KeyCode::Char('b'));
	pub const HELP: KeyPair = (KeyModifiers::NONE, KeyCode::Char('h'));
}
