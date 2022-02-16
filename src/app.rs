use clap::Parser;
use crossterm::event::KeyEvent;
use notify_rust::Notification;
use widgets::AppPage;

use crate::{counter::Counter, keys::*, terminal::Terminal};

#[derive(Default, Parser)]
#[clap(author, version, about)]
pub struct AppOpts {
	#[clap(short, long)]
	pub focus_time: Option<u16>,
	#[clap(short, long)]
	pub clover_break_bonus: Option<u16>,
	#[clap(short, long)]
	pub notify: bool,
}

pub struct App {
	pub counter: Counter,
	pub page: AppPage,
}

impl App {
	pub fn new(counter: Counter) -> Self {
		App {
			counter,
			page: AppPage::Main,
		}
	}

	pub fn with_opts(opts: &AppOpts) -> Self {
		App {
			counter: Counter::with_opts(opts),
			page: AppPage::Main,
		}
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
			HELP => {
				self.page = match self.page {
					AppPage::Help => AppPage::Main,
					_ => AppPage::Help,
				};
				false
			}
			(_, _) => false,
		}
	}

	pub fn draw_with(&mut self, terminal: &mut Terminal) {
		terminal.0.draw(|f| AppPage::render(f, self)).unwrap();
	}
}

pub enum AppNotification {
	BreakAlmostOver,
	BreakOver,
	PomComplete,
	CloverComplete,
}

impl From<AppNotification> for Notification {
	fn from(msg: AppNotification) -> Notification {
		let mut notif = Notification::new();
		notif.summary("Flussomodoro").auto_icon();
		match msg {
			AppNotification::BreakAlmostOver => {
				notif.body("Your break is almost over! Get ready!")
			}
			AppNotification::BreakOver => {
				notif.body("Your break expired and your pom has reset :(")
			}
			AppNotification::CloverComplete => {
				notif.body("Clover complete! Great job!")
			}
			AppNotification::PomComplete => {
				notif.body("Pom complete! Keep it up!")
			}
		};
		notif
	}
}

mod widgets {
	use tui::{
		backend::Backend,
		layout::{Alignment, Constraint, Direction, Layout},
		style::{Color, Modifier, Style},
		text::{Span, Spans},
		widgets::{Block, Borders, Row, Table, Tabs, Widget},
		Frame,
	};
	use enumerare::Cycle;

	use super::App;
	use crate::counter::Counter;

	#[derive(Copy, Clone, Cycle)]
	pub enum AppPage {
		Main,
		Help,
	}

	impl AppPage {
		pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {
			let size = f.size();
			let chunks = Layout::default()
				.direction(Direction::Vertical)
				.constraints(vec![Constraint::Length(3), Constraint::Min(0)])
				.split(size);

			f.render_widget(
				Block::default()
					.style(Style::default().fg(Color::White).bg(Color::Black)),
				size,
			);

			let titles = ["Counter"]
				.iter()
				.map(|t| Spans::from(vec![
					Span::styled(*t, Style::default().fg(Color::Green)),
				]))
				.collect();
			f.render_widget(
				Tabs::new(titles)
					.block(
						Block::default()
							.borders(Borders::ALL)
							.title("Flussomodoro")
							.title_alignment(Alignment::Center),
					)
					.highlight_style(Style::default().fg(Color::Yellow))
					.select(app.page as usize),
				chunks[0],
			);
			match app.page {
				AppPage::Main =>  f.render_widget(app.page.render_main(&app.counter), chunks[1]),
				AppPage::Help => f.render_widget(app.page.render_help(), chunks[1]),
			}
		}

		pub fn render_main(&self, counter: &Counter) -> impl Widget {
			Table::new(vec![Row::new(vec![
				counter.focus_time().to_string(),
				counter.break_time().to_string(),
				counter.pom().to_string(),
				counter.work_state().to_string(),
			])])
			.header(
				Row::new(vec!["Focus Time", "Break Time", "Pom", "State"])
					.style(
						Style::default().add_modifier(Modifier::UNDERLINED),
					),
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

		fn render_help(&self) -> impl Widget {
			Table::new(vec![Row::new(vec!["testing!"])])
				.header(Row::new(vec!["owo"]))
				.widths(&[Constraint::Length(15)])
		}
	}
}
