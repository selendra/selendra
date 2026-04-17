use frame_support::{
    assert_noop, assert_ok,
    traits::{Currency, OnFinalize, OnInitialize, QueryPreimage},
    weights::Weight,
};
use pallet_collective::Instance1;
use parity_scale_codec::Encode;
use primitives::TOKEN;
use selendra_runtime::{
    AccountId, Balances, Council, Preimage, Runtime, RuntimeCall, RuntimeOrigin, 
    Sudo, System, TechnicalCommittee,
};
use sp_core::{sr25519, Pair};
use sp_io::TestExternalities;
use sp_runtime::{
    traits::{BlakeTwo256, Hash as HashTrait},
    AccountId32,
};

/// Create a test environment for integration testing
pub fn new_test_ext() -> TestExternalities {
    let accounts = get_test_accounts();
    let sudo_key = accounts[0].clone(); // Use first account as sudo key
    
    // Create a minimal test environment
    let mut ext = TestExternalities::new_empty();
    
    ext.execute_with(|| {
        System::set_block_number(1);
        
        // Set up initial balances for test accounts
        let initial_balance = 1000 * TOKEN;
        
        for account in &accounts {
            Balances::make_free_balance_be(account, initial_balance);
        }
        
        // Set up sudo key using the proper API
        assert_ok!(Sudo::set_key(RuntimeOrigin::root(), sp_runtime::MultiAddress::Id(sudo_key.clone())));
    });
    ext
}

/// Generate test accounts
pub fn get_test_accounts() -> Vec<AccountId> {
    let mut accounts = Vec::new();
    for i in 0..10 {
        let mut seed = [0u8; 32];
        seed[0] = i as u8;
        let pair = sr25519::Pair::from_seed(&seed);
        accounts.push(AccountId32::from(pair.public()));
    }
    accounts
}

/// Helper function to advance blocks
pub fn run_to_block(n: u32) {
    while System::block_number() < n {
        let current = System::block_number();
        System::on_finalize(current);
        System::set_block_number(current + 1);
        System::on_initialize(current + 1);
    }
}

#[cfg(test)]
mod council_member_tests {
    use super::*;

    #[test]
    fn test_add_council_member_via_root() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let new_member = accounts[1].clone();
            
            // Verify initial state - no council members
            assert_eq!(Council::members().len(), 0);
            
            // Add council member via root origin (simulating sudo/governance action)
            assert_ok!(Council::set_members(
                RuntimeOrigin::root(),
                vec![new_member.clone()],
                Some(new_member.clone()),
                0
            ));
            
            // Verify member was added
            let members = Council::members();
            assert_eq!(members.len(), 1);
            assert!(members.contains(&new_member));
            
