import { Connection, Keypair, SystemProgram, PublicKey, Commitment } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, Address, BN } from "@project-serum/anchor"
import { WbaVault, IDL } from "../programs/wba_vault";
import wallet from "../wba-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Commitment
const commitment: Commitment = "confirmed";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment });

// Create our program
const program = new Program<WbaVault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);

// E7xnTWFGss7CgqGioiSic5tYjscvs2RJutqDpx6Z2MuS
const vaultState = new PublicKey("E7xnTWFGss7CgqGioiSic5tYjscvs2RJutqDpx6Z2MuS");
console.log(`Vault public key: ${vaultState.toBase58()}`);

// Cvault auth seeds
const vaultAuthKeys = [Buffer.from("auth"), vaultState.toBuffer()];

// Create the PDA for our enrollment account
const [vaultAuth, _bump] = PublicKey.findProgramAddressSync(
    vaultAuthKeys,
    program.programId
);

// Create the vault key
const vaultKeys = [Buffer.from("vault"), vaultAuth.toBuffer()];
const [vaultKey, _bump2] = PublicKey.findProgramAddressSync(
  vaultKeys,
  program.programId
);



// Execute our enrollment transaction
(async () => {
    try {

        // const signature = await program.methods
        // .withdraw(new BN(<number>))
        // .accounts({
        //    ???
        // })
        // .signers([
        //     keypair
        // ]).rpc();
        // console.log(`Withdraw success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();