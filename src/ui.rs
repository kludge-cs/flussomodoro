#![allow(clippy::borrow_interior_mutable_const)]

mod pages;
mod styles;

use ratatui::{
	layout::{Alignment, Constraint, Layout, Rect},
	text::{Line, Span},
	Frame,
};
use styles::*;
use tui_flusso_widgets::AlignedTabs;

use crate::app::App;

// AppPage::Help must ALWAYS be the first meta page
#[derive(Clone, Copy)]
pub enum AppPage {
	Main(pages::Main),
	// TODO: create pages for these
	EisenhowerMat,
	Kanban,
	Tasks,
	Help(pages::Help),
	Settings,
}

impl Default for AppPage {
	fn default() -> Self {
		AppPage::Main(pages::Main::default())
	}
}

impl AppPage {
	pub fn toggle_help(&self) -> Self {
		match self {
			AppPage::Help(_) => AppPage::default(),
			_ => AppPage::Help(pages::Help::default()),
		}
	}

	pub fn toggle_settings(&self) -> Self {
		match self {
			AppPage::Help(_) => AppPage::default(),
			_ => AppPage::Settings,
		}
	}

	fn idx_of(x: &Self) -> usize {
		// SAFETY: std::mem::discriminant will always return a valid
		// representation, and is therefore safe to transmute out of.
		unsafe { std::mem::transmute(std::mem::discriminant(x)) }
	}

	pub fn next_non_meta(&self) -> Self {
		match self {
			AppPage::Main(_) => AppPage::EisenhowerMat,
			AppPage::EisenhowerMat => AppPage::Kanban,
			AppPage::Kanban => AppPage::Tasks,
			AppPage::Tasks => AppPage::Main(pages::Main::default()),
			x => *x,
		}
	}

	pub fn prev_non_meta(&self) -> Self {
		match self {
			AppPage::Main(_) => AppPage::Tasks,
			AppPage::Tasks => AppPage::Kanban,
			AppPage::Kanban => AppPage::EisenhowerMat,
			AppPage::EisenhowerMat => AppPage::Main(pages::Main::default()),
			x => *x,
		}
	}

	pub fn scroll_by(&mut self, scroll: i16) {
		if let AppPage::Help(help) = self {
			help.scroll_by(scroll)
		}
	}
}

pub trait Page {
	fn render(&self, area: Rect, f: &mut Frame, app: &App);
}

impl Page for AppPage {
	fn render(&self, area: Rect, f: &mut Frame, app: &App) {
		let chunks = Layout::default()
			.constraints(vec![Constraint::Length(3), Constraint::Min(0)])
			.split(area);
		let titles: Vec<Line> =
			["Counter", "Eisenhower's Matrix", "Kanban", "Tasks"]
				.iter()
				.map(|t| Line::from(Span::styled(*t, *ELEM)))
				.collect();

		f.render_widget(
			AlignedTabs::new(titles)
				.block(block_std().title("Flussomodoro"))
				.highlight_style(*ELEM_SEL)
				.alignment(Alignment::Center)
				.select(AppPage::idx_of(self)),
			chunks[0],
		);

		match self {
			AppPage::Main(x) => x.render(chunks[1], f, app),
			AppPage::Help(x) => x.render(chunks[1], f, app),
			// TODO: render new pages
			_ => (),
		}
	}
}
