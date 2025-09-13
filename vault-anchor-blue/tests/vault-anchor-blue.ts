import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VaultAnchorBlue } from "../target/types/vault_anchor_blue";

describe("vault-anchor-blue", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.vaultAnchorBlue as Program<VaultAnchorBlue>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
