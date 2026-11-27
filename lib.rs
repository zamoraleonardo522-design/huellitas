use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

// Solana Playground reemplaza este ID al compilar (Build)
declare_id!("11111111111111111111111111111111");

#[program]
pub mod huellitas {
    use super::*;

    /// Transfiere una donación en SOL al refugio y
    /// registra su importe en pesos (en centavos).
    pub fn donar(ctx: Context<Donar>, lamports: u64, centavos_mxn: u64) -> Result<()> {
        require!(lamports > 0 && centavos_mxn > 0, ErrorDonacion::MontoInvalido);

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.donante.to_account_info(),
                    to: ctx.accounts.refugio.to_account_info(),
                },
            ),
            lamports,
        )?;

        emit!(DonacionRecibida {
            donante: ctx.accounts.donante.key(),
            refugio: ctx.accounts.refugio.key(),
            lamports,
            centavos_mxn,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Donar<'info> {
    #[account(mut)]
    pub donante: Signer<'info>,
    #[account(mut)]
    pub refugio: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct DonacionRecibida {
    pub donante: Pubkey,
    pub refugio: Pubkey,
    pub lamports: u64,
    pub centavos_mxn: u64,
}

#[error_code]
pub enum ErrorDonacion {
    #[msg("El monto debe ser mayor a cero")]
    MontoInvalido,
}
