lib.txt 


use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::{token::{TokenAccount, Mint, Token}};
use anchor_spl::token::Transfer;

declare_id!("D2cnhPbd5Go1DfbAWWgvn1f6sco4PM28NvFgmgKSr4vJ");

#[program]
pub mod patreon {
    use super::*;

    pub fn create_patreon(
        ctx: Context<CreatePatreon>,
        name:String,
        description:String,
        amount:u64,
    ) -> ProgramResult {
        let patreon_details=&mut ctx.accounts.patreon_db;
        patreon_details.admin = *ctx.accounts.user.key;
        patreon_details.name=name;
        patreon_details.description=description;
        patreon_details.amount=amount;
        Ok(())
    }

    pub fn withdraw(ctx:Context<Withdraw>,amount:u64)->ProgramResult{
        let user = &mut ctx.accounts.user;
        let patreon_account_data = &mut ctx.accounts.patreon_account;
        if patreon_account_data.admin!=*user.key {
            return Err(ProgramError::IncorrectProgramId)
        }
        let rent_balace=Rent::get()?.minimum_balance(patreon_account_data.to_account_info().data_len());
        if **patreon_account_data.to_account_info().lamports.borrow()-rent_balace < amount {
            return Err(ProgramError::InsufficientFunds)
        }
        **patreon_account_data.to_account_info().try_borrow_mut_lamports()? -=amount;
        **user.to_account_info().try_borrow_mut_lamports()? +=amount;
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> ProgramResult {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.patreon_account.key(),
            amount
        );
        let  _res = anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.patreon_account.to_account_info()
            ]
        );
        (&mut ctx.accounts.patreon_account).amount += amount;
        Ok(())
    }
    pub fn initializestatepda(_ctx: Context<Initialisedstatepda>,_bump:u8) -> Result<()> {
        msg!("state got Initialised");
        Ok(())
    }

  

    pub fn sendtokenpda(ctx: Context<SendTokenPDA>,_bump1:u8,_bump2:u8,_amount:u64) -> Result<()> {
        msg!("token process start for PDA transfer...");
        let state = &mut(ctx.accounts.statepda);
        msg!("before: {}",state.amount);
        msg!("{} bump after",state.bump);
        state.bump=_bump1;
        state.amount=_amount;
        msg!("after: {}",state.amount);
        msg!("{} bump after",state.bump);
        let bump_vector=_bump1.to_le_bytes();
        let dep=&mut ctx.accounts.deposit_token_account.key();
        let sender=&ctx.accounts.owner;
        let inner=vec![sender.key.as_ref(),dep.as_ref(),"state".as_ref(),bump_vector.as_ref()];
        let outer=vec![inner.as_slice()];
        let transfer_instruction = Transfer { 
            
            from : ctx.accounts.deposit_token_account.to_account_info(),
            to : ctx.accounts.tokenpda.to_account_info(),
            authority: sender.to_account_info()
        
        };
        let cpi_ctx = CpiContext::new_with_signer(
         ctx.accounts.token_program.to_account_info(),
         transfer_instruction,
         outer.as_slice(),
     );
       msg!("transfer call start");
        anchor_spl::token::transfer(cpi_ctx, _amount)?;
        ctx.accounts.tokenpda.reload()?;
        msg!("token pda key {}",ctx.accounts.tokenpda.key());
        msg!("token after transfer to reciever in PDA {}",ctx.accounts.tokenpda.amount);
        msg!("succesfully transfered"); 
        Ok(())
    }
}



#[derive(Accounts)]
pub struct CreatePatreon<'info>{
    #[account(init, payer=user, space=9000, seeds=[b"PATREON_DEMO".as_ref(), user.key().as_ref()], bump)]
    pub patreon_db:Account<'info,PatreonDB>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>
}


#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub patreon_account: Account<'info,PatreonDB>,
    #[account(mut)]
    pub user:Signer<'info>
}

#[derive(Accounts)]
pub struct Donate<'info>{
    #[account(mut)]
    patreon_account:Account<'info,PatreonDB>,
    #[account(mut)]
    user:Signer<'info>,
    system_program:Program<'info,System>
}

#[derive(Accounts)]
#[instruction(_bump : u8)]
pub struct Initialisedstatepda<'info> {
    #[account(
        init,
        payer = owner,
        seeds=[owner.key.as_ref(),deposit_token_account.key().as_ref(),"state".as_ref()],
        bump,
        space=200
    )]
    statepda: Account<'info, State>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub deposit_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info,System>,

}

#[derive(Accounts)]
#[instruction(_bump : u8)]
pub struct Initialisetokenpda<'info> {

    #[account(
        init,
        seeds = [owner.key.as_ref(),deposit_token_account.key().as_ref()],
        bump,
        payer = owner,
        token::mint = mint,
        token::authority = statepda,
     )]
     
    pub tokenpda: Account<'info, TokenAccount>,
    pub statepda: Account<'info,State>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub deposit_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info,System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info,Token>,
   
}
#[derive(Accounts)]
pub struct SendTokenPDA<'info> {

    #[account(mut)]
    pub tokenpda: Account<'info, TokenAccount>,
    pub statepda: Account<'info,State>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub deposit_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info,System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info,Token>,
   
}

#[account]
pub struct PatreonDB{
    admin:Pubkey,
    name:String,
    description:String,
    amount:u64,
}

#[account]
#[derive(Default)]
pub struct State {
    bump: u8,
    amount: u64,           
}