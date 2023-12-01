import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { Connection, Keypair, SystemProgram, PublicKey, Commitment, LAMPORTS_PER_SOL } from "@solana/web3.js"
import wallet from "../../../wba-wallet.json"
// anchor test --skip-local-validator
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const vaultSeeds = [Buffer.from("vault"), keypair.publicKey.toBuffer()];
const program = anchor.workspace.AnchorVault as Program<AnchorVault>;

const [vault, _bump] = PublicKey.findProgramAddressSync(
    vaultSeeds,
    program.programId
);
const provider = anchor.AnchorProvider.env();


describe("anchor-vault", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...block
    })
    return signature
  }
  
  const log = async(signature: string): Promise<string> => {
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${provider.connection.rpcEndpoint}`);
    return signature;
  }

  it("Airdrop", async () => {
    await provider.connection.requestAirdrop(keypair.publicKey, LAMPORTS_PER_SOL * 1000000)
    .then(confirm)
    .then(log)
  })

  it("deposit!", async () => {
    // Add your test here.
    const tx = await program.methods.deposit(new anchor.BN(100000000000))
    .accounts({
      signer: keypair.publicKey,
      vault: vault,
      systemProgram: SystemProgram.programId,
  }).signers([keypair]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("close!", async () => {
    // Add your test here.
    const tx = await program.methods.close()
    .accounts({
      signer: keypair.publicKey,
      vault: vault,
      systemProgram: SystemProgram.programId,
  }).signers([keypair]).rpc();
    console.log("Your transaction signature", tx);
  });
});
