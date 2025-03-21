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

#![cfg(test)]

use {
    common::*,
    cumulus_primitives_core::ParaId,
    dancebox_runtime::{
        AuthorNoting, AuthorityAssignment, AuthorityMapping, CollatorAssignment, CollatorSelection,
        Configuration, Proxy, ProxyType,
    },
    frame_support::{assert_ok, BoundedVec},
    nimbus_primitives::NIMBUS_KEY_ID,
    pallet_author_noting::ContainerChainBlockInfo,
    pallet_author_noting_runtime_api::runtime_decl_for_author_noting_api::AuthorNotingApi,
    pallet_collator_assignment_runtime_api::runtime_decl_for_collator_assignment_api::CollatorAssignmentApi,
    pallet_registrar_runtime_api::{
        runtime_decl_for_registrar_api::RegistrarApi, ContainerChainGenesisData,
    },
    parity_scale_codec::Encode,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_core::Get,
    sp_runtime::{
        traits::{BlakeTwo256, OpaqueKeys},
        DigestItem,
    },
    sp_std::vec,
    test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem},
    tp_consensus::runtime_decl_for_tanssi_authority_assignment_api::TanssiAuthorityAssignmentApiV1,
    tp_core::well_known_keys,
};

mod common;

const UNIT: Balance = 1_000_000_000_000_000_000;

#[test]
fn genesis_balances() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Balances::usable_balance(AccountId::from(ALICE)),
                210_000 * UNIT,
            );
            assert_eq!(
                Balances::usable_balance(AccountId::from(BOB)),
                100_000 * UNIT,
            );
        });
}

#[test]
fn genesis_para_registrar() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .build()
        .execute_with(|| {
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );
        });
}

#[test]
fn genesis_para_registrar_deregister() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );

            run_to_block(2);
            assert_ok!(Registrar::deregister(root_origin(), 1002.into()), ());

            // Pending
            assert_eq!(
                Registrar::pending_registered_para_ids(),
                vec![(2u32, BoundedVec::try_from(vec![1001u32.into()]).unwrap())]
            );

            run_to_session(1);
            assert_eq!(
                Registrar::pending_registered_para_ids(),
                vec![(2u32, BoundedVec::try_from(vec![1001u32.into()]).unwrap())]
            );
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );

            run_to_session(2);
            assert_eq!(Registrar::pending_registered_para_ids(), vec![]);
            assert_eq!(Registrar::registered_para_ids(), vec![1001.into()]);
        });
}

#[test]
fn genesis_para_registrar_runtime_api() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );
            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);

            run_to_block(2);
            assert_ok!(Registrar::deregister(root_origin(), 1002.into()), ());
            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);

            run_to_session(1);
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );
            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);

            run_to_session(2);
            assert_eq!(Registrar::registered_para_ids(), vec![1001.into()]);
            assert_eq!(Runtime::registered_paras(), vec![1001.into()]);
        });
}

#[test]
fn genesis_para_registrar_container_chain_genesis_data_runtime_api() {
    let genesis_data_1001 = empty_genesis_data();
    let genesis_data_1002 = ContainerChainGenesisData {
        storage: vec![(b"key".to_vec(), b"value".to_vec()).into()],
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: vec![],
        properties: Default::default(),
    };
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, genesis_data_1001.clone(), vec![]),
            (1002, genesis_data_1002.clone(), vec![]),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            assert_eq!(
                Registrar::registered_para_ids(),
                vec![1001.into(), 1002.into()]
            );
            assert_eq!(Runtime::registered_paras(), vec![1001.into(), 1002.into()]);

            assert_eq!(
                Runtime::genesis_data(1001.into()).as_ref(),
                Some(&genesis_data_1001)
            );
            assert_eq!(
                Runtime::genesis_data(1002.into()).as_ref(),
                Some(&genesis_data_1002)
            );
            assert_eq!(Runtime::genesis_data(1003.into()).as_ref(), None);

            // This API cannot be used to get the genesis data of the orchestrator chain,
            // with id 100
            // TODO: where is that 100 defined?
            assert_eq!(Runtime::genesis_data(100.into()).as_ref(), None);

            run_to_block(2);
            assert_ok!(Registrar::deregister(root_origin(), 1002.into()), ());

            // Deregistered container chains are deleted immediately
            // TODO: they should stay until session 2, just like the para id does
            assert_eq!(Runtime::genesis_data(1002.into()).as_ref(), None);

            let genesis_data_1003 = ContainerChainGenesisData {
                storage: vec![(b"key3".to_vec(), b"value3".to_vec()).into()],
                name: Default::default(),
                id: Default::default(),
                fork_id: Default::default(),
                extensions: vec![],
                properties: Default::default(),
            };
            assert_ok!(
                Registrar::register(
                    origin_of(ALICE.into()),
                    1003.into(),
                    genesis_data_1003.clone()
                ),
                ()
            );

            // Registered container chains are inserted immediately
            assert_eq!(
                Runtime::genesis_data(1003.into()).as_ref(),
                Some(&genesis_data_1003)
            );
        });
}

