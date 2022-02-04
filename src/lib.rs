pub mod app;
pub mod counter;

use app::AppMessage;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

type KeyPair = (KeyModifiers, KeyCode);

pub const QUIT: KeyPair = (KeyModifiers::NONE, KeyCode::Char('q'));
pub const SIGINT: KeyPair = (KeyModifiers::CONTROL, KeyCode::Char('c'));
pub const PAUSE: KeyPair = (KeyModifiers::NONE, KeyCode::Char('p'));
pub const BREAK: KeyPair = (KeyModifiers::NONE, KeyCode::Char('b'));

pub fn handle_key_event(event: KeyEvent) -> (bool, Option<AppMessage>) {
	match (event.modifiers, event.code) {
		SIGINT | QUIT => (true, None),
		PAUSE => (false, Some(AppMessage::ToggleActive)),
		BREAK => (false, Some(AppMessage::ToggleBreak)),
		(_, _) => (false, None),
	}
}
