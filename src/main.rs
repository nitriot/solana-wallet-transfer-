// /src/main.rs

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction, transaction::Transaction,
};
use std::str::FromStr;
use dotenv::dotenv;
use std::env;
use std::io::{self, Write};

fn main() {
    dotenv().ok();

    // loads .env vars 
    let key_str = env::var("PRIVATE_KEY").expect("Missing PRIVATE_KEY"); // btw it supports base58 private key. 

    // Ask user input for reciverr pk (public key)
    print!("Enter recipient public key: ");
    io::stdout().flush().unwrap();
    let mut recipient = String::new();
    io::stdin().read_line(&mut recipient).unwrap();
    let recipient = recipient.trim(); // Remove newline

    // Ask user input for amount
    print!("Enter amount in SOL: ");
    io::stdout().flush().unwrap();
    let mut amount_input = String::new();
    io::stdin().read_line(&mut amount_input).unwrap();
    let amount_sol = amount_input
        .trim()
        .parse::<f64>()
        .expect("Invalid amount");

    // lamports 
    let lamports = (amount_sol * 1_000_000_000_f64) as u64;

    // prepare keypair /sender keypair = reciver pk (public key)
    let sender_keypair = Keypair::from_base58_string(&key_str);
    let recipient_pubkey = Pubkey::from_str(&recipient).expect("Invalid recipient");

    // RPC you can use any solana rpc i suggest you to use helius rpc 
    let helius_url = "https://mainnet.helius-rpc.com/?api-key=your helius api key";
    let rpc = RpcClient::new_with_commitment(helius_url.to_string(), solana_sdk::commitment_config::CommitmentConfig::confirmed()); // edit the "confirmed" if you want to change commitment

    // Commitment levels:
    // - processed: Fastest, tx has been seen by a leader but not yet voted on
    // - confirmed: Seen by at least 1 confirmed block (safer)
    // - finalized: Maximum security, confirmed by supermajority of validators
    
    // Get recent blockhash (it get recent blockhash from rpc and sign tx on new block)
    let blockhash = rpc.get_latest_blockhash().expect("Failed to get blockhash");

    // Create transfer instruction
    let ix = system_instruction::transfer(&sender_keypair.pubkey(), &recipient_pubkey, lamports);

    // Create tx (transaction)
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&sender_keypair.pubkey()),
        &[&sender_keypair],
        blockhash,
    );

    // Send transaction to rpc 
    match rpc.send_and_confirm_transaction(&tx) {
        Ok(sig) => println!("✅ Transaction success! Sig: {}", sig),
        Err(err) => eprintln!("❌ Transaction failed: {:?}", err),
    }
}
