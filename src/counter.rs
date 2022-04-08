use derivative::Derivative;
use tui::{
	layout::Constraint,
	style::{Color, Modifier, Style},
	widgets::{Block, Borders, Row, Table},
};

const DEFAULT_FOCUS_TIME: u16 = 25 * 60;
const CLOVER_BREAK_BONUS: u16 = 15 * 60;

#[derive(Derivative, Copy, Clone)]
#[derivative(Default)]
pub struct Counter {
	#[derivative(Default(value = "DEFAULT_FOCUS_TIME"))]
	focus_time: u16,
	break_time: u16,
	#[derivative(Default(value = "1"))]
	pom: u8,
	work_state: CounterWorkState,
}

impl Counter {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn focus_time(&self) -> u16 {
		self.focus_time
	}

	pub fn break_time(&self) -> u16 {
		self.break_time
	}

	pub fn pom(&self) -> u8 {
		self.pom
	}

	pub fn work_state(&self) -> CounterWorkState {
		self.work_state
	}

	pub fn work_state_mut(&mut self) -> &mut CounterWorkState {
		&mut self.work_state
	}

	pub fn start(&mut self) {
		self.work_state = CounterWorkState::from(Some(true));
	}

	pub fn reset(&mut self) {
		self.focus_time = DEFAULT_FOCUS_TIME;
		self.work_state.0.take();
	}

	pub fn work(&mut self) {
		if !self.work_state.is_active() {
			return;
		} else if self.work_state.is_focusing() {
			if self.focus_time % 5 == 0 {
				self.break_time += 1;
			}
			self.focus_time -= 1;
		} else {
			self.break_time -= 1;
			if self.break_time == 0 {
				self.reset();
			}
		}

		if self.focus_time == 0 {
			self.pom += 1;
			self.reset();
		}

		if self.pom == 5 {
			self.break_time += CLOVER_BREAK_BONUS;
			self.pom = 1;
		}
	}

	pub fn to_table(&self) -> Table {
		Table::new(vec![Row::new(vec![
			self.focus_time.to_string(),
			self.break_time.to_string(),
			self.pom.to_string(),
			self.work_state.to_string(),
		])])
		.header(
			Row::new(vec!["Focus Time", "Break Time", "Pom", "State"])
				.style(Style::default().add_modifier(Modifier::UNDERLINED)),
		)
		.style(Style::default().fg(Color::White).bg(Color::Black))
		.block(Block::default().borders(Borders::ALL))
		.widths(&[
			Constraint::Length(10),
			Constraint::Length(10),
			Constraint::Length(3),
			Constraint::Length(15),
		])
		.column_spacing(2)
	}
}

#[derive(Copy, Clone, Default)]
pub struct CounterWorkState(Option<bool>);

impl CounterWorkState {
	pub fn is_active(&self) -> bool {
		self.0.is_some()
	}

	pub fn is_focusing(&self) -> bool {
		self.is_active() && self.0.unwrap()
	}

	pub fn into_inner(&self) -> Option<bool> {
		self.0
	}

	pub fn set_active(&mut self, active: bool) {
		self.0 = if active { Some(true) } else { None }
	}

	pub fn toggle_active(&mut self) {
		self.0 = match self.0 {
			None => Some(true),
			Some(_) => None,
		}
	}

	pub fn set_break(&mut self, on_break: bool) {
		self.0 = Some(on_break);
	}

	pub fn toggle_break(&mut self) {
		self.0 = self.0.take().map(|state| !state);
	}
}

impl From<Option<bool>> for CounterWorkState {
	fn from(x: Option<bool>) -> Self {
		CounterWorkState(x)
	}
}

impl ToString for CounterWorkState {
	fn to_string(&self) -> String {
		match self.0 {
			None => "Paused".to_string(),
			Some(false) => "Break".to_string(),
			Some(true) => "Focus".to_string(),
		}
	}
}