#[test]
fn test_author_collation_aura() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(5);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 4u64);
            // slot 4, alice
            assert!(current_author() == AccountId::from(ALICE));

            run_to_block(6);

            assert_eq!(current_slot(), 5u64);
            // slot 5, bob
            assert!(current_author() == AccountId::from(BOB));
        });
}

#[test]
fn test_author_collation_aura_change_of_authorities_on_session() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // We change invulnerables
            // We first need to set the keys
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_id.clone(),
                },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(CollatorSelection::set_invulnerables(
                root_origin(),
                vec![CHARLIE.into(), DAVE.into()]
            ));

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32);
            let author = get_orchestrator_current_author().unwrap();

            assert_eq!(current_author(), author);
            assert!(authorities() == vec![alice_id, bob_id]);

            // Invulnerables should have triggered on new session authorities change
            run_to_session(2u32);
            let author_after_changes = get_orchestrator_current_author().unwrap();

            assert_eq!(current_author(), author_after_changes);
            assert_eq!(authorities(), vec![charlie_id, dave_id]);
        });
}

#[test]
fn test_author_collation_aura_add_assigned_to_paras() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // We change invulnerables
            // We first need to set the keys
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys { nimbus: charlie_id },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys { nimbus: dave_id },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(CollatorSelection::set_invulnerables(
                root_origin(),
                vec![ALICE.into(), BOB.into(), CHARLIE.into(), DAVE.into()]
            ));

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32);
            let author = get_orchestrator_current_author().unwrap();

            assert_eq!(current_author(), author);
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);

            // Invulnerables should have triggered on new session authorities change
            // However charlie and dave shoudl have gone to one para (1001)
            run_to_session(2u32);
            assert_eq!(authorities(), vec![alice_id, bob_id]);
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
        });
}

#[test]
fn test_authors_without_paras() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Only Alice and Bob collate for our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());
            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            // It does not matter if we insert more collators, only two will be assigned
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);

            // Set moondance collators to min 2 max 5
            assert_ok!(
                Configuration::set_min_orchestrator_collators(root_origin(), 2),
                ()
            );
            assert_ok!(
                Configuration::set_max_orchestrator_collators(root_origin(), 5),
                ()
            );

            run_to_session(2);
            assert_eq!(authorities(), vec![alice_id, bob_id, charlie_id, dave_id]);
        });
}

#[test]
fn test_authors_paras_inserted_a_posteriori() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            assert_ok!(
                Registrar::register(origin_of(ALICE.into()), 1001.into(), empty_genesis_data()),
                ()
            );
            assert_ok!(
                Registrar::mark_valid_for_collating(root_origin(), 1001.into()),
                ()
            );
            assert_ok!(
                Registrar::register(origin_of(ALICE.into()), 1002.into(), empty_genesis_data()),
                ()
            );
            assert_ok!(
                Registrar::mark_valid_for_collating(root_origin(), 1002.into()),
                ()
            );

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);

            // Charlie and Dave should be assigne dot para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
        });
}

