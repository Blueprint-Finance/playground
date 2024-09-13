use anchor_lang::{prelude::*, system_program};
use anchor_lang::{InstructionData, ToAccountMetas};
use playground::{Counter, Position, Storage};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::Instruction,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

// #[macro_export]
macro_rules! anchor_processor {
    ($program:ident) => {{
        fn entry(
            program_id: &::solana_program::pubkey::Pubkey,
            accounts: &[::solana_program::account_info::AccountInfo],
            instruction_data: &[u8],
        ) -> ::solana_program::entrypoint::ProgramResult {
            let accounts = Box::leak(Box::new(accounts.to_vec()));

            $program::entry(program_id, accounts, instruction_data)
        }

        ::solana_program_test::processor!(entry)
    }};
}

#[tokio::test]
async fn test_create_increment_storage() {
    std::env::set_var("SBF_OUT_DIR", "../../target/deploy");
    let mut program = ProgramTest::new("playground", playground::ID, anchor_processor!(playground));
    let user = Keypair::new();
    program.add_account(
        user.pubkey(),
        Account {
            lamports: 1_000_000_000,
            data: vec![],
            owner: system_program::ID,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    let (storage, _) = Pubkey::find_program_address(&[payer.pubkey().as_ref()], &playground::ID);

    let ix = Instruction {
        program_id: playground::ID,
        accounts: playground::accounts::CreateStorage {
            storage,
            authority: payer.pubkey(),
            system_program: solana_program::system_program::id(),
        }
        .to_account_metas(Some(true)),
        data: playground::instruction::CreateStorage {}.data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let res = banks_client.process_transaction(tx).await;

    assert!(res.is_ok());

    let counter_res = banks_client.get_account(storage).await.unwrap().unwrap();

    let storage_state = Storage::try_deserialize(&mut counter_res.data.as_slice()).unwrap();
    assert_eq!(storage_state.space, 8);
    assert_eq!(storage_state.total_records, 0);

    // Assign storage
    let entries = vec![
        Position {
            address: Pubkey::new_unique(),
            value: 12345678,
        },
        Position {
            address: Pubkey::new_unique(),
            value: 12345678,
        },
        Position {
            address: Pubkey::new_unique(),
            value: 12345678,
        },
        Position {
            address: Pubkey::new_unique(),
            value: 12345678,
        },
        Position {
            address: Pubkey::new_unique(),
            value: 12345678,
        },
        Position {
            address: Pubkey::new_unique(),
            value: 12345678,
        },
    ];
    let ix = Instruction {
        program_id: playground::ID,
        accounts: playground::accounts::AssignStorage { storage }.to_account_metas(Some(true)),
        data: playground::instruction::AssignStorage {
            entries: entries.clone(),
        }
        .data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    let counter_res = banks_client.get_account(storage).await.unwrap().unwrap();

    let storage_state = Storage::try_deserialize(&mut counter_res.data.as_slice()).unwrap();
    assert_eq!(storage_state.total_records, 6);
    assert_eq!(storage_state.positions, entries);
}
