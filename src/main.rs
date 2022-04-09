use std::{
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

use clap::Parser;
use crossterm::event::Event;
use flussomodoro::app::{App, AppOpts};
use futures::{FutureExt, StreamExt};
use notify_rust::Notification;
use tokio::{io, time::interval};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
	let opts = AppOpts::parse();
	let mut app = App::with_opts(&opts).setup_term()?;
	let mut interval = interval(Duration::from_secs(1));
	interval.tick().await; // first tick is immediate
	let mut event_stream = crossterm::event::EventStream::new().fuse();

	let stop_lock = AtomicBool::new(false);
	app.render();

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
						app.render();
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
		app.render();
	}

	Ok(())
}
