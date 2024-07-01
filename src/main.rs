use ethers_core::utils::to_checksum;
use ethers_signers::{LocalWallet, Signer};
use std::str::FromStr;

fn main() {
    let mut private_key = std::env::args().nth(1).unwrap_or_default();
    if private_key.len() == 0 {
        std::io::stdin()
            .read_line(&mut private_key)
            .expect("Failed to read private key!");

        private_key = private_key.trim().to_string();
    }

    private_key = private_key.replace("\"", "");

    let wallet = LocalWallet::from_str(private_key.as_str()).expect("Invalid private key!");

    println!("Address: {}", to_checksum(&wallet.address(), None));
}
