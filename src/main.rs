#![allow(unused)]
use ed25519_dalek::{SigningKey, Signature, Signer, Verifier, VerifyingKey};
use rand::rngs::OsRng;


fn main() {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let public_key = signing_key.verifying_key();

    let payload = b"Hello, world!";
    let signature = signing_key.sign(payload);
    
    // output results
    let public_key_bytes = public_key.to_bytes();
    let signature_bytes = signature.to_bytes();

    println!("Public Key: {:?}", public_key_bytes);
    println!("Public Key(hex): {:?}", hex::encode(public_key_bytes));
    println!("Signature: {:?}", signature_bytes);
    println!("Signature(hex): {:?}", hex::encode(signature_bytes));

    // check auth
    assert!(check_auth(public_key, payload, &signature));

}

fn check_auth(public_key: VerifyingKey, payload: &[u8], signature: &Signature) -> bool {
    public_key.verify(payload, signature).is_ok()
}