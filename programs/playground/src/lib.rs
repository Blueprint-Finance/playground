use anchor_lang::prelude::*;

use std::io::Write;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod playground {

    use anchor_lang::Discriminator;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.user = ctx.accounts.user.key();
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }

    pub fn create_storage(ctx: Context<CreateStorage>) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.authority = ctx.accounts.authority.key();
        storage.space = 8;
        storage.total_records = 0;
        // storage.indices = Vec::new();
        storage.positions = Vec::new();

        Ok(())
    }

    pub fn assign_storage(ctx: Context<AssignStorage>, entries: Vec<Position>) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        msg!("Assigning storage");

        let num_entries = entries.len();
        let storage_space = (storage.space - storage.total_records) as usize;
        if num_entries > storage_space {
            return err!(PlaygroundError::NotEnoughSpace);
        }
        for (i, entry) in entries.into_iter().enumerate() {
            // storage.indices.push((num_entries + i) as u8);
            storage.positions.push(entry);
            storage.total_records += 1;
        }

        msg!("Indices capacity {}, length {}", storage.positions.capacity(), storage.positions.len());
        let mut buffer = Vec::<u8>::with_capacity(256);
        buffer.write_all(Storage::discriminator().as_slice()).unwrap();
        storage.serialize(&mut buffer)?;

        msg!("Buffer length: {}", buffer.len());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 64)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub user: Pubkey,
    pub count: u64,
}

#[derive(Accounts)]
pub struct CreateStorage<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        seeds = [authority.key.as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<Storage>())
    ]
    pub storage: Account<'info, Storage>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssignStorage<'info> {
    #[account(mut)]
    pub storage: Account<'info, Storage>,
}

#[derive(Accounts)]
pub struct IncreaseStorage<'info> {
    #[account(mut)]
    pub storage: Account<'info, Storage>,
}

#[derive(Accounts)]
pub struct DecreaseStorage<'info> {
    #[account(mut)]
    pub storage: Account<'info, Storage>,
}

#[account]
#[repr(C)]
#[derive(Debug)]
pub struct Storage {
    pub authority: Pubkey,
    pub space: u8,
    pub total_records: u8,
    pub padding: [u8; 6],
    pub positions: Vec<Position>,
    // pub indices: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, PartialEq)]
pub struct Position {
    pub value: u64,
    pub address: Pubkey,
}

#[error_code]
pub enum PlaygroundError {
    #[msg("Invalid owner")]
    InvalidOwner,

    #[msg("Not enough storage space")]
    NotEnoughSpace,
}
