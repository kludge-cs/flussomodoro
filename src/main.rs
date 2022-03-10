use std::{
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

use clap::Parser;
use crossterm::event::Event;
use flussomodoro::{
	app::{App, AppOpts},
	terminal::Terminal,
};
use futures::{FutureExt, StreamExt};
use notify_rust::Notification;
use tokio::{io, time::interval};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
	let mut terminal = Terminal::with_stdout(std::io::stdout());
	let opts = AppOpts::parse();
	let mut app = App::with_opts(&opts);
	let mut interval = interval(Duration::from_secs(1));
	interval.tick().await; // first tick is immediate
	let mut event_stream = crossterm::event::EventStream::new().fuse();

	let stop_lock = AtomicBool::new(false);

	terminal.setup_backend()?;
	app.draw_with(&mut terminal);

	while !stop_lock.load(Ordering::SeqCst) {
		loop {
			tokio::select! {
				event = event_stream.select_next_some() => {
					if let Ok(Event::Key(key_event)) = event {
						let should_stop = app.handle_key_event(key_event);
						stop_lock.store(should_stop, Ordering::SeqCst);
						if should_stop {
							break;
						}
						app.draw_with(&mut terminal);
					}
				}
				_ = interval.tick().fuse() => {
					break;
				}
			}
		}
		app.counter.work(|msg| {
			if opts.notify {
				Notification::from(msg).show().unwrap();
			}
		});
		app.draw_with(&mut terminal);
	}

	Ok(())
}
