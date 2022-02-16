use std::io::{self, Stdout};
use crossterm::{terminal::{disable_raw_mode, LeaveAlternateScreen, enable_raw_mode, EnterAlternateScreen}, execute};
use tui::{backend::{CrosstermBackend}, Terminal as TuiTerminal};

type CrossTerminal = TuiTerminal<CrosstermBackend<Stdout>>;

pub struct Terminal(pub CrossTerminal);

impl Terminal {
	pub fn with_stdout(handle: Stdout) -> Self {
		Terminal(TuiTerminal::new(CrosstermBackend::new(handle)).unwrap())
	}

	pub fn with_backend(backend: CrossTerminal) -> Self {
		Terminal(backend)
	}

	pub fn setup_backend(&mut self) -> io::Result<&mut Self> {
		enable_raw_mode()?;
		execute!(self.0.backend_mut(), EnterAlternateScreen).unwrap();
		Ok(self)
	}
}

impl Drop for Terminal {
	fn drop(&mut self) {
		disable_raw_mode().unwrap();
		execute!(self.0.backend_mut(), LeaveAlternateScreen).unwrap();
	}
}
