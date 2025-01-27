// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_asset_rate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-03, STEPS: `50`, REPEAT: `2`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `cob`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/debug/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=2
// --pallet=pallet_asset_rate
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/kusama/src/weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_asset_rate`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_rate::WeightInfo for WeightInfo<T> {
	/// Storage: AssetRate ConversionRateToNative (r:1 w:1)
	/// Proof: AssetRate ConversionRateToNative (max_values: None, max_size: Some(1237), added: 3712, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `4702`
		// Minimum execution time: 53_000_000 picoseconds.
		Weight::from_parts(55_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4702))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: AssetRate ConversionRateToNative (r:1 w:1)
	/// Proof: AssetRate ConversionRateToNative (max_values: None, max_size: Some(1237), added: 3712, mode: MaxEncodedLen)
	fn update() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `110`
		//  Estimated: `4702`
		// Minimum execution time: 60_000_000 picoseconds.
		Weight::from_parts(60_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4702))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: AssetRate ConversionRateToNative (r:1 w:1)
	/// Proof: AssetRate ConversionRateToNative (max_values: None, max_size: Some(1237), added: 3712, mode: MaxEncodedLen)
	fn remove() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `110`
		//  Estimated: `4702`
		// Minimum execution time: 66_000_000 picoseconds.
		Weight::from_parts(74_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4702))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