            // Verify prime was set
            assert_eq!(Council::prime(), Some(new_member));
        });
    }

    #[test]
    fn test_add_multiple_council_members_via_root() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let new_members = vec![accounts[1].clone(), accounts[2].clone(), accounts[3].clone()];
            
            // Add multiple council members via root
            assert_ok!(Council::set_members(
                RuntimeOrigin::root(),
                new_members.clone(),
                Some(accounts[1].clone()),
                0
            ));
            
            // Verify all members were added
            let members = Council::members();
            assert_eq!(members.len(), 3);
            for member in &new_members {
                assert!(members.contains(member));
            }
            
            // Verify prime
            assert_eq!(Council::prime(), Some(accounts[1].clone()));
        });
    }

    #[test]
    fn test_add_council_member_via_existing_council_vote() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let sudo_account = accounts[0].clone();
            let initial_members = vec![accounts[1].clone(), accounts[2].clone(), accounts[3].clone()];
            let new_member = accounts[4].clone();
            
            // First set up initial council via sudo
            let setup_call = RuntimeCall::Council(
                pallet_collective::Call::set_members {
                    new_members: initial_members.clone(),
                    prime: Some(accounts[1].clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account),
                Box::new(setup_call)
            ));
            
            // Now add new member via council proposal
            let mut updated_members = initial_members.clone();
            updated_members.push(new_member.clone());
            
            let proposal_call = RuntimeCall::Council(
                pallet_collective::Call::set_members {
                    new_members: updated_members,
                    prime: Some(accounts[1].clone()),
                    old_count: 3,
                }
            );
            
            // Create proposal
            assert_ok!(Council::propose(
                RuntimeOrigin::signed(accounts[1].clone()),
                3, // threshold
                Box::new(proposal_call),
                1000 // length bound
            ));
            
            // Vote on proposal
            let proposal_hash = Council::proposals()[0];
            assert_ok!(Council::vote(
                RuntimeOrigin::signed(accounts[1].clone()),
                proposal_hash,
                0, // proposal index
                true
            ));
            assert_ok!(Council::vote(
                RuntimeOrigin::signed(accounts[2].clone()),
                proposal_hash,
                0,
                true
            ));
            assert_ok!(Council::vote(
                RuntimeOrigin::signed(accounts[3].clone()),
                proposal_hash,
                0,
                true
            ));
            
            // Close and execute proposal using max proposal weight
            let max_weight = Weight::from_parts(400_000_000_000, 50_000_000);
            assert_ok!(Council::close(
                RuntimeOrigin::signed(accounts[1].clone()),
                proposal_hash,
                0,
                max_weight,
                1000
            ));
            
            // Verify new member was added
            let members = Council::members();
            assert_eq!(members.len(), 4);
            assert!(members.contains(&new_member));
        });
    }

    #[test]
    fn test_query_council_state() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let sudo_account = accounts[0].clone();
            let council_members = vec![accounts[1].clone(), accounts[2].clone(), accounts[3].clone()];
            
            // Set up council
            let call = RuntimeCall::Council(
                pallet_collective::Call::set_members {
                    new_members: council_members.clone(),
                    prime: Some(accounts[1].clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account),
                Box::new(call)
            ));
            
            // Query council state
            let members = Council::members();
            let prime = Council::prime();
            let proposals = Council::proposals();
            let proposal_count = Council::proposal_count();
            
            // Verify state queries - council members are sorted internally
            let mut expected_members = council_members.clone();
            expected_members.sort();
            let mut actual_members = members.clone();
            actual_members.sort();
            assert_eq!(actual_members, expected_members);
            assert_eq!(prime, Some(accounts[1].clone()));
            assert_eq!(members.len(), 3);
            assert_eq!(proposals.len(), 0);
            assert_eq!(proposal_count, 0);
            
            // Test membership check using members() method
            let members = Council::members();
            assert!(members.contains(&accounts[1]));
            assert!(members.contains(&accounts[2]));
            assert!(members.contains(&accounts[3]));
            assert!(!members.contains(&accounts[4]));
        });
    }

    #[test]
    fn test_council_member_removal() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let sudo_account = accounts[0].clone();
            let initial_members = vec![accounts[1].clone(), accounts[2].clone(), accounts[3].clone()];
            
            // Set up initial council
            let setup_call = RuntimeCall::Council(
                pallet_collective::Call::set_members {
                    new_members: initial_members.clone(),
                    prime: Some(accounts[1].clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account.clone()),
                Box::new(setup_call)
            ));
            
            // Remove one member
            let updated_members = vec![accounts[1].clone(), accounts[2].clone()];
            let removal_call = RuntimeCall::Council(
                pallet_collective::Call::set_members {
                    new_members: updated_members.clone(),
                    prime: Some(accounts[1].clone()),
                    old_count: 3,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account),
                Box::new(removal_call)
            ));
            
            // Verify member was removed
            let members = Council::members();
            assert_eq!(members.len(), 2);
            assert!(members.contains(&accounts[1]));
            assert!(members.contains(&accounts[2]));
            assert!(!members.contains(&accounts[3]));
        });
    }
}

#[cfg(test)]
mod technical_committee_tests {
    use super::*;

