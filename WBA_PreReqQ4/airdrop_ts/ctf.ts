import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js"
import { Ctf, IDL } from "./programs/ctf";
import { Program, Wallet, AnchorProvider, Address } from "@project-serum/anchor"

import wallet from "./wba_wallet.json"
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");
const provider = new AnchorProvider(connection, new Wallet(keypair), {commitment: "confirmed"});
const program = new Program<Ctf>(IDL,"ctf1VWeMtgxa24zZevsXqDg6xvcMVy4FbP3cxLCpGha" as Address, provider);


// const to = new PublicKey("bergNvF6e4qZ9dJDYHBBhC9r644Mg5S2rB7PdDC3USH");
const ctf_seeds = [Buffer.from("profile"), keypair.publicKey.toBuffer()];
const [ctf_profile, _bump] = PublicKey.findProgramAddressSync(ctf_seeds, program.programId);

// Execute our enrollment transaction
(async () => {
    try {
        const txhash = await program.methods
            .initializeCtfProfile()
            .accounts({
            owner: keypair.publicKey,
            profile: ctf_profile,
            systemProgram: SystemProgram.programId,
            })
            .signers([
                keypair
            ]).rpc();

        console.log(`Success! Check out your TX here:
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

// (async () => {
//     try {
//         const transaction = new Transaction().add(
//             SystemProgram.transfer({
//             fromPubkey: from.publicKey,
//             toPubkey: to,
//             lamports: LAMPORTS_PER_SOL/10,
//         })
//     );
//     transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
//     transaction.feePayer = from.publicKey;
//     // Sign transaction, broadcast, and confirm
//     const signature = await sendAndConfirmTransaction(
//         connection,
//         transaction,
//         [from]
//     );
//     console.log(`Success! Check out your TX here:
//     https://explorer.solana.com/tx/${signature}?cluster=devnet`);
//     } catch(e) {
//     console.error(`Oops, something went wrong: ${e}`)
//     }
// })();

// (async () => {
//     try {
//         // Get balance of dev wallet
//         const balance = await connection.getBalance(from.publicKey)
//         // Create a test transaction to calculate fees
//         const transaction = new Transaction().add(
//             SystemProgram.transfer({
//             fromPubkey: from.publicKey,
//             toPubkey: to,
//             lamports: balance,
//             })
//     );
//     transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
//     transaction.feePayer = from.publicKey;
//     const fee = (await connection.getFeeForMessage(transaction.compileMessage(), 'confirmed')).value || 0;
//     // Remove our transfer instruction to replace it
//     transaction.instructions.pop();
//     // Now add the instruction back with correct amount of lamports
//     transaction.add(
//         SystemProgram.transfer({
//         fromPubkey: from.publicKey,
//         toPubkey: to,
//         lamports: balance - fee,
//         })
//     );
//     // Sign transaction, broadcast, and confirm
//     const signature = await sendAndConfirmTransaction(
//         connection,
//         transaction,
//         [from]
//     );
//     console.log(`Success! Check out your TX here:
//     https://explorer.solana.com/tx/${signature}?cluster=devnet`)
//     } catch(e) {
//     console.error(`Oops, something went wrong: ${e}`)
//     }
// })();