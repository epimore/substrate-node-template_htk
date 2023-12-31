use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),
                   Some((1, frame_system::Pallet::<Test>::block_number())));
    });
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
        assert_noop!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
                   Error::<Test>::ProofAlreadyExist);
    });
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1),claim.clone()));
    });
}

#[test]
fn revoke_claim_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_noop!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
                   Error::<Test>::ClaimNotExist);
    });
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
        assert_noop!(PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
                   Error::<Test>::NotClaimOwner);
    });
}

#[test]
fn trans_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoeModule::trans_claim(RuntimeOrigin::signed(1),claim.clone(),2));
        assert_eq!(Proofs::<Test>::get(&claim),
                   Some((2, frame_system::Pallet::<Test>::block_number())));
    });
}

#[test]
fn trans_claim_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_noop!(PoeModule::trans_claim(RuntimeOrigin::signed(1),claim.clone(),2),
        Error::<Test>::ClaimNotExist);
    });
}

#[test]
fn trans_claim_failed_when_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
        assert_noop!(PoeModule::trans_claim(RuntimeOrigin::signed(2),claim.clone(),3),
        Error::<Test>::NotClaimOwner);
    });
}

#[test]
fn check_claim_owner_when_trans() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
        assert_eq!(Proofs::<Test>::get(&claim),
                   Some((1, frame_system::Pallet::<Test>::block_number())));
        PoeModule::trans_claim(RuntimeOrigin::signed(1), claim.clone(), 2).unwrap();
        assert_eq!(Proofs::<Test>::get(&claim),
                   Some((2, frame_system::Pallet::<Test>::block_number())));
    });
}