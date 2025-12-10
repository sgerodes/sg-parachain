//! Benchmarking setup for pallet-bridge

use super::*;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
	use super::*;
	#[cfg(test)]
	use crate::pallet::Pallet as BridgePallet;
	use frame_system::RawOrigin;

	#[benchmark]
	fn send_message() {
		let caller: T::AccountId = whitelisted_caller();
		let destination = b"parachain_2000".to_vec();
		let message = b"Hello from parachain!".to_vec();
		
		#[extrinsic_call]
		send_message(RawOrigin::Signed(caller), destination, message);

		// Verify message was sent by checking storage
		assert!(MessageStatus::<T>::iter().next().is_some());
	}

	#[benchmark]
	fn execute_cross_chain_call() {
		let caller: T::AccountId = whitelisted_caller();
		let destination = b"parachain_2001".to_vec();
		let call_data = b"execute_function".to_vec();
		
		#[extrinsic_call]
		execute_cross_chain_call(RawOrigin::Signed(caller), destination, call_data);

		// Verify cross-chain call was executed by checking storage
		assert!(MessageStatus::<T>::iter().next().is_some());
	}

	impl_benchmark_test_suite!(BridgePallet, crate::mock::new_test_ext(), crate::mock::Test);
}
