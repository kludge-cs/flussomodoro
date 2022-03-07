mod pages;
mod styles;

use styles::*;
use tui::{
	backend::Backend,
	layout::{Constraint, Layout, Rect},
	text::{Span, Spans},
	widgets::Tabs,
	Frame,
};

use crate::app::App;

#[derive(Clone, Copy)]
pub enum AppPage {
	Main(pages::Main),
	Help(pages::Help),
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

	pub fn scroll_by(&mut self, scroll: i16) {
		match self {
			AppPage::Help(help) => help.scroll_by(scroll),
			_ => (),
		}
	}
}

pub trait Page {
	fn render<B: Backend>(&self, area: Rect, f: &mut Frame<B>, app: &App);
}

impl Page for AppPage {
	fn render<B: Backend>(&self, area: Rect, f: &mut Frame<B>, app: &App) {
		let chunks = Layout::default()
			.constraints(vec![Constraint::Length(3), Constraint::Min(0)])
			.split(area);
		let titles = ["Counter"]
			.iter()
			.map(|t| Spans::from(Span::styled(*t, *ELEM)))
			.collect();
		f.render_widget(
			Tabs::new(titles)
				.block(block_std().title("Flussomodoro"))
				.highlight_style(*ELEM_SEL)
				.select(unsafe { std::mem::transmute(self) }),
			chunks[0],
		);

		match self {
			AppPage::Main(x) => x.render(chunks[1], f, app),
			AppPage::Help(x) => x.render(chunks[1], f, app),
		}
	}
}