    #[test]
    fn test_add_technical_committee_member_via_sudo() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let sudo_account = accounts[0].clone();
            let new_member = accounts[1].clone();
            
            // Verify initial state
            assert_eq!(TechnicalCommittee::members().len(), 0);
            
            // Add technical committee member via sudo
            let call = RuntimeCall::TechnicalCommittee(
                pallet_collective::Call::set_members {
                    new_members: vec![new_member.clone()],
                    prime: Some(new_member.clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account),
                Box::new(call)
            ));
            
            // Verify member was added
            let members = TechnicalCommittee::members();
            assert_eq!(members.len(), 1);
            assert!(members.contains(&new_member));
            assert_eq!(TechnicalCommittee::prime(), Some(new_member));
        });
    }

    #[test]
    fn test_add_multiple_tech_committee_members_via_sudo() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let sudo_account = accounts[0].clone();
            let new_members = vec![
                accounts[1].clone(), 
                accounts[2].clone(), 
                accounts[3].clone(),
                accounts[4].clone(),
            ];
            
            // Add multiple tech committee members via sudo
            let call = RuntimeCall::TechnicalCommittee(
                pallet_collective::Call::set_members {
                    new_members: new_members.clone(),
                    prime: Some(accounts[1].clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account),
                Box::new(call)
            ));
            
            // Verify all members were added
            let members = TechnicalCommittee::members();
            assert_eq!(members.len(), 4);
            for member in &new_members {
                assert!(members.contains(member));
            }
            
            assert_eq!(TechnicalCommittee::prime(), Some(accounts[1].clone()));
        });
    }

    #[test]
    fn test_add_tech_committee_member_via_council_vote() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let sudo_account = accounts[0].clone();
            
            // First set up council to have authority over tech committee
            let council_members = vec![accounts[1].clone(), accounts[2].clone(), accounts[3].clone()];
            let council_setup = RuntimeCall::Council(
                pallet_collective::Call::set_members {
                    new_members: council_members.clone(),
                    prime: Some(accounts[1].clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account.clone()),
                Box::new(council_setup)
            ));
            
            // Set up initial tech committee
            let initial_tech_members = vec![accounts[4].clone(), accounts[5].clone()];
            let tech_setup = RuntimeCall::TechnicalCommittee(
                pallet_collective::Call::set_members {
                    new_members: initial_tech_members.clone(),
                    prime: Some(accounts[4].clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account),
                Box::new(tech_setup)
            ));
            
            // Now add new tech committee member via council vote
            let new_tech_member = accounts[6].clone();
            let mut updated_tech_members = initial_tech_members.clone();
            updated_tech_members.push(new_tech_member.clone());
            
            let proposal_call = RuntimeCall::TechnicalCommittee(
                pallet_collective::Call::set_members {
                    new_members: updated_tech_members,
                    prime: Some(accounts[4].clone()),
                    old_count: 2,
                }
            );
            
            // Create council proposal to modify tech committee
            assert_ok!(Council::propose(
                RuntimeOrigin::signed(accounts[1].clone()),
                3, // threshold (3/5 council majority)
                Box::new(proposal_call),
                1000
            ));
            
            // Vote and execute
            let proposal_hash = Council::proposals()[0];
            assert_ok!(Council::vote(
                RuntimeOrigin::signed(accounts[1].clone()),
                proposal_hash,
                0,
                true
            ));
            assert_ok!(Council::vote(
                RuntimeOrigin::signed(accounts[2].clone()),
                proposal_hash,
                0,
                true
            ));
            assert_ok!(Council::vote(
                RuntimeOrigin::signed(accounts[3].clone()),
                proposal_hash,
                0,
                true
            ));
            
            let max_weight = Weight::from_parts(400_000_000_000, 50_000_000);
            assert_ok!(Council::close(
                RuntimeOrigin::signed(accounts[1].clone()),
                proposal_hash,
                0,
                max_weight,
                1000
            ));
            
            // Verify new tech committee member was added
            let tech_members = TechnicalCommittee::members();
            assert_eq!(tech_members.len(), 3);
            assert!(tech_members.contains(&new_tech_member));
        });
    }

    #[test]
    fn test_query_technical_committee_state() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let sudo_account = accounts[0].clone();
            let tech_members = vec![
                accounts[1].clone(), 
                accounts[2].clone(), 
                accounts[3].clone(),
                accounts[4].clone(),
            ];
            
            // Set up technical committee
            let call = RuntimeCall::TechnicalCommittee(
                pallet_collective::Call::set_members {
                    new_members: tech_members.clone(),
                    prime: Some(accounts[2].clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account),
                Box::new(call)
            ));
            
            // Query technical committee state
            let members = TechnicalCommittee::members();
            let prime = TechnicalCommittee::prime();
            let proposals = TechnicalCommittee::proposals();
            let proposal_count = TechnicalCommittee::proposal_count();
            
            // Verify state queries - technical committee members are sorted internally
            let mut expected_members = tech_members.clone();
            expected_members.sort();
            let mut actual_members = members.clone();
            actual_members.sort();
            assert_eq!(actual_members, expected_members);
            assert_eq!(prime, Some(accounts[2].clone()));
            assert_eq!(members.len(), 4);
            assert_eq!(proposals.len(), 0);
            assert_eq!(proposal_count, 0);
            
            // Test membership checks using contains
            for i in 1..5 {
                assert!(members.contains(&accounts[i]));
            }
            assert!(!members.contains(&accounts[5]));
            assert!(!members.contains(&accounts[0]));
        });
    }

    #[test]
    fn test_tech_committee_proposal_creation_and_voting() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let sudo_account = accounts[0].clone();
            let tech_members = vec![accounts[1].clone(), accounts[2].clone(), accounts[3].clone()];
            
            // Set up technical committee
            let setup_call = RuntimeCall::TechnicalCommittee(
                pallet_collective::Call::set_members {
                    new_members: tech_members.clone(),
                    prime: Some(accounts[1].clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account),
                Box::new(setup_call)
            ));
            
            // Create a technical committee proposal (e.g., system remark)
            let proposal_call = RuntimeCall::System(
                frame_system::Call::remark { remark: b"Technical Committee Test".to_vec() }
            );
            
            assert_ok!(TechnicalCommittee::propose(
                RuntimeOrigin::signed(accounts[1].clone()),
                2, // threshold
                Box::new(proposal_call),
                100
            ));
            
            // Verify proposal was created
            let proposals = TechnicalCommittee::proposals();
            assert_eq!(proposals.len(), 1);
            assert_eq!(TechnicalCommittee::proposal_count(), 1);
            
            // Vote on the proposal
            let proposal_hash = proposals[0];
            
            assert_ok!(TechnicalCommittee::vote(
                RuntimeOrigin::signed(accounts[1].clone()),
                proposal_hash,
                0,
                true
            ));
            
            assert_ok!(TechnicalCommittee::vote(
                RuntimeOrigin::signed(accounts[2].clone()),
                proposal_hash,
                0,
                true
            ));
            
            // Close and execute the proposal (use weight for remark call)
            let proposal_weight = Weight::from_parts(10_000_000, 0);
            assert_ok!(TechnicalCommittee::close(
                RuntimeOrigin::signed(accounts[1].clone()),
                proposal_hash,
                0,
                proposal_weight,
                100
            ));
            
            // Verify proposal was executed and removed
            assert_eq!(TechnicalCommittee::proposals().len(), 0);
        });
    }
}

