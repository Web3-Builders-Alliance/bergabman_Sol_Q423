import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import wallet from "../../../../wba-wallet.json"

import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = createNft(umi, {
      mint,
      name: "WBArug",
      symbol: "WRUG",
      uri: "https://arweave.net/nNGbeNgJ5xO0WkzkPDGDVofQwpwSjAu-lp7l0xwsI-U",
      sellerFeeBasisPoints: percentAmount(1, 2),
    });
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
  
    console.log(
      `Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
  
    console.log("Mint Address: ", mint.publicKey);
  })();
  // https://explorer.solana.com/address/Dv9sRcnMhN8XcZSgd7FAgYXktwC2dxy8xQtQuP9N1the/attributes?cluster=devnet