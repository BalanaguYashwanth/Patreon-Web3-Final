use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::{token::{TokenAccount, Mint, Token}};

// use anchor_spl::token::Transfer;
use {
    anchor_spl::{
        associated_token,
        token,
    },
};
use spl_token::instruction::transfer;
use solana_program::program::invoke_signed;

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

    pub fn transfer_nft(ctx: Context<TransferNft>)->ProgramResult{
        
        msg!("Creating buyer token account...");
        msg!("Buyer Token Address: {}", &ctx.accounts.buyer_token_account.key());    
        
        associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create {
                    payer: ctx.accounts.buyer_authority.to_account_info(),
                    associated_token: ctx.accounts.buyer_token_account.to_account_info(),
                    authority: ctx.accounts.buyer_authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
        )?;

        msg!("Transferring NFT...");
        msg!("Owner Token Address: {}", &ctx.accounts.owner_token_account.key());    
        msg!("Buyer Token Address: {}", &ctx.accounts.buyer_token_account.key());    
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.owner_token_account.to_account_info(),
                    to: ctx.accounts.buyer_token_account.to_account_info(),
                    authority:ctx.accounts.token_holder.to_account_info(), //ctx.accounts.seller.to_account_info()
                }
            ),
            1
        )?;
        

        // let transfer_tokens_instruction = transfer(
        //     &ctx.accounts.mint.key(),
        //     &ctx.accounts.owner_token_account.key(),
        //     &ctx.accounts.buyer_token_account.key(),
        //     &ctx.accounts.buyer_authority.key(),
        //     &[],
        //    1
        // )?;

        // let required_accounts_for_transfer = [
        //     ctx.accounts.seller.clone(),
        //     ctx.accounts.buyer.clone(),
        //     ctx.accounts.seller.clone(),
        // ];

        // invoke_signed(
        //     &transfer_tokens_instruction,
        //     &required_accounts_for_transfer,
        //     &[ ]
        // )?;

        msg!("NFT transferred successfully.");
        
        msg!("Sale completed successfully!");

        Ok(())
    }


    pub fn initializestatepda(_ctx: Context<Initialisedstatepda>,_bump:u8) -> Result<()> {
        msg!("state got Initialised");
        Ok(())
    }

    pub fn verifytoken(ctx:Context<VerifyPatreonDetails>, owner:Pubkey,date:u64,token_address:String) -> ProgramResult{
         let verify_details  = &mut ctx.accounts.verify_patreon_token_details;
         verify_details.owner = owner;
         verify_details.date=date;
         verify_details.token_address=token_address;
        //  verify_details.fund_pda_wallet=fund_pda_wallet;
         Ok(())
    }

    pub fn verifyalltokens(ctx:Context<VerifyPatreonDetailsAccount>, owner:Pubkey,date:String,token_address:String) -> ProgramResult{
        let verify_details  = &mut ctx.accounts.patreon_token_details;
        verify_details.owner = owner;
        verify_details.date=date;
        verify_details.token_address=token_address;
       //  verify_details.fund_pda_wallet=fund_pda_wallet;
        Ok(())
   }

    pub fn admin_details_registration(ctx:Context<RegisterAdminDetails>,name:String, description:String,contents:String,url:String,amount: u64,patreon_fund_address:Pubkey,owner:Pubkey,time:u64)->ProgramResult{
        
        let all_details = &mut ctx.accounts.details;
        all_details.name = name;
        all_details.amount = amount;
        all_details.description = description;
        all_details.contents = contents;
        all_details.url = url;
        all_details.patreon_fund_address = patreon_fund_address;
        all_details.owner = owner;
        all_details.time = time;
        Ok(())
    }

    // name:String,
    // description:String,
    // contents:String,
    // url:String,
    // amount:u64,
    // patreon_id:Pubkey,
    // owner:Pubkey

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
    pub patreon_account:Account<'info,PatreonDB>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>,
  
}

#[derive(Accounts)]
pub struct TransferNft<'info>{
    #[account(mut)]
    buyer: AccountInfo<'info>,
    #[account(mut)]
    seller: AccountInfo<'info>,
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    // #[account(mut)]
    // pub owner_authority: Pubkey,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_holder:Signer<'info,>, //signer
    #[account(mut)]
    pub buyer_token_account: UncheckedAccount<'info>, //UncheckedAccount<'info>,
    #[account(mut)]
    pub buyer_authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
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
pub struct VerifyPatreonDetails<'info,>{
    #[account(init,payer=user,space=9000)]
    pub verify_patreon_token_details:Account<'info,VerifyPatreonToken>,
    #[account(mut)]
    pub user:Signer<'info,>,
    pub system_program:Program<'info,System>
}

#[derive(Accounts)]
pub struct VerifyPatreonDetailsAccount<'info,>{
    #[account(init,payer=user,space=9000)]
    pub patreon_token_details:Account<'info,VerifyPatreonTokenDetails>,
    #[account(mut)]
    pub user:Signer<'info,>,
    pub system_program:Program<'info,System>
}

#[derive(Accounts)]
pub struct RegisterAdminDetails<'info,>{
    #[account(init,payer=user,space=9000)]
    pub details:Account<'info,AdminDetails>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>
}

#[account]
pub struct VerifyPatreonToken{
    // name:String,
    // description:String,
    // amount:i64,
    //mediaURL:String,
    // PDA:String, //where we are transferring account 2 or more sol
    // fund_pda_wallet:String,
    owner:Pubkey, //patreonkeypair
    date:u64,
    token_address:String, //newly created NFT address
}

#[account]
pub struct VerifyPatreonTokenDetails{
    // name:String,
    // description:String,
    // amount:i64,
    //mediaURL:String,
    // PDA:String, //where we are transferring account 2 or more sol
    // fund_pda_wallet:String,
    owner:Pubkey, //patreonkeypair
    date:String,
    token_address:String, //newly created NFT address
}


#[account]
pub struct AdminDetails{
    name:String,
    description:String,
    contents:String,
    url:String,
    amount:u64,
    patreon_fund_address:Pubkey,
    owner:Pubkey,
    time:u64
}

#[account]
pub struct PatreonDB{
    admin:Pubkey,
    name:String,
    description:String,
    amount:u64,
    //mediaURL:String,
}

#[account]
#[derive(Default)]
pub struct State {
    bump: u8,
    amount: u64,
}