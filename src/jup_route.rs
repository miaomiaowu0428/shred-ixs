use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{borsh1, pubkey::Pubkey};
use solana_tx_parser::instruction;

instruction!(
    program_id: "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
    name: JupiterRouteIx,
    discriminator: [0xe5,0x17,0xcb,0x97,0x7a,0xe3,0xad,0x2a],
    accounts: {
        token_program:  { writable: false, signer: false },
        user_transfer_authority:  { writable: false, signer: false },
        user_source_token_account:  { writable: false, signer: false },
        user_destination_token_account:  { writable: false, signer: false },
        destination_token_account:  { writable: false, signer: false },
        destination_mint:  { writable: false, signer: false },
        platform_fee_account:  { writable: false, signer: false },
        event_authority:  { writable: false, signer: false },
        program:  { writable: false, signer: false },
        account_9:  { writable: false, signer: false },
    },
    data: {
        plans: Vec<RoutePlanStep>,
        in_amount: u64,
        out_amount: u64,
    },
);

// -------------------------- 基础枚举（无依赖） --------------------------

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub enum AccountsType {
    TransferHookA,
    TransferHookB,
    TransferHookReward,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub enum DefiTunaAccountsType {
    TransferHookA,
    TransferHookB,
    TransferHookInput,
    TransferHookIntermediate,
    TransferHookOutput,
    SupplementalTickArrays,
    SupplementalTickArraysOne,
    SupplementalTickArraysTwo,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub enum CandidateSwapResult {
    OutAmount(u64),
    ProgramError(u64),
}

// -------------------------- 基础结构体 --------------------------

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub struct FeeEvent {
    pub account: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub struct RemainingAccountsSlice {
    pub accounts_type: u8,
    pub length: u8,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct RemainingAccountsInfo {
    pub slices: Vec<RemainingAccountsSlice>,
}
impl Eq for RemainingAccountsInfo {}

impl PartialEq for RemainingAccountsInfo {
    fn eq(&self, other: &Self) -> bool {
        if self.slices.len() == other.slices.len() {
            for (s, o) in self.slices.iter().zip(other.slices.iter()) {
                if s != o {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub enum CandidateSwap {
    HumidiFi {
        swap_id: u64,
        is_base_to_quote: bool,
    },
    TesseraV {
        side: Side,
    },
}

// -------------------------- 核心枚举（依赖 Side 等） --------------------------

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub enum Swap {
    Saber,
    SaberAddDecimalsDeposit,
    SaberAddDecimalsWithdraw,
    TokenSwap,
    Sencha,
    Step,
    Cropper,
    Raydium,
    Crema {
        a_to_b: bool,
    },
    Lifinity,
    Mercurial,
    Cykura,
    Serum {
        side: Side,
    },
    MarinadeDeposit,
    MarinadeUnstake,
    Aldrin {
        side: Side,
    },
    AldrinV2 {
        side: Side,
    },
    Whirlpool {
        a_to_b: bool,
    },
    Invariant {
        x_to_y: bool,
    },
    Meteora,
    GooseFX,
    DeltaFi {
        stable: bool,
    },
    Balansol,
    MarcoPolo {
        x_to_y: bool,
    },
    Dradex {
        side: Side,
    },
    LifinityV2,
    RaydiumClmm,
    Openbook {
        side: Side,
    },
    Phoenix {
        side: Side,
    },
    Symmetry {
        from_token_id: u64,
        to_token_id: u64,
    },
    TokenSwapV2,
    HeliumTreasuryManagementRedeemV0,
    StakeDexStakeWrappedSol,
    StakeDexSwapViaStake {
        bridge_stake_seed: u32,
    },
    GooseFXV2,
    Perps,
    PerpsAddLiquidity,
    PerpsRemoveLiquidity,
    MeteoraDlmm,
    OpenBookV2 {
        side: Side,
    },
    RaydiumClmmV2,
    StakeDexPrefundWithdrawStakeAndDepositStake {
        bridge_stake_seed: u32,
    },
    Clone {
        pool_index: u8,
        quantity_is_input: bool,
        quantity_is_collateral: bool,
    },
    SanctumS {
        src_lst_value_calc_accs: u8,
        dst_lst_value_calc_accs: u8,
        src_lst_index: u32,
        dst_lst_index: u32,
    },
    SanctumSAddLiquidity {
        lst_value_calc_accs: u8,
        lst_index: u32,
    },
    SanctumSRemoveLiquidity {
        lst_value_calc_accs: u8,
        lst_index: u32,
    },
    RaydiumCP,
    WhirlpoolSwapV2 {
        a_to_b: bool,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    OneIntro,
    PumpWrappedBuy,
    PumpWrappedSell,
    PerpsV2,
    PerpsV2AddLiquidity,
    PerpsV2RemoveLiquidity,
    MoonshotWrappedBuy,
    MoonshotWrappedSell,
    StabbleStableSwap,
    StabbleWeightedSwap,
    Obric {
        x_to_y: bool,
    },
    FoxBuyFromEstimatedCost,
    FoxClaimPartial {
        is_y: bool,
    },
    SolFi {
        is_quote_to_base: bool,
    },
    SolayerDelegateNoInit,
    SolayerUndelegateNoInit,
    TokenMill {
        side: Side,
    },
    DaosFunBuy,
    DaosFunSell,
    ZeroFi,
    StakeDexWithdrawWrappedSol,
    VirtualsBuy,
    VirtualsSell,
    Perena {
        in_index: u8,
        out_index: u8,
    },
    PumpSwapBuy,
    PumpSwapSell,
    Gamma,
    MeteoraDlmmSwapV2 {
        remaining_accounts_info: RemainingAccountsInfo,
    },
    Woofi,
    MeteoraDammV2,
    MeteoraDynamicBondingCurveSwap,
    StabbleStableSwapV2,
    StabbleWeightedSwapV2,
    RaydiumLaunchlabBuy {
        share_fee_rate: u64,
    },
    RaydiumLaunchlabSell {
        share_fee_rate: u64,
    },
    BoopdotfunWrappedBuy,
    BoopdotfunWrappedSell,
    Plasma {
        side: Side,
    },
    GoonFi {
        is_bid: bool,
        blacklist_bump: u8,
    },
    HumidiFi {
        swap_id: u64,
        is_base_to_quote: bool,
    },
    MeteoraDynamicBondingCurveSwapWithRemainingAccounts,
    TesseraV {
        side: Side,
    },
    PumpWrappedBuyV2,
    PumpWrappedSellV2,
    PumpSwapBuyV2,
    PumpSwapSellV2,
    Heaven {
        a_to_b: bool,
    },
    SolFiV2 {
        is_quote_to_base: bool,
    },
    Aquifer,
    PumpWrappedBuyV3,
    PumpWrappedSellV3,
    PumpSwapBuyV3,
    PumpSwapSellV3,
    JupiterLendDeposit,
    JupiterLendRedeem,
    DefiTuna {
        a_to_b: bool,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    AlphaQ {
        a_to_b: bool,
    },
    RaydiumV2,
    SarosDlmm {
        swap_for_y: bool,
    },
    Futarchy {
        side: Side,
    },
    MeteoraDammV2WithRemainingAccounts,
    Obsidian,
    WhaleStreet {
        side: Side,
    },
    DynamicV1 {
        candidate_swaps: Vec<CandidateSwap>,
    },
    PumpWrappedBuyV4,
    PumpWrappedSellV4,
    CarrotIssue,
    CarrotRedeem,
    Manifest {
        side: Side,
    },
    BisonFi {
        a_to_b: bool,
    },
}

// -------------------------- 复合结构体（依赖 Swap 等） --------------------------

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct RoutePlanStep {
    pub swap: Swap,
    pub percent: u8,
    pub input_index: u8,
    pub output_index: u8,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct RoutePlanStepV2 {
    pub swap: Swap,
    pub bps: u16,
    pub input_index: u8,
    pub output_index: u8,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct SwapEvent {
    pub amm: Pubkey,
    pub input_mint: Pubkey,
    pub input_amount: u64,
    pub output_mint: Pubkey,
    pub output_amount: u64,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct SwapEventV2 {
    pub input_mint: Pubkey,
    pub input_amount: u64,
    pub output_mint: Pubkey,
    pub output_amount: u64,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct SwapsEvent {
    pub swap_events: Vec<SwapEventV2>,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct TokenLedger {
    pub token_account: Pubkey,
    pub amount: u64,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct BestSwapOutAmountViolation {
    pub expected_out_amount: u64,
    pub out_amount: u64,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct CandidateSwapResults {
    pub results: Vec<CandidateSwapResult>,
}

#[test]
fn test_parse() {
    let data_hex = "e517cb977ae3ad2a020000004b00000000640001076401022e852300000000005235ec05000000000807000000000000";
    let bytes = hex::decode(data_hex).unwrap();
    let res = JupiterRouteIx::from_full_data(&bytes);
    println!("res: {res:#?}")
}
