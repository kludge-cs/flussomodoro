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
	work_state: Option<bool>,
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

	pub fn work_state(&self) -> Option<bool> {
		self.work_state
	}

	pub fn work_state_to_string(work_state: Option<bool>) -> String {
		match work_state {
			None => "Paused".to_string(),
			Some(false) => "Break".to_string(),
			Some(true) => "Focus".to_string(),
		}
	}

	pub fn toggle_active(&mut self) {
		self.work_state = match self.work_state() {
			None => Some(true),
			Some(_) => None,
		}
	}

	pub fn toggle_break(&mut self) {
		self.work_state = self.work_state.take().map(|state| !state);
	}

	pub fn start(&mut self) {
		self.work_state = Some(true);
	}

	pub fn reset(&mut self) {
		self.focus_time = DEFAULT_FOCUS_TIME;
		self.work_state.take();
	}

	pub fn work(&mut self) {
		if self.work_state.is_none() {
			return;
		} else if self.work_state.unwrap() {
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

		if self.pom == 4 {
			self.break_time += CLOVER_BREAK_BONUS;
			self.pom = 1;
		}
	}

	pub fn to_table(&self) -> Table {
		Table::new(vec![Row::new(vec![
			self.focus_time.to_string(),
			self.break_time.to_string(),
			self.pom.to_string(),
			Self::work_state_to_string(self.work_state),
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
