# Gossip

![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-00D4AA?style=for-the-badge&logo=anchor&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)

Gossip is a Solana program that enables users to create paid gossip messages with a reveal mechanism. Users can create gossip about others, and interested parties can pay to reveal the content, creating a monetized gossip marketplace on the blockchain.

> [!NOTE]  
> **Program ID**: `8afyMAB2tiA8a6M9KMYgWfcrLK5nKcbp7NBuqdYxW8kR`

## Overview

The Gossip program implements a simple yet effective monetization model for gossip content. Users create gossip messages with a fixed price, and others can pay to reveal the content. The system uses secure vaults to handle payments and ensures only the gossip creator can withdraw earnings.

## Architecture

### Core Components

#### 1. **Gossip**
The main data structure representing a gossip message.

- **Maker**: The creator of the gossip message
- **Text**: The gossip content (max 20 characters)
- **Mention**: The person being mentioned in the gossip
- **Price**: Dynamic price based on text length and mention status
- **Is Revealed**: Boolean flag tracking revelation status
- **Total Collected**: Running total of earnings from reveals

#### 2. **GossipVault**
A secure PDA vault that holds payments for gossip reveals.

- **PDA Derivation**: `["gossip_vault", gossip_key]`
- **Owner**: The gossip creator who can withdraw funds
- **Amount**: Tracks the stored payment amount

### Program Design Patterns

#### PDA (Program Derived Address) Strategy
```rust
// Gossip PDA
seeds = [b"gossip", user.key().as_ref()]

// Vault PDA
seeds = [b"gossip_vault", gossip.key().as_ref()]
```

#### Payment & Vault System
- **Secure Storage**: Payments are held in program-owned PDAs
- **Owner Authorization**: Only gossip creators can withdraw
- **Automatic Closure**: Vaults are closed after withdrawal to reclaim rent

## Features

### Implemented Features

#### **Gossip Creation**
- Create gossip messages with mentions of other users
- Dynamic pricing model based on text length and mention status
- Automatic PDA generation for each gossip
- Text length validation (max 20 characters)

#### **Reveal Mechanism**
- Pay-to-reveal system for gossip content
- Secure vault creation for payment storage
- Prevention of double reveals
- Automatic payment processing

#### **Withdrawal System**
- Complete vault withdrawal with closure
- Owner-only authorization
- Automatic rent reclamation
- Manual lamport transfer for PDA accounts

#### **Security & Validation**
- Custom error handling for unauthorized access
- PDA seed validation
- Account ownership verification
- Proper constraint enforcement

### 💡 Use Cases

#### **Social Entertainment**
- Anonymous gossip sharing with monetization
- Celebrity or influencer gossip markets
- Community drama and entertainment

#### **Information Markets**
- Paid insider information sharing
- Exclusive content revelation
- Rumor and news monetization

### Key Instructions

1. **create_gossip**: Create a new gossip message with mention and pricing
2. **reveal_gossip**: Pay to reveal gossip content and create payment vault
3. **withdraw_from_vault**: Withdraw all earnings and close the vault

### Running Tests
```bash
# Run all tests
anchor test

# Run specific test commands
yarn run ts-mocha tests/gossip.ts
```

## Program Flow

### 1. **Gossip Creation**
```typescript
await program.methods.createGossip(
  "Secret info here",
  mentionedPersonPubkey
).accounts({
  user: creator.publicKey,
  gossip: gossipPda,
  systemProgram: SystemProgram.programId,
}).signers([creator]).rpc();
```

### 2. **Gossip Revelation**
```typescript
await program.methods.revealGossip().accounts({
  buyer: buyer.publicKey,
  gossip: gossipPda,
  vault: vaultPda,
  systemProgram: SystemProgram.programId,
}).signers([buyer]).rpc();
```

### 3. **Earnings Withdrawal**
```typescript
await program.methods.withdrawFromVault().accounts({
  owner: creator.publicKey,
  vault: vaultPda,
  gossip: gossipPda,
  destination: creator.publicKey,
  systemProgram: SystemProgram.programId,
}).signers([creator]).rpc();
```

## Technical Implementation

### State Management
```rust
#[account]
pub struct Gossip {
    pub maker: Pubkey,        // Creator of the gossip
    pub text: String,         // Gossip content (max 20 chars)
    pub mention: Pubkey,      // Person mentioned
    pub is_revealed: bool,    // Revelation status
    pub price: u64,          // Dynamic price based on text length and mention status
    pub bump: u8,            // PDA bump seed
    pub total_collected: u64, // Total earnings
}

#[account]
pub struct GossipVault {
    pub owner: Pubkey,       // Gossip creator
    pub amount: u64,         // Stored payment
}
```

### Security Features
- **PDA Authority**: Program controls vault signing
- **Owner Validation**: Custom constraint checking
- **Account Closure**: Automatic rent reclamation
- **Error Handling**: Custom error types for clarity

## Quick Start

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd gossip
   ```

2. **Install dependencies**
   ```bash
   yarn install
   ```

3. **Build the program**
   ```bash
   anchor build
   ```

4. **Run tests**
   ```bash
   anchor test
   ```

5. **Deploy to testnet**
   ```bash
   anchor deploy --provider.cluster testnet
   ```

## Future Development

### Planned Features
- **Dynamic Pricing**: Allow creators to set custom prices (tiered pricing)
- **Batch Operations**: Multiple gossip management
- **Reputation System**: Track user credibility
- **Content Moderation**: Filtering and reporting mechanisms
- **Revenue Sharing**: Split earnings with mentioned parties

### Potential Improvements
- **Gas Optimization**: Reduce transaction costs
- **Mobile Integration**: React Native compatibility
- **Social Features**: Following, blocking, and notifications
- **Analytics**: Earnings and engagement tracking

## Contributing

This project is ready for testnet deployment and community feedback. Contributions welcome for:
- Security audits and improvements
- Feature enhancements
- Documentation updates
- Test coverage expansion

## License

ISC License - See package.json for details.
