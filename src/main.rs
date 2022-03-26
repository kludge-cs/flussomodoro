use std::{
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

use clap::Parser;
use crossterm::event::Event;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{
	embed_migrations, EmbeddedMigrations, MigrationHarness,
};
use dirs::config_dir;
use flussomodoro::{
	app::{App, AppOpts},
	terminal::Terminal,
};
use futures::{FutureExt, StreamExt};
use notify_rust::Notification;
use tokio::{io, time::interval};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[tokio::main]
async fn main() -> Result<(), io::Error> {
	let dir = config_dir()
		.map(|mut path| {
			path.push("flussomodoro.db");
			path
		})
		.and_then(|path| path.to_str().map(|x| x.to_string()))
		.expect("Failed to acquire config directory");

	let mut conn = SqliteConnection::establish(&dir)
		.expect("Failed to connect to database");

	conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");

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
