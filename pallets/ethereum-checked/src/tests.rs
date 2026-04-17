// This file is part of Astar.

// Copyright (C) Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

#![cfg(test)]

use super::*;
use mock::*;

use primitives::ethereum_checked::EthereumTxInput;
use ethereum::{ReceiptV3 as Receipt, TransactionV2 as Transaction};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::DispatchError;

fn bounded_input(data: &'static str) -> EthereumTxInput {
    EthereumTxInput::try_from(hex::decode(data).expect("invalid input hex"))
        .expect("input too large")
}

#[test]
fn transact_works() {
    ExtBuilder::default().build().execute_with(|| {
        let store_tx = CheckedEthereumTx {
            gas_limit: U256::from(1_000_000),
            target: contract_address(),
            value: U256::zero(),
            // Calling `store(3)`
            input: bounded_input(
                "6057361d0000000000000000000000000000000000000000000000000000000000000003",
            ),
            maybe_access_list: None,
        };
        assert_ok!(EthereumChecked::transact(
            RuntimeOrigin::signed(ALICE),
            store_tx.clone()
        ));
        assert_ok!(EthereumChecked::transact(
            RuntimeOrigin::signed(ALICE),
            store_tx
        ));

        // Get the count of pending transactions
        let pending_count = pallet_ethereum::Pending::<TestRuntime>::count();
        assert_eq!(pending_count, 2);

        // Check first transaction (index 0)
        if let Some((Transaction::EIP1559(ref t), _, Receipt::EIP1559(ref r))) = pallet_ethereum::Pending::<TestRuntime>::get(0) {
            // nonce 0, status code 1 (success)
            assert_eq!(t.nonce, U256::zero());
            assert_eq!(r.status_code, 1);
        } else {
            panic!("unexpected transaction type");
        }

        // Check second transaction (index 1)
        if let Some((Transaction::EIP1559(ref t), _, Receipt::EIP1559(ref r))) = pallet_ethereum::Pending::<TestRuntime>::get(1) {
            // nonce 1, status code 1 (success)
            assert_eq!(t.nonce, U256::one());
            assert_eq!(r.status_code, 1);
        } else {
            panic!("unexpected transaction type");
        }

        assert_eq!(Nonce::<TestRuntime>::get(), U256::from(2));

        let retrieve_tx = CheckedEthereumTx {
            gas_limit: U256::from(1_000_000),
            target: contract_address(),
            value: U256::zero(),
            // Calling `retrieve`
            input: bounded_input("2e64cec1"),
            maybe_access_list: None,
        };
        let (_, call_info) =
            EthereumChecked::xvm_transact(ALICE_H160, retrieve_tx).expect("failed to retrieve");
        assert_eq!(U256::from_big_endian(&(call_info.value)), 3.into());
    });
}

#[test]
fn origin_check_works() {
    ExtBuilder::default().build().execute_with(|| {
        let store_tx = CheckedEthereumTx {
            gas_limit: U256::from(1_000_000),
            target: contract_address(),
            value: U256::zero(),
            // Calling `store(3)`
            input: bounded_input(
                "6057361d0000000000000000000000000000000000000000000000000000000000000003",
            ),
            maybe_access_list: None,
        };
        // Now signed origin should work
        assert_ok!(EthereumChecked::transact(
            RuntimeOrigin::signed(ALICE),
            store_tx.clone()
        ));
        // Root and none should fail
        assert_noop!(
            EthereumChecked::transact(RuntimeOrigin::root(), store_tx.clone()),
            DispatchError::BadOrigin
        );
        assert_noop!(
            EthereumChecked::transact(RuntimeOrigin::none(), store_tx),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn no_hash_collision() {
    ExtBuilder::default().build().execute_with(|| {
        let store_tx = CheckedEthereumTx {
            gas_limit: U256::from(1_000_000),
            target: contract_address(),
            value: U256::zero(),
            // Calling `store(3)`
            input: bounded_input(
                "6057361d0000000000000000000000000000000000000000000000000000000000000003",
            ),
            maybe_access_list: None,
        };
        for _ in 0..5 {
            assert_ok!(EthereumChecked::transact(
                RuntimeOrigin::signed(ALICE),
                store_tx.clone()
            ));
            assert_ok!(<EthereumChecked as CheckedEthereumTransact>::xvm_transact(
                BOB_H160,
                store_tx.clone()
            ));
            assert_ok!(<EthereumChecked as CheckedEthereumTransact>::xvm_transact(
                CHARLIE_H160,
                store_tx.clone()
            ));
        }

        // Collect transaction hashes from all pending transactions
        let mut tx_hashes = Vec::new();
        let pending_count = pallet_ethereum::Pending::<TestRuntime>::count();
        for i in 0..pending_count {
            if let Some((tx, _, _)) = pallet_ethereum::Pending::<TestRuntime>::get(i) {
                tx_hashes.push(tx.hash());
            }
        }

        tx_hashes.dedup();
        assert_eq!(tx_hashes.len(), 15);
    });
}
