import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { DevCapital } from "../target/types/dev_capital";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";




describe("dev-capital", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DevCapital as Program<DevCapital>;

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
  const { connection } = provider;

  const funder = Keypair.generate();
  const dev = Keypair.generate();
  const random_key_3 = Keypair.generate();
  const random_key_4 = Keypair.generate();
  console.log(`funder key ${funder.publicKey}`);
  console.log(`dev key ${dev.publicKey}`);

  const dev_fund = PublicKey.findProgramAddressSync([
    Buffer.from("dev_fund"),
    funder.publicKey.toBuffer(),
    dev.publicKey.toBuffer(),
  ],
  program.programId)[0];
  console.log(`dev_fund pda key ${dev_fund}`);

  const dev_deploy = PublicKey.findProgramAddressSync([
    Buffer.from("dev_deploy"),
    dev_fund.toBuffer(),
    dev.publicKey.toBuffer(),
  ],
  program.programId)[0];

  const dev_deploy_offsets = PublicKey.findProgramAddressSync([
    Buffer.from("dev_deploy_offsets"),
    dev_deploy.toBuffer(),
  ],
  program.programId)[0];
  const dev_deploy_data = PublicKey.findProgramAddressSync([
    Buffer.from("dev_deploy_data"),
    dev_deploy.toBuffer(),
  ],
  program.programId)[0];

  it("Airdrop", async () => {
    await Promise.all([
      await connection.requestAirdrop(funder.publicKey, LAMPORTS_PER_SOL * 50)
      .then(confirm).then(log),
      await connection.requestAirdrop(dev.publicKey, LAMPORTS_PER_SOL * 50)
      .then(confirm),
      // bergNvF6e4qZ9dJDYHBBhC9r644Mg5S2rB7PdDC3USH
    ])
  })

  it("Is initialized!", async () => {
    // Add your test here.
    try {
      const tx = await program.methods.initialize().rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
      error.logs.forEach(element => {
        console.log(element);
      });
    }
    
  });

  it("Initialized dev fund!", async () => {
    // Add your test here.

    try {
      const tx = await program.methods.initDevFund(new BN(LAMPORTS_PER_SOL * 9)).accounts({
        funder: funder.publicKey,
        dev: dev.publicKey,
        devFund: dev_fund,
        systemProgram: SystemProgram.programId,
      }).signers([funder]).rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
      error.logs.forEach(element => {
        console.log(element);
      });
    }
    
  });

  it("Initialized dev deploy!", async () => {
    // Add your test here.

    try {
      const tx = await program.methods.initDevDeploy(42000*1, 42000*1, 42000*1,).accounts({
        // funder: funder.publicKey,
        dev: dev.publicKey,
        devFund: dev_fund,
        devDeploy: dev_deploy,
        devDeployOffsets: dev_deploy_offsets,
        devDeployData: dev_deploy_data,
        systemProgram: SystemProgram.programId,
      }).signers([dev]).rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
      error.logs.forEach(element => {
        console.log(element);
      });
    }
    
  });
});