#[cfg(test)]
mod democracy_integration_tests {
    use super::*;

    #[test]
    fn test_preimage_creation() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let account = accounts[1].clone();
            
            // Create a simple preimage (system remark)
            let simple_call = RuntimeCall::System(
                frame_system::Call::remark { remark: b"Democracy test".to_vec() }
            );
            let encoded_call = simple_call.encode();
            
            // Store the preimage
            assert_ok!(Preimage::note_preimage(
                RuntimeOrigin::signed(account),
                encoded_call.clone(),
            ));
            
            // Verify preimage was stored by checking if it exists
            let hash = <BlakeTwo256 as HashTrait>::hash(&encoded_call);
            // Use the public API to check preimage existence
            let preimage_exists = Preimage::len(&hash).is_some();
            assert!(preimage_exists);
        });
    }

    #[test]
    fn test_council_treasury_proposal() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let sudo_account = accounts[0].clone();
            
            // Set up council first
            let council_members = vec![accounts[1].clone(), accounts[2].clone(), accounts[3].clone()];
            let council_setup = RuntimeCall::Council(
                pallet_collective::Call::set_members {
                    new_members: council_members.clone(),
                    prime: Some(accounts[1].clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account),
                Box::new(council_setup)
            ));
            
            // Create treasury proposal via council
            let treasury_call = RuntimeCall::Treasury(
                pallet_treasury::Call::approve_proposal {
                    proposal_id: 0,
                }
            );
            
            assert_ok!(Council::propose(
                RuntimeOrigin::signed(accounts[1].clone()),
                3, // 3/5 threshold
                Box::new(treasury_call),
                1000
            ));
            
            // Verify proposal was created
            let proposals = Council::proposals();
            assert_eq!(proposals.len(), 1);
        });
    }
}

