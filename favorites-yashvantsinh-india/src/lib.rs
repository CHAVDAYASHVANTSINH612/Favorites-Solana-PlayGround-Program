use anchor_lang::prelude::*;

declare_id!("5wETgfJT5qVDgGzc25YaxoybNjvfbfUVAGCkdmjpMXxt");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; //SPECIFIES TYPE OF ACCOUNT IT IS

#[program] // by adding this rust program becomes solana contract (it is power of anchor)
pub mod favorites {
    use super::*;

    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}", context.program_id);
        let user_public_key = context.accounts.user.key();

        msg!(
            "User {}'s favorite number: {}, color is {} and hobbies are {:?}",
            user_public_key,
            number,
            color,
            hobbies
        );
        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(()) // return
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}