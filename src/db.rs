use diesel::{delete, insert_into, prelude::*, replace_into};

use crate::schema::{config, tasks};

#[derive(Clone, Debug, Queryable)]
#[diesel(table_name = tasks)]
pub struct Task {
	pub id: i32,
	pub name: String,
	pub focus_req: i32,
	pub focus_done: i32,
	pub box_cat: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
	pub name: String,
	pub focus_req: i32,
	pub focus_done: i32,
	pub box_cat: i32,
}

pub fn create_task(
	conn: &mut SqliteConnection,
	name: String,
	focus_req: i32,
	focus_done: i32,
	box_cat: i32,
) -> QueryResult<Task> {
	use crate::schema::tasks::dsl::tasks;

	insert_into(tasks)
		.values(NewTask { name, focus_req, focus_done, box_cat })
		.returning(tasks::all_columns())
		.get_result(conn)
}

pub fn get_tasks(conn: &mut SqliteConnection) -> QueryResult<Vec<Task>> {
	use crate::schema::tasks::dsl::tasks;

	tasks.select(tasks::all_columns()).load(conn)
}

#[derive(Clone, Queryable, Insertable)]
#[diesel(table_name = config)]
pub struct ConfigItem {
	pub key: String,
	pub data: Vec<u8>,
}

pub trait ConfigValue {
	fn set_into(
		self,
		conn: &mut SqliteConnection,
		key: String,
	) -> QueryResult<()>;
	fn get_from(conn: &mut SqliteConnection, key: String) -> QueryResult<Self>
	where
		Self: Sized; // required to use in Result types
}

impl ConfigValue for u16 {
	fn set_into(
		self,
		conn: &mut SqliteConnection,
		key: String,
	) -> QueryResult<()> {
		set_config_data(conn, key, self.to_be_bytes().to_vec())
	}

	fn get_from(
		conn: &mut SqliteConnection,
		key: String,
	) -> QueryResult<Self> {
		get_config_data(conn, key)
			.map(|data| u16::from_be_bytes(data.try_into().unwrap()))
	}
}

impl ConfigValue for bool {
	fn set_into(
		self,
		conn: &mut SqliteConnection,
		key: String,
	) -> QueryResult<()> {
		set_config_data(conn, key, vec![self as u8])
	}

	fn get_from(
		conn: &mut SqliteConnection,
		key: String,
	) -> QueryResult<Self> {
		get_config_data(conn, key).map(|data| data[0] == 1)
	}
}

fn set_config_data(
	conn: &mut SqliteConnection,
	key: String,
	data: Vec<u8>,
) -> QueryResult<()> {
	use crate::schema::config::dsl::config;

	replace_into(config).values(ConfigItem { key, data }).execute(conn)?;
	Ok(())
}

pub fn unset_config_data(
	conn: &mut SqliteConnection,
	select_key: String,
) -> QueryResult<()> {
	use crate::schema::config::dsl::{config, key};

	delete(config.filter(key.eq(select_key))).execute(conn)?;
	Ok(())
}

fn get_config_data(
	conn: &mut SqliteConnection,
	select_key: String,
) -> QueryResult<Vec<u8>> {
	use crate::schema::config::dsl::*;

	config.select(data).filter(key.eq(select_key)).get_result(conn)
}