#[cfg(test)]
mod comprehensive_governance_tests {
    use super::*;

    #[test]
    fn test_full_governance_workflow() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let sudo_account = accounts[0].clone();
            
            // Step 1: Set up initial council via sudo
            let initial_council = vec![accounts[1].clone(), accounts[2].clone(), accounts[3].clone()];
            let council_setup = RuntimeCall::Council(
                pallet_collective::Call::set_members {
                    new_members: initial_council.clone(),
                    prime: Some(accounts[1].clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account.clone()),
                Box::new(council_setup)
            ));
            
            // Step 2: Council sets up technical committee
            let tech_members = vec![accounts[4].clone(), accounts[5].clone()];
            let tech_setup = RuntimeCall::TechnicalCommittee(
                pallet_collective::Call::set_members {
                    new_members: tech_members.clone(),
                    prime: Some(accounts[4].clone()),
                    old_count: 0,
                }
            );
            
            assert_ok!(Council::propose(
                RuntimeOrigin::signed(accounts[1].clone()),
                3,
                Box::new(tech_setup),
                1000
            ));
            
            // Vote and execute
            let proposal_hash = Council::proposals()[0];
            for member in &initial_council {
                assert_ok!(Council::vote(
                    RuntimeOrigin::signed(member.clone()),
                    proposal_hash,
                    0,
                    true
                ));
            }
            
            let max_weight = Weight::from_parts(400_000_000_000, 50_000_000);
            assert_ok!(Council::close(
                RuntimeOrigin::signed(accounts[1].clone()),
                proposal_hash,
                0,
                max_weight,
                1000
            ));
            
            // Step 3: Verify both committees are set up
            let mut expected_council = initial_council.clone();
            expected_council.sort();
            let mut actual_council = Council::members();
            actual_council.sort();
            assert_eq!(actual_council, expected_council);
            
            let mut expected_tech = tech_members.clone();
            expected_tech.sort();
            let mut actual_tech = TechnicalCommittee::members();
            actual_tech.sort();
            assert_eq!(actual_tech, expected_tech);
            
            // Step 4: Add new member to council via existing council
            let new_council_member = accounts[6].clone();
            let mut updated_council = initial_council.clone();
            updated_council.push(new_council_member.clone());
            
            let expand_council = RuntimeCall::Council(
                pallet_collective::Call::set_members {
                    new_members: updated_council.clone(),
                    prime: Some(accounts[1].clone()),
                    old_count: 3,
                }
            );
            
            assert_ok!(Council::propose(
                RuntimeOrigin::signed(accounts[1].clone()),
                3,
                Box::new(expand_council),
                1000
            ));
            
