import { Connection, Keypair, SystemProgram, PublicKey, Commitment } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, Address, BN } from "@project-serum/anchor"
import { WbaVault, IDL } from "./programs/wba_vault";
import wallet from "../../../wba-wallet.json"
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

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

// Mint address
const mint = new PublicKey("FJyS7xhbQjp5aJSQDNbp9HaMBtcs7kEPwwovXbC5vkrN");

const token_decimals = 1_000_000n;

// Execute our enrollment transaction
(async () => {
    try {

        // Get the token account of the fromWallet address, and if it does not exist, create it
        const ownerAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );
        console.log(`Your ata is: ${ownerAta.address.toBase58()}`);
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const vaultAta  = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            vaultAuth,
            true
        );

        const signature = await program.methods
        .depositSpl(new BN(2000000))
        .accounts({
          owner: keypair.publicKey,
          vaultState,
          vaultAuth,
          systemProgram: SystemProgram.programId,
          ownerAta: ownerAta.address,
          vaultAta: vaultAta.address,
          tokenMint: mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([
            keypair
        ]).rpc();

        console.log(`Deposit SPL success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

// https://explorer.solana.com/tx/ryRgMiu55fwJGko9CAe8nbPbke7tmnddCmpeEY8ZsY1dVg62xFUkiJNmrWDzdhqYFDkMYciDgUoQXnMU8Y1k5YP?cluster=devnet