#[test]
fn test_authors_paras_inserted_a_posteriori_with_collators_already_assigned() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 5,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob collate in our chain
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());
            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id, charlie_id, dave_id]);

            assert_ok!(
                Registrar::register(origin_of(ALICE.into()), 1001.into(), empty_genesis_data()),
                ()
            );
            assert_ok!(
                Registrar::mark_valid_for_collating(root_origin(), 1001.into()),
                ()
            );

            // Assignment should happen after 2 sessions
            run_to_session(1u32);
            let assignment = CollatorAssignment::collator_container_chain();
            assert!(assignment.container_chains.is_empty());
            run_to_session(2u32);

            // Charlie and Dave are now assigned to para 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
            assert_eq!(
                assignment.orchestrator_chain,
                vec![ALICE.into(), BOB.into()]
            );
        });
}

#[test]
fn test_parachains_deregister_collators_re_assigned() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob are authorities
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            // Charlie and Dave to 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            assert_ok!(Registrar::deregister(root_origin(), 1001.into()), ());

            // Assignment should happen after 2 sessions
            run_to_session(1u32);

            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            run_to_session(2u32);

            // Charlie and Dave should be assigne dot para 1002 this time
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1002u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );
        });
}

#[test]
fn test_parachains_deregister_collators_config_change_reassigned() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));

            // Alice and Bob are authorities
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            assert_eq!(authorities(), vec![alice_id, bob_id]);

            // Set orchestrator collators to 1
            assert_ok!(
                Configuration::set_max_orchestrator_collators(root_origin(), 1),
                ()
            );

            // Set container chain collators to 3
            assert_ok!(
                Configuration::set_collators_per_container(root_origin(), 3),
                ()
            );

            // Charlie and Dave to 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            // Assignment should happen after 2 sessions
            run_to_session(1u32);

            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            run_to_session(2u32);

            // Charlie, Dave and BOB should be assigne dot para 1001 this time
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into(), BOB.into()]
            );

            assert_eq!(assignment.orchestrator_chain, vec![ALICE.into()]);
        });
}

#[test]
fn test_orchestrator_collators_with_non_sufficient_collators() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
        ])
        .with_collators(vec![(AccountId::from(ALICE), 210 * UNIT)])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(ALICE));

            // Alice and Bob are authorities
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());

            assert_eq!(authorities(), vec![alice_id]);
        });
}

#[test]
fn test_configuration_on_session_change() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(1);
            assert_eq!(Configuration::config().max_collators, 100);
            assert_eq!(Configuration::config().min_orchestrator_collators, 2);
            assert_eq!(Configuration::config().collators_per_container, 2);

            assert_ok!(Configuration::set_max_collators(root_origin(), 50), ());
            run_to_session(1u32);

            assert_ok!(
                Configuration::set_min_orchestrator_collators(root_origin(), 20),
                ()
            );
            assert_eq!(Configuration::config().max_collators, 100);
            assert_eq!(Configuration::config().min_orchestrator_collators, 2);
            assert_eq!(Configuration::config().collators_per_container, 2);

            run_to_session(2u32);
            assert_ok!(
                Configuration::set_collators_per_container(root_origin(), 10),
                ()
            );
            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().min_orchestrator_collators, 2);
            assert_eq!(Configuration::config().collators_per_container, 2);

            run_to_session(3u32);

            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().min_orchestrator_collators, 20);
            assert_eq!(Configuration::config().collators_per_container, 2);

            run_to_session(4u32);

            assert_eq!(Configuration::config().max_collators, 50);
            assert_eq!(Configuration::config().min_orchestrator_collators, 20);
            assert_eq!(Configuration::config().collators_per_container, 10);
        });
}

