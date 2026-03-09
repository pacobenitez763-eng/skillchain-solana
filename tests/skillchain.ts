import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Skillchain } from "../target/types/skillchain";

describe("skillchain", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Skillchain as Program<Skillchain>;

  it("Crear curso", async () => {
    const curso = anchor.web3.Keypair.generate();

    await program.methods
      .crearCurso("Curso Solana", "Aprender Solana desde cero")
      .accounts({
        curso: curso.publicKey,
        admin: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([curso])
      .rpc();

    console.log("Curso creado:", curso.publicKey.toString());
  });
});
