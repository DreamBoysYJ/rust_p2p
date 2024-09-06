use crypto::digest::Digest;
use crypto::sha3::Sha3;
use hex;
use k256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Info {
    private_key: String,
    public_key: String,
    eoa: String,
}

// check existence of keystore file 
pub fn run() {
    if !check_keystore_file() {
        create_keys(); 
    }
    nonce();
}

pub fn check_keystore_file() -> bool {
    if let Ok(mut file) = File::open("src/accounts/key/keystore.json") {
        // if file already exists
        let mut contents = String::new();
        if let Err(err) = file.read_to_string(&mut contents) {
            eprintln!("Failed to read file : {}", err);
            return false;
        }
        let json_contents: Info = match serde_json::from_str(&contents) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to parse JSON : {}", e);
                return false;
            }
        };
        println!(
            "You already have account! \neoa : {}, \npublic_key : {},\nprivate_key :{} ",
            json_contents.eoa, json_contents.public_key, json_contents.private_key
        );
        return true;
    }
    false 
}

/*
   generate public/private key + eoa 
   generate a keystore file in the key folder in JSON format
 */
pub fn create_keys() {

    // create private key
    let private_key: SigningKey = SigningKey::random(&mut OsRng);
    let pk = private_key.to_bytes();

    // create public key
    let public_key: VerifyingKey = VerifyingKey::from(&private_key);
    let public_k = public_key.to_bytes();

    let mut hasher = Sha3::sha3_256();

    hasher.input(&public_k);

    let mut EOA_with_32bytes: [u8; 32] = Default::default();
    hasher.result(&mut EOA_with_32bytes);

    let EOA: [u8; 20] = EOA_with_32bytes[12..32].try_into().unwrap();

    let eoa: String = hex::encode(EOA);

    let public_key = hex::encode(public_k);
    let private_key: String = hex::encode(pk);

    // store on the keystore
    let account_info = Info {
        eoa: eoa.to_string(),
        private_key: private_key.to_string(),
        public_key: public_key.to_string(),
    };

    if let Err(err) = generate_keystore_file(&account_info, "src/accounts/key/keystore.json") {
        eprintln!("Error writing keystore file: {}", err);
    } else {
        println!("created keystore file successfully");
    }
}

fn generate_keystore_file(data: &Info, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_str = serde_json::to_string(data)?;

    let mut file = File::create(file_path)?;

    file.write_all(json_str.as_bytes())?;

    Ok(())
}

pub fn nonce() {
    println!();
}
