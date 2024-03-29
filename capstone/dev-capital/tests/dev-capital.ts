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

  const dev_config = PublicKey.findProgramAddressSync([
    Buffer.from("dev_config"),
    dev_fund.toBuffer(),
    dev.publicKey.toBuffer(),
  ],
  program.programId)[0];

  const deploy_offsets = PublicKey.findProgramAddressSync([
    Buffer.from("deploy_offsets"),
    dev_fund.toBuffer(),
    dev.publicKey.toBuffer()
  ],
  program.programId)[0];
  const deploy_data = PublicKey.findProgramAddressSync([
    Buffer.from("deploy_data"),
    dev_fund.toBuffer(),
    dev.publicKey.toBuffer()
  ],
  program.programId)[0];

  it("Airdrop", async () => {
    await Promise.all([
      await connection.requestAirdrop(funder.publicKey, LAMPORTS_PER_SOL * 50)
      .then(confirm).then(log),
      await connection.requestAirdrop(dev.publicKey, LAMPORTS_PER_SOL * 50)
      .then(confirm),
      await connection.requestAirdrop(provider.publicKey, LAMPORTS_PER_SOL * 50)
      .then(confirm).then(log),
      // bergNvF6e4qZ9dJDYHBBhC9r644Mg5S2rB7PdDC3USH
    ])
  })


  it("Initialized dev fund!", async () => {

    try {
      const tx = await program.methods.initDevFund(new BN(LAMPORTS_PER_SOL * 20)).accounts({
        funder: funder.publicKey,
        dev: dev.publicKey,
        devFund: dev_fund,
        systemProgram: SystemProgram.programId,
      }).signers([funder]).rpc();
      log(tx);
    } catch (error) {
      console.log(error);
      error.logs.forEach(element => {
        console.log(element);
      });
    }
    
  });

  it("Initialized dev deploy!", async () => {

    try {
      const tx = await program.methods.initDevConfig(30000*1, 30000*1, 206000*1, 286176*1,).accounts({
        dev: dev.publicKey,
        devFund: dev_fund,
        devConfig: dev_config,
        deployOffsets: deploy_offsets,
        deployData: deploy_data,
        systemProgram: SystemProgram.programId,
      }).signers([dev]).rpc();
      // log(tx);
    } catch (error) {
      console.log(error);
      error.logs.forEach(element => {
        console.log(element);
      });
    }
    
  });

  it("Accounts sized!", async () => {
    // Add your test here.
    const devDeployFetched = await program.account.devConfig.fetch(dev_config);
    const dataOrigLen = devDeployFetched.dataOrigLen;
    const offsets_len = devDeployFetched.ot5Len + devDeployFetched.ot6Len;
    // console.log(devDeployFetched);
    const transaction = new Transaction();

    const instr_offsets = await program.methods.accountSizeOffsets().accounts({
      dev: dev.publicKey,
      devFund: dev_fund,
      devConfig: dev_config,
      deployOffsets: deploy_offsets,
    }).instruction();

    const instr_data = await program.methods.accountSizeData().accounts({
      dev: dev.publicKey,
      devFund: dev_fund,
      devConfig: dev_config,
      deployData: deploy_data,
    }).instruction();

    let dataCount = 0;
    while ((dataCount*10240)<dataOrigLen*1) {
      dataCount+=1;
      transaction.add(instr_data)
    }

    let increaseCount = 0;
    while ((increaseCount*10240)<(offsets_len*3)+8) {
      increaseCount+=1;
      transaction.add(instr_offsets)
    }

    try {
      const signature = await anchor.web3.sendAndConfirmTransaction(
        connection,
        transaction,
        [dev],
      );
      log(signature);
    } catch (error) {
      console.log(error);
      error.logs.forEach(element => {
        console.log(element);
      });
    }
    
  });



  it("deploy offsets!", async () => {

    try {
      const tx = await program.methods.deployOffsets(Buffer.from([0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00])).accounts({
        dev: dev.publicKey,
        devFund: dev_fund,
        devConfig: dev_config,
        deployOffsets: deploy_offsets,
        deployData: deploy_data,
        // systemProgram: SystemProgram.programId,
      }).signers([dev]).rpc();
      log(tx);
    } catch (error) {
      console.log(error);
      error.logs.forEach(element => {
        console.log(element);
      });
    }
    
  });




});


