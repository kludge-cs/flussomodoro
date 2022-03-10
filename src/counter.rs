use derivative::Derivative;

use crate::app::{AppNotification, AppOpts};

// logic:
// every 5 seconds of focus, the user gets 1 second of break.
// that's 5 minutes of break per 25 minutes of focus.
// once 25 minutes has elapsed, a focus session is complete.
// once the user has completed 4 focus sessions, they get 15 minutes of break.
// if the user runs out of break, their current focus session resets.

const DEFAULT_FOCUS_TIME: u16 = 25 * 60;
const DEFAULT_CLOVER_BONUS: u16 = 15 * 60;

#[derive(Derivative, Copy, Clone)]
#[derivative(Default)]
pub struct Counter {
	#[derivative(Default(value = "DEFAULT_FOCUS_TIME"))]
	original_focus_time: u16,
	#[derivative(Default(value = "DEFAULT_FOCUS_TIME"))]
	focus_time: u16,
	#[derivative(Default(value = "DEFAULT_CLOVER_BONUS"))]
	clover_break_bonus: u16,
	break_time: u16,
	#[derivative(Default(value = "1"))]
	pom: u8,
	work_state: CounterWorkState,
}

impl Counter {
	pub fn new() -> Self {
		Counter::default()
	}

	pub fn with_opts(opts: &AppOpts) -> Self {
		Counter {
			original_focus_time: opts.focus_time.unwrap_or(DEFAULT_FOCUS_TIME),
			focus_time: opts.focus_time.unwrap_or(DEFAULT_FOCUS_TIME),
			clover_break_bonus: opts
				.clover_break_bonus
				.unwrap_or(DEFAULT_CLOVER_BONUS),
			..Counter::default()
		}
	}

	pub const fn original_focus_time(&self) -> u16 {
		self.original_focus_time
	}

	pub const fn focus_time(&self) -> u16 {
		self.focus_time
	}

	pub const fn break_time(&self) -> u16 {
		self.break_time
	}

	pub const fn pom(&self) -> u8 {
		self.pom
	}

	pub const fn work_state(&self) -> CounterWorkState {
		self.work_state
	}

	pub const fn work_state_mut(&mut self) -> &mut CounterWorkState {
		&mut self.work_state
	}

	pub fn start(&mut self) {
		self.work_state = CounterWorkState::from(Some(true));
	}

	pub fn reset(&mut self) {
		self.focus_time = self.original_focus_time;
		self.work_state.0.take();
	}

	pub fn work<F: Fn(AppNotification)>(&mut self, notifier: F) {
		if !self.work_state.is_active() {
			return;
		} else if self.work_state.is_focusing() {
			if self.focus_time % 5 == 0 {
				self.break_time += 1;
			}
			self.focus_time -= 1;
		} else {
			self.break_time -= 1;
			if self.break_time == 30 {
				notifier(AppNotification::BreakAlmostOver);
			} else if self.break_time == 0 {
				self.reset();
				notifier(AppNotification::BreakOver);
			}
		}

		if self.focus_time == 0 {
			self.pom += 1;
			self.reset();
			notifier(AppNotification::PomComplete);
		}

		if self.pom == 5 {
			self.break_time += self.clover_break_bonus;
			self.pom = 1;
			notifier(AppNotification::CloverComplete);
		}
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
