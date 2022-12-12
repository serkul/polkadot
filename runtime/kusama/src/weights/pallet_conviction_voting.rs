// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_conviction_voting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_conviction_voting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_conviction_voting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_conviction_voting::WeightInfo for WeightInfo<T> {
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn vote_new() -> Weight {
		// Minimum execution time: 129_807 nanoseconds.
		Weight::from_ref_time(133_007_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn vote_existing() -> Weight {
		// Minimum execution time: 154_191 nanoseconds.
		Weight::from_ref_time(156_774_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn remove_vote() -> Weight {
		// Minimum execution time: 128_599 nanoseconds.
		Weight::from_ref_time(131_816_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	fn remove_other_vote() -> Weight {
		// Minimum execution time: 71_872 nanoseconds.
		Weight::from_ref_time(73_800_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ConvictionVoting VotingFor (r:2 w:2)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:10 w:10)
	// Storage: Scheduler Agenda (r:2 w:2)
	/// The range of component `r` is `[0, 512]`.
	fn delegate(r: u32, ) -> Weight {
		// Minimum execution time: 77_152 nanoseconds.
		Weight::from_ref_time(706_169_111 as u64)
			// Standard Error: 59_459
			.saturating_add(Weight::from_ref_time(26_675_761 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: ConvictionVoting VotingFor (r:2 w:2)
	// Storage: Referenda ReferendumInfoFor (r:10 w:10)
	// Storage: Scheduler Agenda (r:2 w:2)
	/// The range of component `r` is `[0, 512]`.
	fn undelegate(r: u32, ) -> Weight {
		// Minimum execution time: 57_429 nanoseconds.
		Weight::from_ref_time(678_011_885 as u64)
			// Standard Error: 59_201
			.saturating_add(Weight::from_ref_time(26_729_943 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn unlock() -> Weight {
		// Minimum execution time: 91_278 nanoseconds.
		Weight::from_ref_time(93_505_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}