use anchor_lang::prelude::*;
use playground::*;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
#[tokio::test]
async fn test_initialize() {
    let mut validator = ProgramTest::default();
    let mut program = validator.add_program("playground", playground::ID, anchor_processor!(playground::entry));

    // let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // let counter = Keypair::new();
    // let user = Keypair::new();
    // program_test.add_account(s
    //     user.pubkey(),
    //     Account {
    //         lamports: 1_000_000_000,
    //         data: vec![],
    //         owner: system_program::id(),
    //         executable: false,
    //         rent_epoch: 0,
    //     },
    // );

    // let ix = solana_sdk::instruction::Instruction {
    //     program_id,
    //     accounts: Initialize {
    //         counter: counter,
    //         user: user.pubkey(),
    //         system_program: system_program::ID,
    //     }
    //     .to_account_metas(None),
    //     data: simple_counter::instruction::Initialize {}.data(),
    // };

    // let transaction = Transaction::new_signed_with_payer(
    //     &[ix],
    //     Some(&payer.pubkey()),
    //     &[&payer, &user, &counter],
    //     recent_blockhash,
    // );

    // banks_client.process_transaction(transaction).await.unwrap();

    // let counter_account = banks_client
    //     .get_account(counter.pubkey())
    //     .await
    //     .unwrap()
    //     .unwrap();

    // let counter_state = Counter::try_deserialize(&mut counter_account.data.as_ref()).unwrap();

    // assert_eq!(counter_state.count, 0);
}

// #[tokio::test]
// async fn test_increment() {
//     let program_id = id();
//     let mut program_test = ProgramTest::new("simple_counter", program_id, None);

//     let counter = Keypair::new();
//     let user = Keypair::new();
//     program_test.add_account(
//         user.pubkey(),
//         Account {
//             lamports: 1_000_000_000,
//             data: vec![],
//             owner: system_program::id(),
//             executable: false,
//             rent_epoch: 0,
//         },
//     );

//     let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

//     // Initialize the counter first
//     let init_ix = solana_sdk::instruction::Instruction {
//         program_id,
//         accounts: Initialize {
//             counter: counter.pubkey(),
//             user: user.pubkey(),
//             system_program: system_program::ID,
//         }
//         .to_account_metas(None),
//         data: simple_counter::instruction::Initialize {}.data(),
//     };

//     let init_tx = Transaction::new_signed_with_payer(
//         &[init_ix],
//         Some(&payer.pubkey()),
//         &[&payer, &user, &counter],
//         recent_blockhash,
//     );

//     banks_client.process_transaction(init_tx).await.unwrap();

//     // Increment the counter
//     let incr_ix = solana_sdk::instruction::Instruction {
//         program_id,
//         accounts: Increment {
//             counter: counter.pubkey(),
//         }
//         .to_account_metas(None),
//         data: simple_counter::instruction::Increment {}.data(),
//     };

//     let incr_tx = Transaction::new_signed_with_payer(
//         &[incr_ix],
//         Some(&payer.pubkey()),
//         &[&payer],
//         recent_blockhash,
//     );

//     banks_client.process_transaction(incr_tx).await.unwrap();

//     let counter_account = banks_client
//         .get_account(counter.pubkey())
//         .await
//         .unwrap()
//         .unwrap();

//     let counter_state = Counter::try_deserialize(&mut counter_account.data.as_ref()).unwrap();

//     assert_eq!(counter_state.count, 1);
// }
