use std::{
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

use clap::Parser;
use crossterm::event::Event;
use flussomodoro::{
	app::{App, AppMessage},
	counter::Counter,
	handle_key_event,
};
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

	app.handle_msg(AppMessage::Start);

	while !stop_lock.load(Ordering::SeqCst) {
		loop {
			tokio::select! {
				event = event_stream.select_next_some() => {
					if let Ok(Event::Key(key_event)) = event {
						let msg_pair = handle_key_event(key_event);
						stop_lock.store(msg_pair.0, Ordering::SeqCst);
						if msg_pair.0 {
							break;
						}
						if let Some(msg) = msg_pair.1 {
							app.handle_msg(msg)
						};
						app.handle_msg(AppMessage::Render);
					}
				}
				_ = interval.tick().fuse() => {
					break;
				}
			}
		}
		app.handle_msgs(vec![AppMessage::Work, AppMessage::Render]);
	}

	Ok(())
}
