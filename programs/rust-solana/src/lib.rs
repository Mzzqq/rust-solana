use anchor_lang::prelude::*;

declare_id!("EwE8oxvjwaxEQBccRRJaYk7jov79j1h6NmC6P665UWM3");

#[program]
pub mod rust_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
