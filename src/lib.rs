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

mod time_fmt {
	use std::fmt::{self, Display, Formatter};

	/// An optimized duration formatter.
	///
	/// For second values, prefer `SSs`
	/// For minute values, prefer `MM:SS`
	/// For hour values, prefer `HH:MM:SS`
	pub struct FormattedTime(u16);

	impl From<u16> for FormattedTime {
		fn from(x: u16) -> Self {
			FormattedTime(x)
		}
	}

	impl Display for FormattedTime {
		fn fmt(&self, f: &mut Formatter) -> fmt::Result {
			if self.0 < 60 {
				return write!(f, "{:02}s", self.0);
			}
			let (secs, mins) = (self.0 % 60, self.0 / 60);
			if mins < 60 {
				return write!(f, "{:02}:{:02}", mins, secs);
			}
			let (mins, hours) = (mins % 60, mins / 60);
			write!(f, "{:02}:{:02}:{:02}", hours, mins, secs)
		}
	}

	#[cfg(test)]
	mod tests {
		use super::FormattedTime;

		#[test]
		fn format_secs() {
			assert_eq!(FormattedTime::from(0).to_string(), "00s");
			assert_eq!(FormattedTime::from(32).to_string(), "32s");
		}

		#[test]
		fn format_mins() {
			assert_eq!(FormattedTime::from(1 * 60).to_string(), "01:00");
			assert_eq!(FormattedTime::from(32 * 60 + 1).to_string(), "32:01");
			assert_eq!(FormattedTime::from(32 * 60 + 32).to_string(), "32:32");
		}

		#[test]
		fn format_hours() {
			assert_eq!(FormattedTime::from(1 * 3600).to_string(), "01:00:00");
			assert_eq!(
				FormattedTime::from(15 * 3600 + 1 * 60).to_string(),
				"15:01:00"
			);
			assert_eq!(
				FormattedTime::from(15 * 3600 + 32 * 60).to_string(),
				"15:32:00"
			);
			assert_eq!(
				FormattedTime::from(15 * 3600 + 32 * 60 + 1).to_string(),
				"15:32:01"
			);
			assert_eq!(
				FormattedTime::from(15 * 3600 + 32 * 60 + 32).to_string(),
				"15:32:32"
			);
		}
	}
}
