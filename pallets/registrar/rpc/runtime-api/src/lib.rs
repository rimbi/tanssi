// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

//! Runtime API for Registrar pallet

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::Get;
use scale_info::prelude::vec::Vec;
pub use tp_container_chain_genesis_data::ContainerChainGenesisData;

sp_api::decl_runtime_apis! {
    pub trait RegistrarApi<ParaId, MaxLengthTokenSymbol> where
        ParaId: parity_scale_codec::Codec,
        MaxLengthTokenSymbol: Get<u32>,
    {
        /// Return the registered para ids
        fn registered_paras() -> Vec<ParaId>;

        /// Fetch genesis data for this para id
        fn genesis_data(para_id: ParaId) -> Option<ContainerChainGenesisData<MaxLengthTokenSymbol>>;

        /// Fetch boot_nodes for this para id
        fn boot_nodes(para_id: ParaId) -> Vec<Vec<u8>>;
    }
}
