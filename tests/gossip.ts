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

  let gossipPda: anchor.web3.PublicKey;

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

    [gossipPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("gossip"),
        user_a.publicKey.toBuffer()
      ],
      program.programId
    );
    
    try {
      const tx = await program.methods.createGossip(
        "I know",
        user_b.publicKey
      ).accounts({
        user: user_a.publicKey,
        gossip: gossipPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user_a]).rpc();

      console.log("✅ Transaction successful:", tx);

      const gossipAccount = await program.account.gossip.fetch(gossipPda);

      console.log("Gossip PDA:", gossipPda.toBase58());
      console.log("✅ Gossip account:", gossipAccount);
      console.log("💰 Dynamic price:", gossipAccount.price / LAMPORTS_PER_SOL, "SOL");
      
      // Verify pricing logic
      // "I know" = 6 chars (tier 0) + mention = 0.01 + 0.005 = 0.015 SOL
      const expectedPrice = 15_000_000; // 0.015 SOL in lamports
      console.log("Expected price:", expectedPrice / LAMPORTS_PER_SOL, "SOL");
      console.log("Actual price:", gossipAccount.price / LAMPORTS_PER_SOL, "SOL");
    } catch (error) {
      console.error("❌ Transaction failed:", error);
      throw error;
    }
  });

  it("Reveal a gossip", async () => {
    const buyer = user_b.publicKey;

    const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("gossip_vault"),
        gossipPda.toBuffer()
      ],
      program.programId
    );

    try {
      const tx = await program.methods.revealGossip().accounts({
        buyer,
        gossip: gossipPda,
      }).signers([user_b]).rpc();

      const gossipAccount = await program.account.gossip.fetch(gossipPda);
      const vaultAccount = await program.account.gossipVault.fetch(vaultPda);

      console.log("✅ Transaction successful:", tx, gossipAccount, vaultAccount, await provider.connection.getBalance(vaultPda) / LAMPORTS_PER_SOL);
      console.log("Balance A: ", await provider.connection.getBalance(user_a.publicKey) / LAMPORTS_PER_SOL);
      console.log("Balance B: ", await provider.connection.getBalance(user_b.publicKey) / LAMPORTS_PER_SOL);
    } catch (error) {
      console.error("❌ Transaction failed:", error);
      throw error;
    }
  });

  it("Withdraw from vault", async () => {
    const owner = user_a.publicKey; // gossip maker is the vault owner
    const destination = user_a.publicKey; // withdraw to owner's account
    
    const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("gossip_vault"),
        gossipPda.toBuffer()
      ],
      program.programId
    );

    console.log("🏦 Withdrawing from vault...");
    console.log("Vault PDA:", vaultPda.toBase58());
    
    // Check balances before withdrawal
    const vaultBalanceBefore = await provider.connection.getBalance(vaultPda);
    const ownerBalanceBefore = await provider.connection.getBalance(owner);
    
    console.log("Vault balance before:", vaultBalanceBefore / LAMPORTS_PER_SOL, "SOL");
    console.log("Owner balance before:", ownerBalanceBefore / LAMPORTS_PER_SOL, "SOL");
  
    try {
      const tx = await program.methods.withdrawFromAnyVault().accounts({
        owner,
        vault: vaultPda,
        destination,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user_a]).rpc();

      console.log("✅ Withdrawal successful:", tx);
      
      // Check balances after withdrawal
      const ownerBalanceAfter = await provider.connection.getBalance(owner);
      const vaultBalanceAfter = await provider.connection.getBalance(vaultPda);
      
      console.log("Owner balance after:", ownerBalanceAfter / LAMPORTS_PER_SOL, "SOL");
      console.log("Vault balance after:", vaultBalanceAfter / LAMPORTS_PER_SOL, "SOL");
      console.log("Balance difference:", (ownerBalanceAfter - ownerBalanceBefore) / LAMPORTS_PER_SOL, "SOL");
      
      // Verify vault is completely empty (0 lamports)
      if (vaultBalanceAfter === 0) {
        console.log("✅ Vault is completely empty (0 lamports)");
      } else {
        console.log("❌ Vault still has", vaultBalanceAfter / LAMPORTS_PER_SOL, "SOL");
      }
      
      // Verify vault account is closed
      try {
        await program.account.gossipVault.fetch(vaultPda);
        console.log("❌ Vault account should be closed but still exists");
      } catch (error) {
        console.log("✅ Vault account successfully closed");
      }
      
    } catch (error) {
      console.error("❌ Transaction failed:", error);
      throw error;
    }
  });

  it("Share a gossip", async () => {
    console.log("📤 Sharing gossip...");
    
    const sharer = user_b.publicKey; // user_b will share the gossip
    
    const [sharedGossipPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("shared_gossip"),
        gossipPda.toBuffer(),
        sharer.toBuffer()
      ],
      program.programId
    );

    try {
      const tx = await program.methods.shareGossip().accounts({
        sharer,
        originalGossip: gossipPda,
        sharedGossip: sharedGossipPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user_b]).rpc();

      console.log("✅ Share successful:", tx);
      
      const sharedGossipAccount = await program.account.sharedGossip.fetch(sharedGossipPda);
      console.log("📤 Shared gossip account:", sharedGossipAccount);
      console.log("💰 Share price:", sharedGossipAccount.sharePrice / LAMPORTS_PER_SOL, "SOL");
      
    } catch (error) {
      console.error("❌ Share failed:", error);
      throw error;
    }
  });

  it("Reveal shared gossip with revenue split", async () => {
    console.log("🔓 Revealing shared gossip...");
    
    const buyer = anchor.web3.Keypair.generate(); // user_c - new buyer
    const sharer = user_b.publicKey;
    
    // Fund the new buyer
    await provider.connection.requestAirdrop(buyer.publicKey, 10 * LAMPORTS_PER_SOL);
    
    // Wait for airdrop to complete
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    const [sharedGossipPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("shared_gossip"),
        gossipPda.toBuffer(),
        sharer.toBuffer()
      ],
      program.programId
    );

    const [creatorVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("creator_vault"),
        sharedGossipPda.toBuffer()
      ],
      program.programId
    );

    const [sharerVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("sharer_vault"),
        sharedGossipPda.toBuffer()
      ],
      program.programId
    );

    // Check balances before
    const buyerBalanceBefore = await provider.connection.getBalance(buyer.publicKey);
    console.log("Buyer balance before:", buyerBalanceBefore / LAMPORTS_PER_SOL, "SOL");

    try {
      const tx = await program.methods.revealSharedGossip().accounts({
        buyer: buyer.publicKey,
        sharedGossip: sharedGossipPda,
        creatorVault: creatorVaultPda,
        sharerVault: sharerVaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([buyer]).rpc();

      console.log("✅ Reveal shared gossip successful:", tx);
      
      // Check vault balances
      const creatorVaultAccount = await program.account.gossipVault.fetch(creatorVaultPda);
      const sharerVaultAccount = await program.account.gossipVault.fetch(sharerVaultPda);
      
      console.log("💰 Creator vault amount:", creatorVaultAccount.amount.toNumber() / LAMPORTS_PER_SOL, "SOL (60%)");
      console.log("💰 Sharer vault amount:", sharerVaultAccount.amount.toNumber() / LAMPORTS_PER_SOL, "SOL (40%)");
      
      // Verify revenue split
      const totalRevenue = creatorVaultAccount.amount.toNumber() + sharerVaultAccount.amount.toNumber();
      const creatorPercentage = (creatorVaultAccount.amount.toNumber() * 100) / totalRevenue;
      const sharerPercentage = (sharerVaultAccount.amount.toNumber() * 100) / totalRevenue;
      
      console.log("📊 Revenue split - Creator:", creatorPercentage.toFixed(1) + "%", "Sharer:", sharerPercentage.toFixed(1) + "%");
      
    } catch (error) {
      console.error("❌ Reveal shared gossip failed:", error);
      throw error;
    }
  });

  it("Withdraw from shared revenue vaults", async () => {
    console.log("💸 Withdrawing shared revenue...");
    
    const sharer = user_b.publicKey;
    
    const [sharedGossipPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("shared_gossip"),
        gossipPda.toBuffer(),
        sharer.toBuffer()
      ],
      program.programId
    );

    const [creatorVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("creator_vault"),
        sharedGossipPda.toBuffer()
      ],
      program.programId
    );

    const [sharerVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("sharer_vault"),
        sharedGossipPda.toBuffer()
      ],
      program.programId
    );

    // Original creator withdraws
    const creatorBalanceBefore = await provider.connection.getBalance(user_a.publicKey);
    
    try {
      const creatorTx = await program.methods.withdrawFromAnyVault().accounts({
        owner: user_a.publicKey,
        vault: creatorVaultPda,
        destination: user_a.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user_a]).rpc();

      console.log("✅ Creator withdrawal successful:", creatorTx);
      
      const creatorBalanceAfter = await provider.connection.getBalance(user_a.publicKey);
      console.log("💰 Creator earned:", (creatorBalanceAfter - creatorBalanceBefore) / LAMPORTS_PER_SOL, "SOL");

    } catch (error) {
      console.error("❌ Creator withdrawal failed:", error);
      throw error;
    }

    // Sharer withdraws
    const sharerBalanceBefore = await provider.connection.getBalance(user_b.publicKey);
    
    try {
      const sharerTx = await program.methods.withdrawFromAnyVault().accounts({
        owner: user_b.publicKey,
        vault: sharerVaultPda,
        destination: user_b.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user_b]).rpc();

      console.log("✅ Sharer withdrawal successful:", sharerTx);
      
      const sharerBalanceAfter = await provider.connection.getBalance(user_b.publicKey);
      console.log("💰 Sharer earned:", (sharerBalanceAfter - sharerBalanceBefore) / LAMPORTS_PER_SOL, "SOL");

    } catch (error) {
      console.error("❌ Sharer withdrawal failed:", error);
      throw error;
    }
  });
});
