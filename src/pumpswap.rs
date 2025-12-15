use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::borsh1;
use solana_tx_parser::instruction;

instruction!(
    // 根据链上数据，Pump.fun AMM 程序 ID 应该是：pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
    program_id: "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA",
    name: PumpSwapSellIx,
    discriminator: [51,230,133,164,1,127,131,173],
    accounts: {
        pool: { writable: true, signer: false },
        user: { writable: true, signer: true },
        global_config: { writable: false, signer: false },
        base_mint: { writable: false, signer: false },
        quote_mint: { writable: false, signer: false },
        user_base_account: { writable: true, signer: false },
        user_quote_account: { writable: true, signer: false },
        pool_base_account: { writable: true, signer: false },
        pool_quote_account: { writable: true, signer: false },
        protocol_fee_recipient: { writable: false, signer: false },
        protocol_fee_account: { writable: true, signer: false },
        base_token_program: { writable: false, signer: false },
        quote_token_program: { writable: false, signer: false },
        system_program: { writable: false, signer: false },
        associated_token_program: { writable: false, signer: false },
        event_authority: { writable: false, signer: false },
        program: { writable: false, signer: false },
        creator_vault_ata: { writable: true, signer: false },
        creator_vault_authority: { writable: false, signer: false },
        fee_config: { writable: false, signer: false },
        fee_program: { writable: false, signer: false },
    },
    data: {
        base_amount_in: u64,
        min_quote_amoutn_out: u64,
    },
);
