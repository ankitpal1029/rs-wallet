mod eth_wallet;
fn main() {
    let (secret_key, pub_key) = eth_wallet::generate_keypair();

    println!("Secret key: {}", secret_key.to_string());
    println!("Public key: {}", pub_key.to_string());
}
