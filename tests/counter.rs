use flussomodoro::counter::{Counter, CounterWorkState};

#[test]
pub fn counter_work_state_access() {
	// if init then work state should be none
	let mut test_counter = Counter::new();
	assert_eq!(test_counter.work_state().into_inner(), None);
	assert!(!test_counter.work_state().is_active());
	assert!(!test_counter.work_state().is_focusing());

	// if started then work state should be active
	test_counter.start();
	assert_eq!(test_counter.work_state().into_inner(), Some(true));
	assert!(test_counter.work_state().is_active());
	assert!(test_counter.work_state().is_focusing());

	// if reset then work state should be none
	test_counter.reset();
	assert_eq!(test_counter.work_state().into_inner(), None);
	assert!(!test_counter.work_state().is_active());
	assert!(!test_counter.work_state().is_focusing());
}

#[test]
pub fn work_state_impl() {
	let mut test_work_state = CounterWorkState::from(None);

	// if none and break toggled then work state should be none
	test_work_state.toggle_break();
	assert_eq!(test_work_state.into_inner(), None);
	assert!(!test_work_state.is_active());
	assert!(!test_work_state.is_focusing());

	// if none and active toggled then work state should be active
	test_work_state.toggle_active();
	assert_eq!(test_work_state.into_inner(), Some(true));
	assert!(test_work_state.is_active());
	assert!(test_work_state.is_focusing());

	// if active and break toggled then work state should be break
	test_work_state.toggle_break();
	assert_eq!(test_work_state.into_inner(), Some(false));
	assert!(test_work_state.is_active());
	assert!(!test_work_state.is_focusing());

	// if break and break toggled then work state should be active
	test_work_state.toggle_break();
	assert_eq!(test_work_state.into_inner(), Some(true));

	// if focus and active toggled then work state should be none
	assert_eq!(test_work_state.into_inner(), Some(true));
	test_work_state.toggle_active();
	assert_eq!(test_work_state.into_inner(), None);

	// if break and active toggled then work state should be none
	test_work_state.toggle_active();
	test_work_state.toggle_break();
	assert_eq!(test_work_state.into_inner(), Some(false));
	test_work_state.toggle_active();
	assert_eq!(test_work_state.into_inner(), None);
}

#[test]
pub fn counter_work_state_functionality() {
	let mut test_counter = Counter::new();

	// if not active then work should do nothing
	(0..5).for_each(|_| test_counter.work());
	assert_eq!(test_counter.focus_time(), 25 * 60);

	// if active then work should work
	test_counter.work_state_mut().toggle_active();
	(0..5).for_each(|_| test_counter.work());
	assert_eq!(test_counter.focus_time(), 25 * 60 - 5);
	assert_eq!(test_counter.break_time(), 1);

	// if active then work should continue to work linearly
	(0..20).for_each(|_| test_counter.work());
	assert_eq!(test_counter.focus_time(), 25 * 60 - 25);
	assert_eq!(test_counter.break_time(), 5);

	// if break then work should decrement break and retain focus
	test_counter.work_state_mut().toggle_break();
	test_counter.work();
	assert_eq!(test_counter.focus_time(), 25 * 60 - 25);
	assert_eq!(test_counter.break_time(), 4);

	// if break empty then work should reset counter
	(0..5).for_each(|_| test_counter.work());
	assert_eq!(test_counter.work_state().into_inner(), None);
	assert_eq!(test_counter.focus_time(), 25 * 60);
	assert_eq!(test_counter.break_time(), 0);
}

#[test]
pub fn session_rollover() {
	let mut test_counter = Counter::new();
	assert_eq!(test_counter.pom(), 1);

	// if active and focus session finished then move to next session
	test_counter.start();
	(0..25 * 60).for_each(|_| test_counter.work());
	assert_eq!(test_counter.work_state().into_inner(), None);
	assert_eq!(test_counter.focus_time(), 25 * 60);
	assert_eq!(test_counter.break_time(), 25 * 60 / 5);
	assert_eq!(test_counter.pom(), 2);

	// if active and 4 focus sessions finished then clover completed
	// if clover completed give break bonus
	test_counter.start();
	(0..25 * 60).for_each(|_| test_counter.work()); // 2nd
	test_counter.start();
	(0..25 * 60).for_each(|_| test_counter.work()); // 3rd
	test_counter.start();
	(0..25 * 60).for_each(|_| test_counter.work()); // 4th
	assert_eq!(test_counter.work_state().into_inner(), None);
	assert_eq!(test_counter.focus_time(), 25 * 60);
	assert_eq!(test_counter.break_time(), 25 * 60 / 5 * 4 + 15 * 60); // 4 sessions of break + clover bonus
	assert_eq!(test_counter.pom(), 1); // back to 1st pom of a clover
}
