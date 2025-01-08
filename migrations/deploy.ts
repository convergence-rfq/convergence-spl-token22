// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@coral-xyz/anchor");
const { PublicKey } = require("solana");

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add token program initialization
  const tokenProgram = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
  const token22Program = new PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

  // Add token program to option market initialization
  await program.methods
    .initializeOptionMarket({
      // ... existing params ...
      tokenProgram: tokenProgram, // or token22Program
    })
    .accounts({
      // ... existing accounts ...
      tokenProgram: tokenProgram, // or token22Program
    })
    .rpc();
};
