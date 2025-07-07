pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AHHFd8At9esnouazyFuEoKxZNyTKAfySR1vcNdxfBb3U");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(Context: Context<Initialize>) -> Result<()> {
        instructions::make_offer::send_offered_token_to_vault()?;
        instructions::make_offer::save_offer(context)
        
    }
}
