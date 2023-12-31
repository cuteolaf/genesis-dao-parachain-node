
//! Autogenerated weights for pallet_dao_votes
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-07, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `blackbox`, CPU: `Intel(R) Core(TM) i5-6200U CPU @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/genesis-dao
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_dao_votes
// --extrinsic
// *
// --steps
// 2
// --repeat
// 1
// --output
// pallets/dao-votes/src/weights.rs
// --template
// benchmarking/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_dao_votes.
pub trait WeightInfo {
	fn create_proposal() -> Weight;
	fn fault_proposal() -> Weight;
	fn finalize_proposal(v: u32, ) -> Weight;
	fn vote() -> Weight;
	fn set_governance_majority_vote() -> Weight;
}

/// Weights for pallet_dao_votes using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: DaoCore Daos (r:1 w:0)
	/// Proof: DaoCore Daos (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: Votes Governances (r:1 w:0)
	/// Proof: Votes Governances (max_values: None, max_size: Some(39), added: 2514, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(93), added: 2568, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	/// Storage: Votes Proposals (r:0 w:1)
	/// Proof: Votes Proposals (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	fn create_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `549`
		//  Estimated: `14520`
		// Minimum execution time: 116_835_000 picoseconds.
		Weight::from_parts(116_835_000, 14520)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Votes Proposals (r:1 w:1)
	/// Proof: Votes Proposals (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: DaoCore Daos (r:1 w:0)
	/// Proof: DaoCore Daos (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	fn fault_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `381`
		//  Estimated: `7786`
		// Minimum execution time: 81_802_000 picoseconds.
		Weight::from_parts(81_802_000, 7786)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Votes Proposals (r:1 w:1)
	/// Proof: Votes Proposals (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: Votes Governances (r:1 w:0)
	/// Proof: Votes Governances (max_values: None, max_size: Some(39), added: 2514, mode: MaxEncodedLen)
	/// Storage: DaoCore Daos (r:1 w:0)
	/// Proof: DaoCore Daos (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: Votes Votes (r:1001 w:1000)
	/// Proof: Votes Votes (max_values: None, max_size: Some(58), added: 2533, mode: MaxEncodedLen)
	/// Storage: Assets AccountHistory (r:1000 w:0)
	/// Proof: Assets AccountHistory (max_values: None, max_size: Some(14400072), added: 14402547, mode: MaxEncodedLen)
	/// The range of component `v` is `[0, 1000]`.
	fn finalize_proposal(_v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `402 + v * (46 ±0)`
		//  Estimated: `1520193915`
		// Minimum execution time: 111_349_000 picoseconds.
		Weight::from_parts(16_923_371_000, 1520193915)
			.saturating_add(T::DbWeight::get().reads(2004_u64))
			.saturating_add(T::DbWeight::get().writes(1001_u64))
	}
	/// Storage: Votes Proposals (r:1 w:0)
	/// Proof: Votes Proposals (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: Votes Governances (r:1 w:0)
	/// Proof: Votes Governances (max_values: None, max_size: Some(39), added: 2514, mode: MaxEncodedLen)
	/// Storage: Votes Votes (r:0 w:1)
	/// Proof: Votes Votes (max_values: None, max_size: Some(58), added: 2533, mode: MaxEncodedLen)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `7397`
		// Minimum execution time: 54_944_000 picoseconds.
		Weight::from_parts(54_944_000, 7397)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DaoCore Daos (r:1 w:0)
	/// Proof: DaoCore Daos (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: Votes Governances (r:0 w:1)
	/// Proof: Votes Governances (max_values: None, max_size: Some(39), added: 2514, mode: MaxEncodedLen)
	fn set_governance_majority_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `133`
		//  Estimated: `3893`
		// Minimum execution time: 44_911_000 picoseconds.
		Weight::from_parts(44_911_000, 3893)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: DaoCore Daos (r:1 w:0)
	/// Proof: DaoCore Daos (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: Votes Governances (r:1 w:0)
	/// Proof: Votes Governances (max_values: None, max_size: Some(39), added: 2514, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(93), added: 2568, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	/// Storage: Votes Proposals (r:0 w:1)
	/// Proof: Votes Proposals (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	fn create_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `549`
		//  Estimated: `14520`
		// Minimum execution time: 116_835_000 picoseconds.
		Weight::from_parts(116_835_000, 14520)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Votes Proposals (r:1 w:1)
	/// Proof: Votes Proposals (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: DaoCore Daos (r:1 w:0)
	/// Proof: DaoCore Daos (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	fn fault_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `381`
		//  Estimated: `7786`
		// Minimum execution time: 81_802_000 picoseconds.
		Weight::from_parts(81_802_000, 7786)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Votes Proposals (r:1 w:1)
	/// Proof: Votes Proposals (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: Votes Governances (r:1 w:0)
	/// Proof: Votes Governances (max_values: None, max_size: Some(39), added: 2514, mode: MaxEncodedLen)
	/// Storage: DaoCore Daos (r:1 w:0)
	/// Proof: DaoCore Daos (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: Votes Votes (r:1001 w:1000)
	/// Proof: Votes Votes (max_values: None, max_size: Some(58), added: 2533, mode: MaxEncodedLen)
	/// Storage: Assets AccountHistory (r:1000 w:0)
	/// Proof: Assets AccountHistory (max_values: None, max_size: Some(14400072), added: 14402547, mode: MaxEncodedLen)
	/// The range of component `v` is `[0, 1000]`.
	fn finalize_proposal(_v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `402 + v * (46 ±0)`
		//  Estimated: `1520193915`
		// Minimum execution time: 111_349_000 picoseconds.
		Weight::from_parts(16_923_371_000, 1520193915)
			.saturating_add(RocksDbWeight::get().reads(2004_u64))
			.saturating_add(RocksDbWeight::get().writes(1001_u64))
	}
	/// Storage: Votes Proposals (r:1 w:0)
	/// Proof: Votes Proposals (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: Votes Governances (r:1 w:0)
	/// Proof: Votes Governances (max_values: None, max_size: Some(39), added: 2514, mode: MaxEncodedLen)
	/// Storage: Votes Votes (r:0 w:1)
	/// Proof: Votes Votes (max_values: None, max_size: Some(58), added: 2533, mode: MaxEncodedLen)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `7397`
		// Minimum execution time: 54_944_000 picoseconds.
		Weight::from_parts(54_944_000, 7397)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: DaoCore Daos (r:1 w:0)
	/// Proof: DaoCore Daos (max_values: None, max_size: Some(428), added: 2903, mode: MaxEncodedLen)
	/// Storage: Votes Governances (r:0 w:1)
	/// Proof: Votes Governances (max_values: None, max_size: Some(39), added: 2514, mode: MaxEncodedLen)
	fn set_governance_majority_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `133`
		//  Estimated: `3893`
		// Minimum execution time: 44_911_000 picoseconds.
		Weight::from_parts(44_911_000, 3893)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
