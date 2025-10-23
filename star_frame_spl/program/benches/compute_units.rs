// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use star_frame_spl::token::state::{MintAccountData, TokenAccountData, MintAccount, TokenAccount};
// use star_frame_spl::token::Token;
// use star_frame_spl::token::instructions::*;
// use star_frame::prelude::Pubkey;
// use star_frame::client::MakeInstruction;

// fn bench_initialize_mint_instruction(c: &mut Criterion) {
//     let mint = Pubkey::new_unique();
//     let mint_authority = Pubkey::new_unique();
//     let init = InitializeMint {
//         decimals: 6,
//         mint_authority,
//         freeze_authority: None,
//     };

//     c.bench_function("initialize_mint_instruction", |b| {
//         b.iter(|| {
//             let ix = Token::instruction(&init, InitializeMintClientAccounts { mint, rent: None }).unwrap();
//             black_box(ix);
//         })
//     });
// }

// fn bench_initialize_account_instruction(c: &mut Criterion) {
//     let account = Pubkey::new_unique();
//     let mint = Pubkey::new_unique();
//     let owner = Pubkey::new_unique();
//     let init = InitializeAccount;

//     c.bench_function("initialize_account_instruction", |b| {
//         b.iter(|| {
//             let ix = Token::instruction(&init, InitializeAccountClientAccounts { account, mint, owner, rent: None }).unwrap();
//             black_box(ix);
//         })
//     });
// }

// fn bench_transfer_instruction(c: &mut Criterion) {
//     let source = Pubkey::new_unique();
//     let destination = Pubkey::new_unique();
//     let owner = Pubkey::new_unique();
//     let amount = 500u64;
//     let instr = Transfer { amount };

//     c.bench_function("transfer_instruction", |b| {
//         b.iter(|| {
//             let ix = Token::instruction(&instr, TransferClientAccounts { source, destination, owner }).unwrap();
//             black_box(ix);
//         })
//     });
// }

// fn bench_token_account_deserialize(c: &mut Criterion) {
//     let bytes = vec![0u8; TokenAccount::LEN];
//     c.bench_function("token_account_deserialize_bytemuck", |b| {
//         b.iter(|| {
//             let _ = black_box(bytemuck::checked::try_from_bytes::<TokenAccountData>(&bytes).unwrap());
//         })
//     });
// }

// fn bench_mint_account_deserialize(c: &mut Criterion) {
//     let bytes = vec![0u8; MintAccount::LEN];
//     c.bench_function("mint_account_deserialize_bytemuck", |b| {
//         b.iter(|| {
//             let _ = black_box(bytemuck::checked::try_from_bytes::<MintAccountData>(&bytes).unwrap());
//         })
//     });
// }

// criterion_group!(
//     benches,
//     bench_initialize_mint_instruction,
//     bench_initialize_account_instruction,
//     bench_transfer_instruction,
//     bench_token_account_deserialize,
//     bench_mint_account_deserialize,
// );
// criterion_main!(benches);

// benches/compute_units.rs
// Bench the Token instruction compute-unit usage with Mollusk.
//
// Notes:
//  - Set SBF_OUT_DIR (see below) before running `cargo bench` so Mollusk can find the program .so.
//  - mollusk measures compute units by executing the instruction in a mini SVM.
//  - Keep your bytemuck / serialization micro-benchmarks in Criterion if you want CPU/time stats.

use mollusk_svm::Mollusk;
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_program::example_mocks::solana_sdk::system_program;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_account::{Account, AccountSharedData};
use star_frame::client::MakeInstruction;
use star_frame_spl::token::Token;
use star_frame_spl::token::instructions::{
    InitializeMint, InitializeMintClientAccounts, InitializeAccount, InitializeAccountClientAccounts,
    Transfer, TransferClientAccounts,
};
use star_frame_spl::token::state::{MintAccount, TokenAccount};

fn make_accounts_for_instruction(ix: &Instruction, special_sizes: &[(Pubkey, usize)]) -> Vec<(Pubkey, Account)> {
    // For each AccountMeta in the instruction, create a dummy AccountSharedData with a sensible size/owner.
    // If the pubkey is listed in `special_sizes`, use that length (e.g. MintAccount::LEN / TokenAccount::LEN).
    ix.accounts
        .iter()
        .map(|am| {
            let key = am.pubkey;
            let size = special_sizes
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, s)| *s)
                .unwrap_or(0usize);
            // If this looks like an on-chain token account (we gave it a known size) set owner to the program,
            // otherwise fall back to system_program as a reasonable default.
            let owner = if size > 0 { ix.program_id } else { system_program::id() };
            // Give it a healthy lamport balance and the requested data size
            let shared = AccountSharedData::new(1_000_000_000, size, &owner);
            (key, shared.into())
        })
        .collect()
}

fn bench_initialize_mint() {
    let mint = Pubkey::new_unique();
    let mint_authority = Pubkey::new_unique();

    let init = InitializeMint {
        decimals: 6,
        mint_authority,
        freeze_authority: None,
    };

    let ix = Token::instruction(&init, InitializeMintClientAccounts { mint, rent: None }).unwrap();

    // special sizes: mint -> MintAccount::LEN
    let accounts = make_accounts_for_instruction(&ix, &[(mint, MintAccount::LEN)]);
    let mollusk = Mollusk::new(&ix.program_id, "star_frame_spl"); // path or program name; see docs below

    MolluskComputeUnitBencher::new(mollusk)
        .bench(("initialize_mint_instruction", &ix, &accounts))
        .out_dir("../program/benches")
        .must_pass(false) // set true if you want test to panic on failure
        .execute();
}

fn bench_initialize_account() {
    let account = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let owner = Pubkey::new_unique();

    let init = InitializeAccount;

    let ix = Token::instruction(
        &init,
        InitializeAccountClientAccounts {
            account,
            mint,
            owner,
            rent: None,
        },
    )
    .unwrap();

    // special sizes: account -> TokenAccount::LEN, mint -> MintAccount::LEN
    let accounts = make_accounts_for_instruction(&ix, &[(account, TokenAccount::LEN), (mint, MintAccount::LEN)]);
    let mollusk = Mollusk::new(&ix.program_id, "star_frame_spl");

    MolluskComputeUnitBencher::new(mollusk)
        .bench(("initialize_account_instruction", &ix, &accounts))
        .out_dir("../program/benches")
        .must_pass(false)
        .execute();
}

fn bench_transfer() {
    let source = Pubkey::new_unique();
    let destination = Pubkey::new_unique();
    let owner = Pubkey::new_unique();
    let amount = 500u64;
    let instr = Transfer { amount };

    let ix = Token::instruction(&instr, TransferClientAccounts { source, destination, owner }).unwrap();

    // special sizes: source & destination are token accounts
    let accounts = make_accounts_for_instruction(&ix, &[(source, TokenAccount::LEN), (destination, TokenAccount::LEN)]);
    let mollusk = Mollusk::new(&ix.program_id, "star_frame_spl");

    MolluskComputeUnitBencher::new(mollusk)
        .bench(("transfer_instruction", &ix, &accounts))
        .out_dir("../program/benches")
        .must_pass(false)
        .execute();
}

fn main() {
    // Find the compiled BPF program; if you build with `cargo build-sbf` set SBF_OUT_DIR to ../target/deploy
    // std::env::set_var("SBF_OUT_DIR", "../target/deploy");
    // solana_logger::setup_with("");

    bench_initialize_mint();
    bench_initialize_account();
    bench_transfer();

}
