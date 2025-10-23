Finished `bench` profile [optimized] target(s) in 0.16s
Running `/home/shradhesh/Desktop/star_frame/target/release/deps/star_frame_spl-c20ab1d1ca0bbf9b --bench`

running 26 tests
test token::instructions::tests::test_amount_to_ui_amount ... ignored
test token::instructions::tests::test_approve ... ignored
test token::instructions::tests::test_approve_checked ... ignored
test token::instructions::tests::test_burn ... ignored
test token::instructions::tests::test_burn_checked ... ignored
test token::instructions::tests::test_close_account ... ignored
test token::instructions::tests::test_freeze_account ... ignored
test token::instructions::tests::test_get_account_data_size ... ignored
test token::instructions::tests::test_initialize_account ... ignored
test token::instructions::tests::test_initialize_account2 ... ignored
test token::instructions::tests::test_initialize_account3 ... ignored
test token::instructions::tests::test_initialize_immutable_owner ... ignored
test token::instructions::tests::test_initialize_mint ... ignored
test token::instructions::tests::test_initialize_mint2 ... ignored
test token::instructions::tests::test_initialize_multisig ... ignored
test token::instructions::tests::test_initialize_multisig2 ... ignored
test token::instructions::tests::test_mint_to ... ignored
test token::instructions::tests::test_mint_to_checked ... ignored
test token::instructions::tests::test_revoke ... ignored
test token::instructions::tests::test_set_authority ... ignored
test token::instructions::tests::test_sync_native ... ignored
test token::instructions::tests::test_thaw_account ... ignored
test token::instructions::tests::test_transfer ... ignored
test token::instructions::tests::test_transfer_checked ... ignored
test token::state::tests::test_account_accessors ... ignored
test token::state::tests::test_mint_accessors ... ignored

test result: ok. 0 passed; 0 failed; 26 ignored; 0 measured; 0 filtered out; finished in 0.00s

Running `/home/shradhesh/Desktop/star_frame/target/release/deps/compute_units-7c33a45aeedb6037 --bench`
Gnuplot not found, using plotters backend
initialize_mint_instruction
												time:   [21.390 ns 21.474 ns 21.556 ns]
												change: [-3.0826% -1.0419% +0.9303%] (p = 0.35 > 0.05)
												No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
	1 (1.00%) low mild
	4 (4.00%) high severe

initialize_account_instruction
												time:   [22.117 ns 22.242 ns 22.387 ns]
												change: [-4.2036% -1.8698% +0.4583%] (p = 0.12 > 0.05)
												No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
	3 (3.00%) high mild
	5 (5.00%) high severe

transfer_instruction    time:   [21.442 ns 21.520 ns 21.598 ns]
												change: [-4.3418% -2.2431% -0.2068%] (p = 0.03 < 0.05)
												Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
	1 (1.00%) low mild
	1 (1.00%) high mild
	2 (2.00%) high severe

token_account_deserialize_bytemuck
												time:   [191.47 ps 192.04 ps 192.57 ps]
												change: [-2.0395% -0.5305% +1.1045%] (p = 0.52 > 0.05)
												No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
	3 (3.00%) low severe
	2 (2.00%) low mild
	3 (3.00%) high severe

mint_account_deserialize_bytemuck
												time:   [191.29 ps 191.65 ps 192.04 ps]
												change: [+4.7319% +6.2445% +7.6838%] (p = 0.00 < 0.05)
												Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
	1 (1.00%) low severe
	2 (2.00%) high mild
	2 (2.00%) high severe

