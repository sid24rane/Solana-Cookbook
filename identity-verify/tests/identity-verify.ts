import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { IdentityVerify } from "../target/types/identity_verify";

describe("identity-verify", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.IdentityVerify as Program<IdentityVerify>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
