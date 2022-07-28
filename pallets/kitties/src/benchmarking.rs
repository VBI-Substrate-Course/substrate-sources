//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {;
		let dna: Vec<u8> = b"abcabcabasdfsaljlsekjrjer"
		let caller: T::AccountId = whitelisted_caller();
	}: {
		create_kitty(RawOrigin::Signed(caller), dna)
	}
	verify {
		assert_eq!(KittyId::<T>::get(), 1);
	}

	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
 