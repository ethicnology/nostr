// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::str::FromStr;

use nostr::prelude::*;

fn main() -> Result<()> {
    let public_key =
        PublicKey::from_str("b2d670de53b27691c0c3400225b65c35a26d06093bcc41f48ffc71e0907f9d4a")?;

    if nip05::verify_blocking(&public_key, "0xtr@oxtr.dev", None)? {
        println!("NIP-05 verified");
    } else {
        println!("NIP-05 NOT verified");
    }

    let profile = nip05::get_profile_blocking("_@fiatjaf.com", None)?;
    println!("Profile: {}", profile.to_bech32()?);

    Ok(())
}
