use solana_sdk::borsh1;
use solana_tx_parser::instruction;

// quote_amount_in 要扣税
instruction!(
    program_id: "b1oomGGqPKGD6errbyfbVMBuzSC8WtAAYo8MwNafWW1",
    name: BloomPumpBuyIx,
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
        base_mint:  { writable: false, signer: false },
        quote_mint: { writable: false, signer: false },
        others:     { writable: false, signer: false },
    },
    data: {
        unknown: [u8; 64],
        quote_amount_in: u64,
    },
);

#[test]
fn test_hex_to_u64() {
    let hex = "ee095f1500000000";
    let bytes = hex::decode(hex).unwrap();
    let amount = u64::from_le_bytes(bytes.try_into().unwrap());
    println!("amount: {}", amount);
}

#[test]
fn test_bloom_pumpswap_sell_parse() {
    let hex = "f1572738014d0e6300002f2b36000000000000000000000000001c010a000b010c0003000d050301070109000e010f01050010011101121866063d1201daebea10e4f42e5a040000ee095f1500000000";
    let bytes = hex::decode(hex).unwrap();
    let ix = BloomPumpBuyIx::from_full_data(&bytes);
    println!("BloomPumpBuyIx: {:#?}", ix);
}
