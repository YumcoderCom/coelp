// Copyright 2023-present The Yumcoder Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
//
// Author: yumcoder (omid.jn@gmail.com)
//
use ed25519_dalek::{
    pkcs8::{self, spki::der::pem::LineEnding, EncodePrivateKey},
    SigningKey,
};
use rand::rngs::OsRng;

pub fn open_sshprivate_key() -> Result<String, pkcs8::Error> {
    let signing_key = SigningKey::generate(&mut OsRng);
    let zeroizing_string = signing_key.to_pkcs8_pem(LineEnding::default())?;
    Ok(zeroizing_string.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_sshprivate_key() {
        // Call the function and check if it returns a result without errors
        let result = open_sshprivate_key();

        // Check if the result is Ok
        assert!(result.is_ok());

        // You can also further inspect the content if needed
        let private_key_pem = result.unwrap();
        // Check if the private key PEM starts with the expected string
        assert!(
            private_key_pem.starts_with("-----BEGIN PRIVATE KEY-----"),
            "Unexpected private key PEM format"
        );
        // Check if the private key PEM ends with the expected string
        assert!(
            private_key_pem.ends_with(format!(
                "-----END PRIVATE KEY-----{:?}",
                LineEnding::default()
            ).as_str()),            
            "Unexpected private key PEM format (end)"
        );
        // println!("Private Key PEM:\n{}", private_key_pem);
    }
}
