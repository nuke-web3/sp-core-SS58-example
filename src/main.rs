//! Example generating SS58 Address format from Substrate keys.
//!
//! See also https://polkadot.subscan.io/tools/ss58_transform for easy conversion of addresses from hex, H160, those listed on the registry: https://github.com/paritytech/ss58-registry/blob/main/ss58-registry.json

use sp_core::{
    crypto::{Ss58AddressFormatRegistry, Ss58Codec},
    hexdisplay::HexDisplay,
    sr25519::Pair,
    Pair as _,
};

fn main() {
    println!(
        "---------- Generating a secret key and use as seed for HDKD derived keys ----------\n"
    );

    // Get a new pair & it's mnemonic phrase.
    let (pair, mnemonic, _) = Pair::generate_with_phrase(None);

    // Derive the public key.
    let pk = pair.public();

    println!("Mnemonic: {:?}", &mnemonic);
    println!("Pubkey (hex): {:?}", <HexDisplay>::from(&pk.0));

    println!(
        "Default SS58 Address (AccountId) for Root Public Key: {:?}",
        &pk.to_ss58check_with_version(Ss58AddressFormatRegistry::SubstrateAccount.into())
    );

    println!("\n---------- Generating Polakdot & Kusama derived keys ----------\n");
    // Hard derive new key pair using `//polkadot`.
    let pair_polkadot = Pair::from_string(&format!("{}//polkadot", &mnemonic), None);
    let pk_polkadot = pair_polkadot.unwrap().public();

    // Hard derive new key pair using `//kusama`.
    let pair_kusama = Pair::from_string(&format!("{}//kusama", &mnemonic), None);
    let pk_kusama = pair_kusama.unwrap().public();

    // Access `AccountId`
    println!(
        "SS58 Address (AccountId) for Polkadot Public Key: {:?}",
        &pk_polkadot.to_ss58check_with_version(Ss58AddressFormatRegistry::PolkadotAccount.into())
    );
    println!(
        "SS58 Address (AccountId) for Kusama Public Key: {:?}",
        &pk_kusama.to_ss58check_with_version(Ss58AddressFormatRegistry::KusamaAccount.into())
    );

    println!("\n---------- Cross check with Subkey ----------");
    println!("Install: https://github.com/paritytech/substrate/tree/master/bin/utils/subkey\n");
    
    println!("$ subkey inspect \"<mnemonic phrase>\" ");
    println!("$ subkey inspect --network <polkadot, kusama, etc.> \"<mnemonic phrase>\"");
}
