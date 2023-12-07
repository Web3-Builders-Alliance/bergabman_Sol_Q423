import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import { Connection, Keypair, SystemProgram, PublicKey, Commitment, LAMPORTS_PER_SOL } from "@solana/web3.js"


describe("anchor-escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;

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

  const maker = Keypair.generate();
  const taker = Keypair.generate();

  let mintA: PublicKey;
  let mintB: PublicKey;



  // it("Airdrop", async () => {
  //   await Promise.all([maker,taker].map((k) =>  provider.connection.requestAirdrop(k.publicKey, LAMPORTS_PER_SOL * 5)
  //   .then(confirm)
  //   .then(log)))
  // })

  it("Make!", async () => {
    // Add your test here.
    const tx = await program.methods.make().accounts({
      maker
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
