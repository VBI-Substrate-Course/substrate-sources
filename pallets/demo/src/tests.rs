use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_can_create_student() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Demo::create_student(Origin::signed(1), b"mystudent".to_vec(), 21));
	});
}

fn student_too_yound() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_noop!(Demo::create_student(Origin::signed(1), b"mystudent".to_vec(), 18), Error::<Test>::TooYoung);
	});
}
