pub mod actions;
pub use actions::*;

use anchor_lang::prelude::*;

declare_id!("FL6C2gLaDqSgeKmVrL7E75rqJhdnn9bvFp7GiQmy1Yrn");

const MAX_RESULT: u64 = u64::MAX;

const STATE_SEED: &[u8] = b"STATE";

#[program]
pub mod anchor_vrf_example {
    use super::*;

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn init_state(ctx: Context<InitState>, params: InitStateParams) -> ProgramResult {
        InitState::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn update_result(ctx: Context<UpdateResult>, params: UpdateResultParams) -> ProgramResult {
        UpdateResult::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn request_result(
        ctx: Context<RequestResult>,
        params: RequestResultParams,
    ) -> ProgramResult {
        RequestResult::actuate(&ctx, &params)
    }
}

#[account(zero_copy)]
pub struct VrfState {
    pub authority: Pubkey,
    pub max_result: u64,
    pub vrf_account: Pubkey,
    pub result_buffer: [u8; 32],
    pub result: u128,
    pub last_timestamp: i64,
}
impl Default for VrfState {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[error]
pub enum ErrorCode {
    #[msg("Not a valid Switchboard VRF account")]
    InvalidSwitchboardVrfAccount,
    #[msg("The max result must not exceed u64")]
    MaxResultExceedsMaximum,
    #[msg("Current round result is empty")]
    EmptyCurrentRoundResult,
    #[msg("Invalid authority account provided.")]
    InvalidAuthorityError,
}
