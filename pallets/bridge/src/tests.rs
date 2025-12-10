use crate::{mock::*, Error, MessageStatus};
use frame::testing_prelude::*;

#[test]
fn it_works_for_sending_message() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic to send a message.
		let destination = b"parachain_2000".to_vec();
		let message = b"Hello from parachain!".to_vec();
		
		assert_ok!(BridgePallet::send_message(RuntimeOrigin::signed(1), destination.clone(), message.clone()));
		
		// Check that message status was stored.
		let message_id = crate::Pallet::<Test>::generate_message_id(&1, &message);
		assert!(MessageStatus::<Test>::contains_key((&1, message_id)));
	});
}

#[test]
fn it_works_for_cross_chain_call() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic to execute a cross-chain call.
		let destination = b"parachain_2001".to_vec();
		let call_data = b"execute_function".to_vec();
		
		assert_ok!(BridgePallet::execute_cross_chain_call(RuntimeOrigin::signed(1), destination.clone(), call_data.clone()));
		
		// Check that message status was stored.
		let message_id = crate::Pallet::<Test>::generate_message_id(&1, &call_data);
		assert!(MessageStatus::<Test>::contains_key((&1, message_id)));
	});
}