#[test]
fn test_author_collation_aura_add_assigned_to_paras_runtime_api() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            // Assert current slot gets updated
            assert_eq!(current_slot(), 1u64);
            assert!(current_author() == AccountId::from(BOB));
            assert_eq!(
                Runtime::parachain_collators(100.into()),
                Some(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(Runtime::parachain_collators(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::current_collator_parachain_assignment(ALICE.into()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(ALICE.into()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::current_collator_parachain_assignment(CHARLIE.into()),
                None
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(CHARLIE.into()),
                None
            );

            // We change invulnerables
            // We first need to set the keys
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys { nimbus: charlie_id },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys { nimbus: dave_id },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(CollatorSelection::set_invulnerables(
                root_origin(),
                vec![ALICE.into(), BOB.into(), CHARLIE.into(), DAVE.into()]
            ));

            // SESSION CHANGE. First session. it takes 2 sessions to see the change
            run_to_session(1u32);
            let author = get_orchestrator_current_author().unwrap();

            assert_eq!(current_author(), author);
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);
            assert_eq!(
                Runtime::parachain_collators(100.into()),
                Some(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(Runtime::parachain_collators(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::current_collator_parachain_assignment(CHARLIE.into()),
                None
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(CHARLIE.into()),
                Some(1001.into())
            );

            // Invulnerables should have triggered on new session authorities change
            // However charlie and dave shoudl have gone to one para (1001)
            run_to_session(2u32);
            assert_eq!(authorities(), vec![alice_id, bob_id]);
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            assert_eq!(
                Runtime::parachain_collators(100.into()),
                Some(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                Runtime::parachain_collators(1001.into()),
                Some(vec![CHARLIE.into(), DAVE.into()])
            );
            assert_eq!(
                Runtime::current_collator_parachain_assignment(CHARLIE.into()),
                Some(1001.into())
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(CHARLIE.into()),
                Some(1001.into())
            );

            // Remove BOB
            assert_ok!(CollatorSelection::set_invulnerables(
                root_origin(),
                vec![ALICE.into(), CHARLIE.into(), DAVE.into()]
            ));

            run_to_session(3u32);
            assert_eq!(
                Runtime::parachain_collators(100.into()),
                Some(vec![ALICE.into(), BOB.into()])
            );
            assert_eq!(
                Runtime::parachain_collators(1001.into()),
                Some(vec![CHARLIE.into(), DAVE.into()])
            );
            assert_eq!(
                Runtime::current_collator_parachain_assignment(BOB.into()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(BOB.into()),
                None
            );

            run_to_session(4u32);
            assert_eq!(
                Runtime::parachain_collators(100.into()),
                Some(vec![ALICE.into()])
            );
            assert_eq!(
                Runtime::parachain_collators(1001.into()),
                Some(vec![CHARLIE.into(), DAVE.into()])
            );
            assert_eq!(
                Runtime::current_collator_parachain_assignment(BOB.into()),
                None
            );
            assert_eq!(
                Runtime::future_collator_parachain_assignment(BOB.into()),
                None
            );
        });
}

#[test]
fn test_consensus_runtime_api() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);

            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(Runtime::para_id_authorities(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(charlie_id.clone()), None);
            assert_eq!(Runtime::check_para_id_assignment(dave_id.clone()), None);

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_id.clone(),
                },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(CollatorSelection::set_invulnerables(
                root_origin(),
                vec![ALICE.into(), BOB.into(), CHARLIE.into(), DAVE.into()]
            ));

            run_to_session(2u32);
            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(
                Runtime::para_id_authorities(1001.into()),
                Some(vec![charlie_id.clone(), dave_id.clone()])
            );
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(bob_id), Some(100.into()));
            assert_eq!(
                Runtime::check_para_id_assignment(charlie_id),
                Some(1001.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(dave_id),
                Some(1001.into())
            );
        });
}

#[test]
fn test_consensus_runtime_api_session_changes() {
    // The test shoul return always the assiignment on the next epoch
    // Meaning that we need to see before the session change block
    // if we can predict correctly
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);

            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(Runtime::para_id_authorities(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(charlie_id.clone()), None);
            assert_eq!(Runtime::check_para_id_assignment(dave_id.clone()), None);

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_id.clone(),
                },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(CollatorSelection::set_invulnerables(
                root_origin(),
                vec![ALICE.into(), BOB.into(), CHARLIE.into(), DAVE.into()]
            ));

            let session_two_edge = dancebox_runtime::Period::get() * 2;
            // Let's run just 2 blocks before the session 2 change first
            // Prediction should still be identical, as we are not in the
            // edge of a session change
            run_to_block(session_two_edge - 2);

            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(Runtime::para_id_authorities(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(charlie_id.clone()), None);
            assert_eq!(Runtime::check_para_id_assignment(dave_id.clone()), None);

            // Now we run to session edge -1. Here we should predict already with
            // authorities of the next block!
            run_to_block(session_two_edge - 1);
            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(
                Runtime::para_id_authorities(1001.into()),
                Some(vec![charlie_id.clone(), dave_id.clone()])
            );
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(bob_id), Some(100.into()));
            assert_eq!(
                Runtime::check_para_id_assignment(charlie_id),
                Some(1001.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(dave_id),
                Some(1001.into())
            );
        });
}

