use enumerare::{Cycle, DefaultEnum};
use tui::{
	backend::Backend,
	layout::{Alignment, Constraint, Direction, Layout},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
	widgets::{Block, Borders, Gauge, Paragraph, Tabs, Widget},
	Frame,
};
use tui_circular_gauge::CircularGauge;

use crate::{app::App, time_fmt::FormattedTime};

#[derive(Copy, Clone, Cycle, DefaultEnum)]
pub enum AppPage {
	Main,
	Help,
}

impl AppPage {
	pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {
		let chunks = Layout::default()
			.constraints(vec![Constraint::Length(3), Constraint::Min(0)])
			.split(f.size());

		let titles = ["Counter"]
			.iter()
			.map(|t| {
				Spans::from(Span::styled(
					*t,
					Style::default().fg(Color::Green),
				))
			})
			.collect();
		f.render_widget(
			Tabs::new(titles)
				.block(
					Block::default()
						.borders(Borders::ALL)
						.style(Style::default().fg(Color::White))
						.title("Flussomodoro")
						.title_alignment(Alignment::Center),
				)
				.highlight_style(Style::default().fg(Color::Yellow))
				.select(app.page as usize),
			chunks[0],
		);
		match app.page {
			AppPage::Main => {
				let focus_time = *app.counter.focus_time();
				let original_focus_time = *app.counter.original_focus_time();

				let main_chunks = Layout::default()
					.constraints(vec![
						Constraint::Length(5),
						Constraint::Min(10),
					])
					.split(chunks[1]);
				let status_session_chunks = Layout::default()
					.direction(Direction::Horizontal)
					.constraints(vec![
						Constraint::Percentage(50),
						Constraint::Percentage(50),
					])
					.split(main_chunks[0]);
				let focus_break_chunks = Layout::default()
					.direction(Direction::Horizontal)
					.constraints(vec![
						Constraint::Percentage(50),
						Constraint::Percentage(50),
					])
					.split(main_chunks[1]);
				f.render_widget(
					Paragraph::new(vec![
						Spans::from(Span::raw("Current task: TBD!")),
						Spans::from(Span::raw("Task progress: TBD!")),
						Spans::from(format!(
							"Status: {}",
							app.counter.work_state().to_string()
						)),
					])
					.block(
						Block::default().borders(Borders::ALL).title("Status"),
					)
					.style(Style::default().fg(Color::White)),
					status_session_chunks[0],
				);
				f.render_widget(
					Gauge::default()
						.block(
							Block::default()
								.borders(Borders::ALL)
								.style(Style::default().fg(Color::White))
								.title("Session"),
						)
						.gauge_style(
							Style::default().fg(Color::Green).bg(Color::Black),
						)
						.label(format!("{}/4", app.counter.pom()))
						.use_unicode(true)
						.ratio(*app.counter.pom() as f64 / 4.0),
					status_session_chunks[1],
				);
				f.render_widget(
					focus_gauge(focus_time, original_focus_time),
					focus_break_chunks[0],
				);
				f.render_widget(
					Paragraph::new(vec![
						Spans::from(format!(
							"Break: {} accumulated",
							FormattedTime::from(*app.counter.break_time())
						)),
					])
					.block(
						Block::default().borders(Borders::ALL).title("Break"),
					)
					.style(Style::default().fg(Color::White)),
					focus_break_chunks[1],
				);
			}
			AppPage::Help => f.render_widget(
				app.page.help_ui((app.scroll.unwrap(), 0)),
				chunks[1],
			),
		}
	}

	fn help_ui(&self, scroll: (u16, u16)) -> impl Widget {
		Paragraph::new(vec![
			Spans::from(Span::styled(
				"Help",
				Style::default()
					.add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
			)),
			Spans::from(""),
			Spans::from("[h] - This menu"),
			Spans::from("[p] - Toggle pause"),
			Spans::from("[b] - Toggle break (while not paused)"),
		])
		.style(Style::default().fg(Color::White))
		.block(Block::default().borders(Borders::ALL))
		.scroll(scroll)
	}
}
pub fn focus_gauge(remaining: u16, initial: u16) -> impl Widget {
	let ratio = (initial - remaining) as f64 / initial as f64;

	CircularGauge::default()
		.block(
			Block::default()
				.borders(Borders::ALL)
				.style(Style::default().fg(Color::White))
				.title("Focus"),
		)
		.gauge_style(Style::default().fg(Color::LightRed))
		.label(Span::styled(
			format!(
				"{} ({:.1}%)",
				FormattedTime::from(remaining),
				ratio * 100.0
			),
			Style::default()
				.fg(Color::LightRed)
				.add_modifier(Modifier::ITALIC),
		))
		.ratio(ratio)
}
