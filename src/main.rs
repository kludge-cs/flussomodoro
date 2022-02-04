use std::{
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

use clap::Parser;
use crossterm::event::Event;
use flussomodoro::{app::App, counter::Counter};
use futures::{FutureExt, StreamExt};
use tokio::{io, time::interval};

// logic:
// every 5 seconds of focus, the user gets 1 second of break.
// that's 5 minutes of break per 25 minutes of focus.
// once 25 minutes has elapsed, a focus session is complete.
// once the user has completed 4 focus sessions, they get 15 minutes of break.
// if the user runs out of break, their current focus session resets.

#[derive(Parser)]
struct Opts {}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
	let mut app = App::new(Counter::new()).setup_term()?;
	let mut interval = interval(Duration::from_secs(1));
	interval.tick().await; // first tick is immediate
	let mut event_stream = crossterm::event::EventStream::new().fuse();

	let stop_lock = AtomicBool::new(false);
	app.start();

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
		app.counter.work();
		app.render();
	}

	Ok(())
}
