use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod skillchain {
    use super::*;

    pub fn crear_curso(
        ctx: Context<CrearCurso>,
        titulo: String,
        descripcion: String
    ) -> Result<()> {

        let curso = &mut ctx.accounts.curso;

        curso.titulo = titulo;
        curso.descripcion = descripcion;
        curso.creador = ctx.accounts.admin.key();

        Ok(())
    }
}

#[account]
pub struct Curso {
    pub titulo: String,
    pub descripcion: String,
    pub creador: Pubkey,
}

#[derive(Accounts)]
pub struct CrearCurso<'info> {

    #[account(init, payer = admin, space = 300)]
    pub curso: Account<'info, Curso>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}
