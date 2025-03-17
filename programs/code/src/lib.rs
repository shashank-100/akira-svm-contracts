use anchor_lang::prelude::*;

declare_id!("2aGsKMY8eYZo8Xag7Vvv4juYbQ9M7xaZvECeCQ1et8jW");

#[program]
pub mod code {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
