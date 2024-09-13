use anchor_lang::{prelude::*, system_program};
use anchor_lang::{InstructionData, ToAccountMetas};
use playground::Counter;
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
async fn test_initialize() {
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

    let counter_account = Keypair::new();
    let counter_key = counter_account.pubkey();

    let ix = Instruction {
        program_id: playground::ID,
        accounts: playground::accounts::Initialize {
            counter: counter_key,
            user: payer.pubkey(),
            system_program: solana_program::system_program::id(),
        }
        .to_account_metas(Some(true)),
        data: playground::instruction::Initialize {}.data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &counter_account],
        recent_blockhash,
    );

    let res = banks_client.process_transaction(tx).await;

    assert!(res.is_ok());

    let counter_res = banks_client
        .get_account(counter_key)
        .await
        .unwrap()
        .unwrap();

    let counter_state = Counter::try_deserialize(&mut counter_res.data.as_slice()).unwrap();
    assert_eq!(counter_state.count, 0);
}

#[tokio::test]
async fn test_increment() {
    // std::env::set_var("SBF_OUT_DIR", "../../target/deploy/");
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

    let counter_account = Keypair::new();
    let counter_key = counter_account.pubkey();

    let ix1 = Instruction {
        program_id: playground::ID,
        accounts: playground::accounts::Initialize {
            counter: counter_key,
            user: payer.pubkey(),
            system_program: solana_program::system_program::id(),
        }
        .to_account_metas(Some(true)),
        data: playground::instruction::Initialize {}.data(),
    };

    let ix = Instruction {
        program_id: playground::ID,
        accounts: playground::accounts::Increment {
            counter: counter_key,
            // user: payer.pubkey(),
            // system_program: solana_program::system_program::id(),
        }
        .to_account_metas(Some(true)),
        data: playground::instruction::Increment {}.data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix1, ix],
        Some(&payer.pubkey()),
        &[&payer, &counter_account],
        recent_blockhash,
    );

    let res = banks_client.process_transaction(tx).await;

    assert!(res.is_ok());

    let counter_res = banks_client
        .get_account(counter_key)
        .await
        .unwrap()
        .unwrap();

    let counter_state = Counter::try_deserialize(&mut counter_res.data.as_slice()).unwrap();
    assert_eq!(counter_state.count, 1);
}
