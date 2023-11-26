import { Keypair } from "@solana/web3.js";
import bs58 from 'bs58'
import prompt from 'prompt-sync';


//Generate a new keypair
let kp = Keypair.generate();
console.log(`You've generated a new Solana wallet:${kp.publicKey.toBase58()}[${kp.secretKey}]`);


describe("base58_to_wallet", () => {
    it('should ask for wallet', () => {
        base58_to_wallet();
      })
});



// #[test]
function base58_to_wallet() {
    console.log("Enter your name:");
    var prompt = require('prompt-sync')();
    var walletb58 = prompt('enter b58 wallet: ');
    let wallet = bs58.decode(walletb58);
    console.log("{:?}", wallet);
}

// #[test]
// fn wallet_to_base58() {
//     let wallet: Vec<u8> =
//     vec![34,46,55,124,141,190,24,204,134,91,70,184,161,181,44,122,15,172,6
//     3,62,153,150,99,255,202,89,105,77,41,89,253,130,27,195,134,14,66,75,24
//     2,7,132,234,160,203,109,195,116,251,144,44,28,56,231,114,50,131,185,16
//     8,138,61,35,98,78,53];
//     let base58 = bs58.encode(wallet).into_string();
//     println!("{:?}", base58);
// }


