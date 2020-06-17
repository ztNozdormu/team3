// Tests to be written here

use crate::{Error, mock::*};
use super::*;
use frame_support::{assert_ok, assert_noop};

// test cases for create_claim
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::Signed(1),claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
    })
}
#[test]
fn create_claim_failed_when_claim_already_exist(){
    new_test_ext().execute
}