#[test]
fn test_consensus_runtime_api_next_session() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);

            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());

            let charlie_id = get_aura_id_from_seed(&AccountId::from(CHARLIE).to_string());
            let dave_id = get_aura_id_from_seed(&AccountId::from(DAVE).to_string());

            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(Runtime::para_id_authorities(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(charlie_id.clone()), None);
            assert_eq!(Runtime::check_para_id_assignment(dave_id.clone()), None);

            // In the next session the assignment will not change
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(charlie_id.clone()),
                None,
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(dave_id.clone()),
                None,
            );

            // Set CHARLIE and DAVE keys
            assert_ok!(Session::set_keys(
                origin_of(CHARLIE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: charlie_id.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(DAVE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: dave_id.clone(),
                },
                vec![]
            ));

            // Set new invulnerables
            assert_ok!(CollatorSelection::set_invulnerables(
                root_origin(),
                vec![ALICE.into(), BOB.into(), CHARLIE.into(), DAVE.into()]
            ));

            let session_two_edge = dancebox_runtime::Period::get() * 2;
            // Let's run just 2 blocks before the session 2 change first
            // Prediction should still be identical, as we are not in the
            // edge of a session change
            run_to_block(session_two_edge - 2);

            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(Runtime::para_id_authorities(1001.into()), Some(vec![]));
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(Runtime::check_para_id_assignment(charlie_id.clone()), None);
            assert_eq!(Runtime::check_para_id_assignment(dave_id.clone()), None);

            // But in the next session the assignment will change, so future api returns different value
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(charlie_id.clone()),
                Some(1001.into()),
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(dave_id.clone()),
                Some(1001.into()),
            );

            // Now we run to session edge -1. Here we should predict already with
            // authorities of the next block!
            run_to_block(session_two_edge - 1);
            assert_eq!(
                Runtime::para_id_authorities(100.into()),
                Some(vec![alice_id.clone(), bob_id.clone()])
            );
            assert_eq!(
                Runtime::para_id_authorities(1001.into()),
                Some(vec![charlie_id.clone(), dave_id.clone()])
            );
            assert_eq!(
                Runtime::check_para_id_assignment(alice_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(bob_id.clone()),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(charlie_id.clone()),
                Some(1001.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment(dave_id.clone()),
                Some(1001.into())
            );

            // check_para_id_assignment_next_session returns the same value as check_para_id_assignment
            // because we are on a session boundary
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(alice_id),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(bob_id),
                Some(100.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(charlie_id),
                Some(1001.into())
            );
            assert_eq!(
                Runtime::check_para_id_assignment_next_session(dave_id),
                Some(1001.into())
            );
        });
}

#[test]
fn test_author_noting_self_para_id_not_noting() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .build()
        .execute_with(|| {
            let mut sproof = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let self_para = parachain_info::Pallet::<Runtime>::get();
            let mut s = ParaHeaderSproofBuilderItem::default();
            s.para_id = self_para;
            s.author_id = HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: Default::default(),
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                },
            });
            sproof.items.push(s);

            set_author_noting_inherent_data(sproof);

            assert_eq!(AuthorNoting::latest_author(self_para), None);
        });
}

