CREATE TABLE IF NOT EXISTS config (
	key     TEXT PRIMARY KEY NOT NULL,
	data    BLOB NOT NULL
);
CREATE TABLE IF NOT EXISTS tasks (
	id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	name        VARCHAR(16) NOT NULL,
	focus_req   INTEGER NOT NULL DEFAULT 4, -- total required focus sessions
	focus_done  INTEGER NOT NULL DEFAULT 0, -- completed focus sessions
	box_cat     INTEGER NOT NULL DEFAULT 0, -- category in Eisenhower's box
	CHECK(focus_done >= 0 AND focus_req >= focus_done AND box_cat >= 0 AND box_cat < 4)
);
