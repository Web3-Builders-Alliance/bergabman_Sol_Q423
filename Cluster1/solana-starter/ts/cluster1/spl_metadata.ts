import wallet from "../../../../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { PublicKey } from "@solana/web3.js";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
// Define our Mint address
const mint = new PublicKey("GevkuerDqH7vna9b7ShTXP4vLXv4h4qGibCnmdvY5GA4");

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

// Add the Token Metadata Program
const token_metadata_program_id = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

// Create PDA for token metadata
const metadata_seeds = [
    Buffer.from("metadata"),
    token_metadata_program_id.toBuffer(),
    mint.toBuffer(),
];
const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(
    metadata_seeds,
    token_metadata_program_id
);

(async () => {
    try {
      const umi = createUmi(RPC_ENDPOINT);
  
      let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
      const myKeypairSigner = createSignerFromKeypair(umi, keypair);
  
      umi.use(signerIdentity(myKeypairSigner));
  
      let myTransaction = createMetadataAccountV3(umi, {
        //accounts
        metadata: publicKey(metadata_pda.toString()),
        mint: publicKey(mint.toString()),
        mintAuthority: myKeypairSigner,
        payer: myKeypairSigner,
        updateAuthority: keypair.publicKey,
        data: {
          name: "test",
          symbol: "tst",
          uri: "example_test.com",
          sellerFeeBasisPoints: 0,
          creators: null,
          collection: null,
          uses: null,
        },
        isMutable: true,
        collectionDetails: null,
      });
  
      let result = await myTransaction.sendAndConfirm(umi);
  
      console.log(myTransaction);
  
      console.log(result);
    } catch (e) {
      console.error(`Oops, something went wrong: ${e}`);
    }
  })();