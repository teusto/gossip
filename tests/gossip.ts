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

  let vaultPda: anchor.web3.PublicKey;

  let gossipPda: anchor.web3.PublicKey; // PDAs for gossips
  
  

  before(async () => {
    console.log("🤑 Funding test accounts...")
    const balance = await provider.connection.getBalance(user_a.publicKey);
    await provider.connection.requestAirdrop(user_b.publicKey, 10 * LAMPORTS_PER_SOL);
    const balance_b = await provider.connection.getBalance(user_b.publicKey);
    console.log("Balance A: ", balance / LAMPORTS_PER_SOL);
    console.log("Balance B: ", balance_b / LAMPORTS_PER_SOL);
  })

  it("Create a new gossip", async () => {
    console.log("Creating a new gossip...")
    const index = new anchor.BN(1);
    const price = new anchor.BN(0.1 * LAMPORTS_PER_SOL);

    const [gossipPda, gossipBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("gossip"),
        user_a.publicKey.toBuffer(),
        index.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );

    const [vaultPda, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("vault"),
        gossipPda.toBuffer(),
      ],
      program.programId
    );
    
    try {
      const tx = await program.methods.createGossip(
        "I know",
        user_b.publicKey,
        index,
        price
      ).accounts({
        user: user_a.publicKey,
        gossip: gossipPda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user_a]).rpc();

      console.log("✅ Transaction successful:", tx);

      const gossipAccount = await program.account.gossip.fetch(gossipPda);

      console.log("✅ Gossip account:", gossipAccount);
    } catch (error) {
      console.error("❌ Transaction failed:", error);
      throw error;
    }
  });

  it("Reveal a gossip", async () => {
    console.log("Actors on that test:")
    console.log("User A:", user_a.publicKey.toBase58());
    console.log("User B:", user_b.publicKey.toBase58());
    console.log("Gossip PDA:", gossipPda.toBase58());


    try {
      const tx = await program.methods.revealGossip(new anchor.BN(0)).accounts({
        user: user_b.publicKey,
        gossip: gossipPda,
      }).signers([user_b]).rpc();

      const gossipAccount = await program.account.gossip.fetch(gossipPda);

      console.log("✅ Transaction successful:", tx, gossipAccount);
    } catch (error) {
      console.error("❌ Transaction failed:", error);
      throw error;
    }
  })
});
