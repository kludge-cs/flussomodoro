table! {
	config (key) {
		key -> Text,
		data -> Binary,
	}
}

table! {
	tasks (id) {
		id -> Integer,
		name -> Text,
		focus_req -> Integer,
		focus_done -> Integer,
		box_cat -> Integer,
	}
}

allow_tables_to_appear_in_same_query!(config, tasks,);