            let proposals = Council::proposals();
            assert!(!proposals.is_empty(), "Expected at least one proposal");
            let proposal_hash = proposals[0];
            
            // This is the second proposal in this test, so index should be 1
            let proposal_index = 1u32;
            
            for member in &initial_council {
                assert_ok!(Council::vote(
                    RuntimeOrigin::signed(member.clone()),
                    proposal_hash,
                    proposal_index,
                    true
                ));
            }
            
            let max_weight = Weight::from_parts(400_000_000_000, 50_000_000);
            assert_ok!(Council::close(
                RuntimeOrigin::signed(accounts[1].clone()),
                proposal_hash,
                proposal_index,
                max_weight,
                1000
            ));
            
            // Step 5: Verify final state
            let final_council = Council::members();
            assert_eq!(final_council.len(), 4);
            assert!(final_council.contains(&new_council_member));
            
            let final_tech = TechnicalCommittee::members();
            assert_eq!(final_tech.len(), 2);
            let mut expected_tech_sorted = tech_members.clone();
            expected_tech_sorted.sort();
            let mut actual_tech_sorted = final_tech.clone();
            actual_tech_sorted.sort();
            assert_eq!(actual_tech_sorted, expected_tech_sorted);
            
            // Step 6: Test comprehensive state queries
            assert_eq!(final_council.len(), 4);
            assert_eq!(final_tech.len(), 2);
            assert_eq!(Council::prime(), Some(accounts[1].clone()));
            assert_eq!(TechnicalCommittee::prime(), Some(accounts[4].clone()));
            
            // Test membership queries using contains
            assert!(final_council.contains(&new_council_member));
            assert!(!final_tech.contains(&new_council_member));
            assert!(final_tech.contains(&accounts[4]));
            assert!(!final_council.contains(&accounts[4]));
        });
    }

    #[test]
    fn test_governance_error_conditions() {
        new_test_ext().execute_with(|| {
            let accounts = get_test_accounts();
            let sudo_account = accounts[0].clone();
            
            // Test: Non-sudo account cannot directly set council members
            let non_sudo = accounts[1].clone();
            let call = RuntimeCall::Council(
                pallet_collective::Call::set_members {
                    new_members: vec![accounts[2].clone()],
                    prime: Some(accounts[2].clone()),
                    old_count: 0,
                }
            );
            
            assert_noop!(
                Sudo::sudo(RuntimeOrigin::signed(non_sudo), Box::new(call.clone())),
                pallet_sudo::Error::<Runtime>::RequireSudo
            );
            
            // Test: Non-member cannot create council proposal
            assert_noop!(
                Council::propose(
                    RuntimeOrigin::signed(accounts[1].clone()),
                    1,
                    Box::new(RuntimeCall::System(frame_system::Call::remark { remark: vec![] })),
                    100
                ),
                pallet_collective::Error::<Runtime, Instance1>::NotMember
            );
            
            // Set up council first
            assert_ok!(Sudo::sudo(
                RuntimeOrigin::signed(sudo_account),
                Box::new(call)
            ));
            
            // Test: Member can create proposal and non-member cannot vote on it
            assert_ok!(Council::propose(
                RuntimeOrigin::signed(accounts[2].clone()), // This is a council member
                2, // threshold higher than member count so it gets stored as proposal
                Box::new(RuntimeCall::System(frame_system::Call::remark { remark: vec![] })),
                100
            ));
            
            let proposals = Council::proposals();
            assert!(!proposals.is_empty(), "Expected at least one proposal");
            let proposal_hash = proposals[0];
            
            // For the first proposal in this test, the index should be 0
            let proposal_index = 0u32;
            
            assert_noop!(
                Council::vote(
                    RuntimeOrigin::signed(accounts[3].clone()), // not a member
                    proposal_hash,
                    proposal_index,
                    true
                ),
                pallet_collective::Error::<Runtime, Instance1>::NotMember
            );
        });
    }
}