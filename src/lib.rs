use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("5mvfdTGjTxhsVj71Hd3CFn5DqRJhKSU9MA6dJ9KkohY");

#[program]
mod escrow_program {
    use super::*;

    pub fn process_payment(ctx: Context<ProcessPayment>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let fee_amount = amount.checked_mul(20).unwrap().checked_div(100).unwrap();
        let escrow_amount = amount.checked_sub(fee_amount).unwrap();

        msg!("Fee amount (20%): {}", fee_amount);
        msg!("Escrow amount (80%): {}", escrow_amount);

        // Transfer fee to fee collector
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.fee_collector.to_account_info(),
                },
            ),
            fee_amount,
        )?;
        msg!("Fee transfer complete");

        // Transfer escrow amount to the PDA (escrow account)
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.escrow_account.to_account_info(),
                },
            ),
            escrow_amount,
        )?;
        msg!("Escrow transfer complete");

        Ok(())
    }

    pub fn release_funds(ctx: Context<ReleaseFunds>, amount: u64) -> Result<()> {
        let escrow_lamports = ctx.accounts.escrow_account.to_account_info().lamports();
        let rent = Rent::get()?;
        let rent_exempt_lamports = rent.minimum_balance(8);

        let available_lamports = escrow_lamports.saturating_sub(rent_exempt_lamports);
        require!(available_lamports >= amount, ErrorCode::InsufficientFunds);

        **ctx.accounts.escrow_account.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.recipient.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProcessPayment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = [b"escrow", user.key().as_ref()], bump)]
    pub escrow_account: AccountInfo<'info>,

    #[account(mut, address = "2p9SEZ3sw9uWvPfXj3gwyStVLQyzPJh5yYETWAqcdCss".parse::<Pubkey>().unwrap())]
    pub fee_collector: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseFunds<'info> {
    #[account(mut, seeds = [b"escrow", recipient.key().as_ref()], bump)]
    pub escrow_account: AccountInfo<'info>,

    #[account(mut)]
    pub recipient: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in escrow account")]
    InsufficientFunds,
    
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
}
