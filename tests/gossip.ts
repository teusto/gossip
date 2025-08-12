import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Gossip } from "../target/types/gossip";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("gossip", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.gossip as Program<Gossip>;
  const provider = anchor.getProvider();

  const user_a = (provider.wallet as anchor.Wallet).payer;
  const user_b = anchor.web3.Keypair.generate();

  before(async () => {
    console.log("🤑 Funding test accounts...")
    const balance = await provider.connection.getBalance(user_a.publicKey);
    console.log("Balance: ", balance / LAMPORTS_PER_SOL);
  })

  it("Create a new gossip", async () => {
    console.log("Creating a new gossip...")
    
    try {
      const tx = await program.methods.createGossip(
        "I know",
        user_b.publicKey,
        new anchor.BN(2)
      ).accounts({
        user: user_a.publicKey,
      }).signers([user_a]).rpc();

      console.log("✅ Transaction successful:", tx);

      const [gossipPda, gossipBump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("gossip"), 
          user_a.publicKey.toBuffer(), 
          new anchor.BN(2).toArrayLike(Buffer, "le", 8) // Convert to little-endian 8-byte buffer
        ],
        program.programId
      );

      const gossipAccount = await program.account.gossip.fetch(gossipPda);

      console.log("✅ Gossip account:", gossipAccount);
    } catch (error) {
      console.error("❌ Transaction failed:", error);
      throw error;
    }
  });
});