#[test]
fn test_author_noting_not_self_para() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .build()
        .execute_with(|| {
            let mut sproof = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let other_para: ParaId = 1001u32.into();

            // Charlie and Dave to 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            let mut s = ParaHeaderSproofBuilderItem::default();
            s.para_id = other_para;
            s.author_id = HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: 1,
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                },
            });
            sproof.items.push(s);

            set_author_noting_inherent_data(sproof);

            assert_eq!(
                AuthorNoting::latest_author(other_para),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: AccountId::from(DAVE)
                })
            );
        });
}

#[test]
fn test_author_noting_runtime_api() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
            (AccountId::from(CHARLIE), 100 * UNIT),
            (AccountId::from(DAVE), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .build()
        .execute_with(|| {
            let mut sproof = ParaHeaderSproofBuilder::default();
            let slot: u64 = 5;
            let other_para: ParaId = 1001u32.into();

            // Charlie and Dave to 1001
            let assignment = CollatorAssignment::collator_container_chain();
            assert_eq!(
                assignment.container_chains[&1001u32.into()],
                vec![CHARLIE.into(), DAVE.into()]
            );

            let mut s = ParaHeaderSproofBuilderItem::default();
            s.para_id = other_para;
            s.author_id = HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                parent_hash: Default::default(),
                number: 1,
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: sp_runtime::generic::Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                },
            });
            sproof.items.push(s);

            set_author_noting_inherent_data(sproof);

            assert_eq!(
                AuthorNoting::latest_author(other_para),
                Some(ContainerChainBlockInfo {
                    block_number: 1,
                    author: AccountId::from(DAVE)
                })
            );

            assert_eq!(
                Runtime::latest_author(other_para),
                Some(AccountId::from(DAVE))
            );
            assert_eq!(Runtime::latest_block_number(other_para), Some(1));
        });
}

#[test]
fn session_keys_key_type_id() {
    assert_eq!(
        dancebox_runtime::SessionKeys::key_ids(),
        vec![NIMBUS_KEY_ID]
    );
}

#[test]
fn test_session_keys_with_authority_mapping() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            let key_mapping_session_0 = AuthorityMapping::authority_id_mapping(0).unwrap();
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());
            let alice_id_2 = get_aura_id_from_seed("ALICE2");
            let bob_id_2 = get_aura_id_from_seed("BOB2");

            assert_eq!(key_mapping_session_0.len(), 2);
            assert_eq!(key_mapping_session_0.get(&alice_id), Some(&ALICE.into()));
            assert_eq!(key_mapping_session_0.get(&bob_id), Some(&BOB.into()));

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);

            // Change Alice and Bob keys to something different
            // for now lets change it to alice_2 and bob_2
            assert_ok!(Session::set_keys(
                origin_of(ALICE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: alice_id_2.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(BOB.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: bob_id_2.clone(),
                },
                vec![]
            ));

            run_to_session(1u32);
            let key_mapping_session_0 = AuthorityMapping::authority_id_mapping(0).unwrap();
            assert_eq!(key_mapping_session_0.len(), 2);
            assert_eq!(key_mapping_session_0.get(&alice_id), Some(&ALICE.into()));
            assert_eq!(key_mapping_session_0.get(&bob_id), Some(&BOB.into()));

            let key_mapping_session_1 = AuthorityMapping::authority_id_mapping(1).unwrap();
            assert_eq!(key_mapping_session_1.len(), 2);
            assert_eq!(key_mapping_session_1.get(&alice_id), Some(&ALICE.into()));
            assert_eq!(key_mapping_session_1.get(&bob_id), Some(&BOB.into()));

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);
            //

            run_to_session(2u32);
            assert!(AuthorityMapping::authority_id_mapping(0).is_none());

            let key_mapping_session_1 = AuthorityMapping::authority_id_mapping(1).unwrap();
            assert_eq!(key_mapping_session_1.len(), 2);
            assert_eq!(key_mapping_session_1.get(&alice_id), Some(&ALICE.into()));
            assert_eq!(key_mapping_session_1.get(&bob_id), Some(&BOB.into()));

            let key_mapping_session_2 = AuthorityMapping::authority_id_mapping(2).unwrap();
            assert_eq!(key_mapping_session_2.len(), 2);
            assert_eq!(key_mapping_session_2.get(&alice_id_2), Some(&ALICE.into()));
            assert_eq!(key_mapping_session_2.get(&bob_id_2), Some(&BOB.into()));

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id_2, bob_id_2]);
        });
}

