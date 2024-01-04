import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { MINT_SIZE, TOKEN_PROGRAM_ID, 
  createAssociatedTokenAccountIdempotentInstruction, 
  createInitializeMint2Instruction, 
  createMintToInstruction, 
  getAssociatedTokenAddress, 
  getAssociatedTokenAddressSync, 
  getMinimumBalanceForRentExemptAccount, 
  getMinimumBalanceForRentExemptMint, 
  getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { randomBytes } from "crypto";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

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


  describe("anchor-escrow", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
  
    const provider = anchor.getProvider();
  
    const { connection } = provider;
  
    const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;
  
    const seed = new BN(randomBytes(8));
  
    const maker = Keypair.generate();
    const taker = Keypair.generate();
    const mintA = Keypair.generate();
    const mintB = Keypair.generate();
    const makerAtaA = getAssociatedTokenAddressSync(mintA.publicKey, maker.publicKey);
    const makerAtaB = getAssociatedTokenAddressSync(mintB.publicKey, maker.publicKey);  
    const takerAtaA = getAssociatedTokenAddressSync(mintA.publicKey, taker.publicKey);
    const takerAtaB = getAssociatedTokenAddressSync(mintB.publicKey, taker.publicKey);
    const escrow = PublicKey.findProgramAddressSync([
      Buffer.from("escrow"),
      maker.publicKey.toBuffer(),
      seed.toBuffer('le', 8)    
    ],
    program.programId)[0];
    const vault = getAssociatedTokenAddressSync(mintA.publicKey, escrow, true);
  
    xit("Airdrop", async () => {
      await Promise.all([
        await connection.requestAirdrop(maker.publicKey, LAMPORTS_PER_SOL * 50)
        .then(confirm).then(log),
        await connection.requestAirdrop(taker.publicKey, LAMPORTS_PER_SOL * 50)
        .then(confirm),
        await connection.requestAirdrop(provider.publicKey, LAMPORTS_PER_SOL * 50)
        .then(confirm)
        // bergNvF6e4qZ9dJDYHBBhC9r644Mg5S2rB7PdDC3USH
      ])
    })
  
    it("Mint", async () => {
      let tx = new Transaction();
      const lamports = await getMinimumBalanceForRentExemptMint(connection);
      tx.instructions = [
        SystemProgram.createAccount({
          fromPubkey: provider.publicKey,
          newAccountPubkey: mintA.publicKey,
          lamports: 0,
          space: MINT_SIZE,
          programId: TOKEN_PROGRAM_ID
        }),
        SystemProgram.createAccount({
          fromPubkey: provider.publicKey,
          newAccountPubkey: mintB.publicKey,
          lamports: 0,
          space: MINT_SIZE,
          programId: TOKEN_PROGRAM_ID
        }),
        createInitializeMint2Instruction(
          mintA.publicKey,
          6,
          maker.publicKey,
          null
        ),
        createInitializeMint2Instruction(
          mintB.publicKey,
          6,
          taker.publicKey,
          null
        ),
        createAssociatedTokenAccountIdempotentInstruction(
          provider.publicKey,
          makerAtaA,
          maker.publicKey,
          mintA.publicKey
        ),
        createAssociatedTokenAccountIdempotentInstruction(
          provider.publicKey,
          takerAtaA,
          taker.publicKey,
          mintB.publicKey  
        ),  
        createMintToInstruction (
          mintA.publicKey,
          makerAtaA,
          maker.publicKey,
          1e9
        ),
        createMintToInstruction (
          mintB.publicKey,
          takerAtaB,
          maker.publicKey,
          1e9
        )
      ];
  
        await provider.sendAndConfirm(tx, [
          mintA, mintB, maker, taker
        ]).then(confirm).then(log);
        
      
      
    });
  
    it("Make", async () => {
      await program.methods.make(
        seed, 
        new BN(1e6),
        new BN(1e6)
      )
      .accounts(
        {
          maker: maker.publicKey,
          mintA: mintA.publicKey,
          mintB: mintB.publicKey,
          makerAtaA,
          escrow,
          vault,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId
        }
      )
      .signers([ maker ])
      .rpc()
      .then(confirm)
      .then(log)
    });
  
    
    
    it("Refund", async () => {
      await program.methods.refund()
      .accounts(
        {
          maker: maker.publicKey,
          mintA: mintA.publicKey,
          makerAtaA,
          escrow,
          vault,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId
        }
      )
      .signers([ maker ])
      .rpc()
      .then(confirm)
      .then(log)
    });
  
    
    it("Take", async () => {
      await program.methods.take()
      .accounts(
        {
          taker: taker.publicKey,
          maker: maker.publicKey,
          mintA: mintA.publicKey,
          mintB: mintB.publicKey,
          takerAtaA,
          takerAtaB,
          makerAtaB,
          escrow,
          vault,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId
        }
      )
      .signers([ maker ])
      .rpc()
      .then(confirm)
      .then(log)
    })
  
  
  
  });
})