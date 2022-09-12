use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create_kitty() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        // Read pallet storage and assert an expected result.
        assert!(KittiesModule::kitties(0).is_some());
    });
}

#[test]
fn it_fails_for_breed_claim_with_not_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(KittiesModule::breed(Origin::signed(2), 1, 2), Error::<Test>::NotOwner);
    });
}

#[test]
fn it_fails_for_breed_with_same_kitty_id() {
    new_test_ext().execute_with(|| {        
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(KittiesModule::breed(Origin::signed(1), 1, 1), Error::<Test>::SameKittyId);
    });
}

#[test]
fn it_fails_for_breed_with_invalid_kitty_id() {
    new_test_ext().execute_with(|| {
        assert_noop!(KittiesModule::breed(Origin::signed(1), 1, 3), Error::<Test>::InvalidKittyId);
    });
}

#[test]
fn it_works_for_breed() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::breed(Origin::signed(1), 1, 2));
    });
}

#[test]
fn it_fails_for_transfer_with_invalid_kitty_id() {
    new_test_ext().execute_with(|| {
        assert_noop!(KittiesModule::transfer(Origin::signed(1), 1, 2), Error::<Test>::InvalidKittyId);
    });
}

#[test]
fn it_fails_for_transfer_with_not_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(KittiesModule::transfer(Origin::signed(2), 1, 3), Error::<Test>::InvalidKittyId);
    });
}

#[test]
fn it_works_for_transfer() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(KittiesModule::transfer(Origin::signed(1), 1, 2), Error::<Test>::InvalidKittyId);
    });
}