#[test]
fn test_session_keys_with_authority_assignment() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_para_ids(vec![
            (1001, empty_genesis_data(), vec![]),
            (1002, empty_genesis_data(), vec![]),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);
            let alice_id = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
            let bob_id = get_aura_id_from_seed(&AccountId::from(BOB).to_string());
            let alice_id_2 = get_aura_id_from_seed("ALICE2");
            let bob_id_2 = get_aura_id_from_seed("BOB2");

            let key_mapping_session_0 = AuthorityAssignment::collator_container_chain(0).unwrap();
            assert_eq!(
                key_mapping_session_0.orchestrator_chain,
                vec![alice_id.clone(), bob_id.clone()],
            );
            assert_eq!(
                CollatorAssignment::collator_container_chain().orchestrator_chain,
                vec![AccountId::from(ALICE), AccountId::from(BOB)],
            );

            let key_mapping_session_1 = AuthorityAssignment::collator_container_chain(1).unwrap();
            assert_eq!(key_mapping_session_1, key_mapping_session_0,);
            let old_assignment_session_1 =
                CollatorAssignment::pending_collator_container_chain().unwrap();
            assert_eq!(
                old_assignment_session_1,
                CollatorAssignment::collator_container_chain(),
            );

            let key_mapping_session_2 = AuthorityAssignment::collator_container_chain(2);
            assert!(key_mapping_session_2.is_none());

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id.clone(), bob_id.clone()]);

            // Change Alice and Bob keys to something different
            // for now lets change it to alice_2 and bob_2
            assert_ok!(Session::set_keys(
                origin_of(ALICE.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: alice_id_2.clone(),
                },
                vec![]
            ));
            assert_ok!(Session::set_keys(
                origin_of(BOB.into()),
                dancebox_runtime::SessionKeys {
                    nimbus: bob_id_2.clone(),
                },
                vec![]
            ));

            run_to_session(1u32);
            let old_key_mapping_session_1 = key_mapping_session_1;

            // Session 0 got removed
            let key_mapping_session_0 = AuthorityAssignment::collator_container_chain(0);
            assert!(key_mapping_session_0.is_none());

            // The values at session 1 did not change
            let key_mapping_session_1 = AuthorityAssignment::collator_container_chain(1).unwrap();
            assert_eq!(key_mapping_session_1, old_key_mapping_session_1,);
            assert_eq!(
                CollatorAssignment::collator_container_chain(),
                old_assignment_session_1,
            );

            // Session 2 uses the new keys
            let key_mapping_session_2 = AuthorityAssignment::collator_container_chain(2).unwrap();
            assert_eq!(
                key_mapping_session_2.orchestrator_chain,
                vec![alice_id_2.clone(), bob_id_2.clone()],
            );
            assert_eq!(CollatorAssignment::pending_collator_container_chain(), None);

            let key_mapping_session_3 = AuthorityAssignment::collator_container_chain(3);
            assert!(key_mapping_session_3.is_none());

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id, bob_id]);

            run_to_session(2u32);

            // Session 1 got removed
            let key_mapping_session_1 = AuthorityAssignment::collator_container_chain(1);
            assert!(key_mapping_session_1.is_none());

            // Session 2 uses the new keys
            let key_mapping_session_2 = AuthorityAssignment::collator_container_chain(2).unwrap();
            assert_eq!(
                key_mapping_session_2.orchestrator_chain,
                vec![alice_id_2.clone(), bob_id_2.clone()],
            );
            assert_eq!(
                old_assignment_session_1,
                CollatorAssignment::collator_container_chain(),
            );

            // Session 3 uses the new keys
            let key_mapping_session_3 = AuthorityAssignment::collator_container_chain(3).unwrap();
            assert_eq!(
                key_mapping_session_3.orchestrator_chain,
                vec![alice_id_2.clone(), bob_id_2.clone()],
            );
            assert_eq!(CollatorAssignment::pending_collator_container_chain(), None);

            let key_mapping_session_4 = AuthorityAssignment::collator_container_chain(4);
            assert!(key_mapping_session_4.is_none());

            // Everything should match to aura
            assert_eq!(authorities(), vec![alice_id_2, bob_id_2]);
        });
}

