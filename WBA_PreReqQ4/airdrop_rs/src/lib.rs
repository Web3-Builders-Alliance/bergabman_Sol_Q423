pub mod programs;

#[cfg(test)]
mod tests {
    // use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
    use solana_sdk;
    use bs58;
    use std::io::{self, BufRead};
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::signature::{Keypair, Signer, read_keypair_file};

    use solana_program::{
    pubkey::Pubkey,
    system_instruction::transfer, system_program,
    };
    use solana_sdk::transaction::Transaction;
    use std::str::FromStr;

    use solana_sdk::message::Message;
    use crate::programs::wba_prereq::{WbaPrereqProgram, CompleteArgs, UpdateArgs};


    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        // 3xy4HcfmAWxbthd1XWaMXk1SHXpfKQno8eNzMrntrx1K
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

        #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet =
        stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',')
            .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
    
    #[test]
    fn airdop() {
        let keypair = read_keypair_file("../../../../dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        };
    }
    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("../../../../dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("bergNvF6e4qZ9dJDYHBBhC9r644Mg5S2rB7PdDC3USH").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                1_000_000_00
            )],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
            );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here:
        https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
        )

    }

    #[test]
    fn empty_dev_w() {
        let keypair = read_keypair_file("../../../../dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("bergNvF6e4qZ9dJDYHBBhC9r644Mg5S2rB7PdDC3USH").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        let message = Message::new_with_blockhash(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                balance,
            )],
            Some(&keypair.pubkey()),
            &recent_blockhash
            );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                balance - fee,
            )],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
            );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here:
        https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
        )

    }
    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
        let signer = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet file");
        let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
        let args = CompleteArgs {
            github: b"bergabman".to_vec()
        };

        let transaction = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);



    }

}
