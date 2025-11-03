use anchor_lang::prelude::*;

declare_id!("1FxotckNZ5JzH3HvxMrsr32FRkuGu5APgCzNgg8zNjf");

#[program]
pub mod anchor_nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