fn call_transfer(
    dest: sp_runtime::MultiAddress<sp_runtime::AccountId32, ()>,
    value: u128,
) -> RuntimeCall {
    RuntimeCall::Balances(pallet_balances::Call::transfer_allow_death { dest, value })
}

#[test]
fn test_proxy_any() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);

            let delay = 0;
            assert_ok!(Proxy::add_proxy(
                origin_of(ALICE.into()),
                AccountId::from(BOB).into(),
                ProxyType::Any,
                delay
            ));

            let balance_before = System::account(AccountId::from(BOB)).data.free;
            let call = Box::new(call_transfer(AccountId::from(BOB).into(), 200_000));
            assert_ok!(Proxy::proxy(
                origin_of(BOB.into()),
                AccountId::from(ALICE).into(),
                None,
                call
            ));
            let balance_after = System::account(AccountId::from(BOB)).data.free;

            assert_eq!(balance_after, balance_before + 200_000);
        });
}

#[test]
fn test_proxy_non_transfer() {
    ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(ALICE), 210_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
            (AccountId::from(CHARLIE), 100_000 * UNIT),
            (AccountId::from(DAVE), 100_000 * UNIT),
        ])
        .with_collators(vec![
            (AccountId::from(ALICE), 210 * UNIT),
            (AccountId::from(BOB), 100 * UNIT),
        ])
        .with_config(pallet_configuration::HostConfiguration {
            max_collators: 100,
            min_orchestrator_collators: 2,
            max_orchestrator_collators: 2,
            collators_per_container: 2,
        })
        .build()
        .execute_with(|| {
            run_to_block(2);

            let delay = 0;
            assert_ok!(Proxy::add_proxy(
                origin_of(ALICE.into()),
                AccountId::from(BOB).into(),
                ProxyType::NonTransfer,
                delay
            ));

            let balance_before = System::account(AccountId::from(BOB)).data.free;
            let call = Box::new(call_transfer(AccountId::from(BOB).into(), 200_000));
            // The extrinsic succeeds but the call is filtered, so no transfer is actually done
            assert_ok!(Proxy::proxy(
                origin_of(BOB.into()),
                AccountId::from(ALICE).into(),
                None,
                call
            ));
            let balance_after = System::account(AccountId::from(BOB)).data.free;

            assert_eq!(balance_after, balance_before);
        });
}

#[test]
fn check_well_known_keys() {
    use frame_support::traits::PalletInfo;

    // Pallet is named "Paras" in Polkadot.
    assert_eq!(
        well_known_keys::PARAS_HEADS_INDEX,
        frame_support::storage::storage_prefix(b"Paras", b"Heads")
    );

    // Tanssi storage. Since we cannot access the storages themselves,
    // we test the pallet prefix matches and then compute manually the full prefix.
    assert_eq!(
        dancebox_runtime::PalletInfo::name::<AuthorityAssignment>(),
        Some("AuthorityAssignment")
    );
    assert_eq!(
        well_known_keys::AUTHORITY_ASSIGNMENT_PREFIX,
        frame_support::storage::storage_prefix(b"AuthorityAssignment", b"CollatorContainerChain")
    );

    assert_eq!(
        dancebox_runtime::PalletInfo::name::<Session>(),
        Some("Session")
    );
    assert_eq!(
        well_known_keys::SESSION_INDEX,
        frame_support::storage::storage_prefix(b"Session", b"CurrentIndex")
    );
}
