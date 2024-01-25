use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer};
use crate::util::basic_helper;

#[allow(dead_code)]
// generate new wallet and get its public key
pub fn generate_new_wallet() -> Pubkey {
    let keypair = Keypair::new();

    // get base58 secret key
    // let secret_key_bytes = keypair.secret().to_bytes();
    // let base58_secret_key = solana_sdk::bs58::encode(secret_key_bytes).into_string();

    Signer::pubkey(&keypair)
}

// getting sol balance
pub fn get_balance(pub_key: &Pubkey) -> u64 {
    let client = basic_helper::get_client();

    match client.get_balance(pub_key) {
        Ok(balance) => {
            balance / LAMPORTS_PER_SOL
        },
        Err(_) => {
            println!("Error getting balance");
            0
        },
    }
}

// airdropping sols
pub fn get_sols(pub_key: &Pubkey) {
    let client = basic_helper::get_client();

    // get sols (this are used for transactions)
    match client.request_airdrop(&pub_key, LAMPORTS_PER_SOL * 5) {
        Ok(sig) => loop {
            if let Ok(confirmed) = client.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(_) => println!("Error requesting airdrop"),
    };
}

// transfer sols
pub fn transfer_sols(
    from_pubkey: &Pubkey,
    from_secret: &str,
    to_pubkey: &Pubkey,
    lamports_to_send: u64,
) {
    // get client
    let client = basic_helper::get_client();

    // get instruction that will passed to the transaction
    let ix = basic_helper::prepare_instruction(from_pubkey, to_pubkey, lamports_to_send);

    // get keypair from the sender's secret key
    let keypair = basic_helper::get_keypair(from_secret);

    // add prepared instruction to a transaction
    let txn = basic_helper::prepare_transaction(ix, from_pubkey, keypair, &client);

    // sending the transfer sol transaction
    match client.send_and_confirm_transaction(&txn) {
        Ok(sig) => loop {
            if let Ok(confirmed) = client.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(e) => println!("Error transferring Sol:, {}", e),
    };

}