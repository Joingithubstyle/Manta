// Copyright 2019-2022 Manta Network.
// This file is part of pallet-manta-pay.
//
// pallet-manta-pay is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// pallet-manta-pay is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with pallet-manta-pay.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-15, STEPS: `1`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 128

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use core::marker::PhantomData;
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use frame_system::Config;

/// Weight functions needed for pallet_manta_pay.
pub trait WeightInfo {
	/// Returns the [`Weight`] of the [`Pallet::mint`] extrinsic.
	fn mint() -> Weight;

	/// Returns the [`Weight`] of the [`Pallet::private_transfer`] extrinsic.
	fn private_transfer() -> Weight;

	/// Returns the [`Weight`] of the [`Pallet::reclaim`] extrinsic.
	fn reclaim() -> Weight;
}

/// Concrete Weight Functions
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T> WeightInfo for SubstrateWeight<T>
where
	T: Config,
{
	/// ```text
	/// Storage: MantaPay Balances (r:1 w:1)
	/// Storage: MantaPay UtxoSet (r:1 w:1)
	/// Storage: MantaPay ShardTrees (r:1 w:1)
	/// Storage: MantaPay UtxoSetOutputs (r:0 w:1)
	/// Storage: MantaPay Shards (r:0 w:1)
	/// ```
	fn mint() -> Weight {
		(103_351_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}

	/// ```text
	/// Storage: MantaPay UtxoSetOutputs (r:2 w:2)
	/// Storage: MantaPay VoidNumberSet (r:2 w:2)
	/// Storage: MantaPay UtxoSet (r:2 w:2)
	/// Storage: MantaPay VoidNumberSetSize (r:1 w:1)
	/// Storage: MantaPay ShardTrees (r:2 w:2)
	/// Storage: MantaPay VoidNumberSetInsertionOrder (r:0 w:2)
	/// Storage: MantaPay Shards (r:0 w:2)
	/// ```
	fn private_transfer() -> Weight {
		(145_263_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(13 as Weight))
	}

	/// ```text
	/// Storage: MantaPay UtxoSetOutputs (r:2 w:1)
	/// Storage: MantaPay VoidNumberSet (r:2 w:2)
	/// Storage: MantaPay UtxoSet (r:1 w:1)
	/// Storage: MantaPay VoidNumberSetSize (r:1 w:1)
	/// Storage: MantaPay ShardTrees (r:1 w:1)
	/// Storage: MantaPay Balances (r:1 w:1)
	/// Storage: MantaPay VoidNumberSetInsertionOrder (r:0 w:2)
	/// Storage: MantaPay Shards (r:0 w:1)
	/// ```
	fn reclaim() -> Weight {
		(122_321_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
}
