use solana_sdk::borsh1;
use solana_tx_parser::instruction;

// unknonw: f1572738014d0e630a010a00000000000000224e0f6d3100000027000303010d010e010f05040006000701100011010b010a010901120113010500140115011601171833e685a4017f83ad
// data: 224e0f6d310000000000000000000000
instruction!(
    program_id: "b1oomGGqPKGD6errbyfbVMBuzSC8WtAAYo8MwNafWW1",
    name: BloomPumpSwapSellIx,
    discriminator: [0xf1,0x57,0x27,0x38,0x01,0x4d,0x0e,0x63],
    accounts: {
        user:       { writable: false, signer: false },
        account_1:  { writable: false, signer: false },
        account_2:  { writable: false, signer: false },
        pool:       { writable: false, signer: false },
        account_4:  { writable: false, signer: false },
        account_5:  { writable: false, signer: false },
        account_6:  { writable: false, signer: false },
        account_7:  { writable: false, signer: false },
        account_8:  { writable: false, signer: false },
        account_9:  { writable: false, signer: false },
        account_10: { writable: false, signer: false },
        account_11: { writable: false, signer: false },
        account_12: { writable: false, signer: false },
        account_13: { writable: false, signer: false },
        base_mint:  { writable: false, signer: false },
        quote_mint: { writable: false, signer: false },
    },
    data: {
        unknown: [u8; 67],
        base_amount_in: u64,
    },
);

#[test]
fn test_bloom_pumpswap_sell_parse() {
    let hex = "f1572738014d0e630a010a00000000000000224e0f6d3100000027000303010d010e010f05040006000701100011010b010a010901120113010500140115011601171833e685a4017f83ad224e0f6d310000000000000000000000";
    let bytes = hex::decode(hex).unwrap();
    let ix = BloomPumpSwapSellIx::from_full_data(&bytes);
    println!("BloomPumpSwapSellIx: {:#?}", ix);
}
