pub mod app;
pub mod counter;

use app::AppMessage;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

macro_rules! gen_keypair {
	($name:ident: $mods:expr, $code:expr) => {
		pub const $name: (KeyModifiers, KeyCode) = ($mods, $code);
	};
}

gen_keypair!(QUIT: KeyModifiers::NONE, KeyCode::Char('q'));
gen_keypair!(SIGINT: KeyModifiers::CONTROL, KeyCode::Char('c'));
gen_keypair!(PAUSE: KeyModifiers::NONE, KeyCode::Char('p'));
gen_keypair!(BREAK: KeyModifiers::NONE, KeyCode::Char('b'));

pub fn handle_key_event(event: KeyEvent) -> (bool, Option<AppMessage>) {
	match (event.modifiers, event.code) {
		SIGINT | QUIT => (true, None),
		PAUSE => (false, Some(AppMessage::ToggleActive)),
		BREAK => (false, Some(AppMessage::ToggleBreak)),
		(_, _) => (false, None),
	}
}
