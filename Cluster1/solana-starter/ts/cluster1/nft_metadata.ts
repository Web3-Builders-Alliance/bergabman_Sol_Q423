import wallet from "../../../../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');
const bundlrUploader = createBundlrUploader(umi);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
    
        const image =
          "https://arweave.net/zcJV5I8_a_AzcLvHwrFedwCl1_vNFw6owhvIpYbGIJ0";
        const metadata = {
          name: "WBArug",
          symbol: "WRUG",
          description: "rugged",
          image,
          attributes: [
            { 
                trait_type: "Quality", 
                value: "High" 
            },
            {
                trait_type: "Rarity",
                value: "Legendary"
            },
          ],
          properties: {
            files: [
              {
                type: "image/png",
                uri:
                  "https://arweave.net/zcJV5I8_a_AzcLvHwrFedwCl1_vNFw6owhvIpYbGIJ0",
              },
            ],
          },
          creators: [{ address: keypair.publicKey, share: 100 }],
        };
        const myUri = await bundlrUploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
      } catch (error) {
        console.log("Oops.. Something went wrong", error);
      }
})();