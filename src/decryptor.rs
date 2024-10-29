use aes::Aes128;
use block_modes::block_padding::NoPadding;
use block_modes::{BlockMode, Cbc};
use pbkdf2::pbkdf2_hmac_array;
use sha1::Sha1;

type Aes128Cbc = Cbc<Aes128, NoPadding>;

#[derive(Debug)]
pub struct UnixCookieDecryptor {
    rounds: u32,
}

impl UnixCookieDecryptor {
    pub fn new(rounds: u32) -> Self {
        Self { rounds }
    }

    pub fn decrypt(
        &self,
        mut value: Vec<u8>,
        key: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Generate derived key using PBKDF2
        let derived_key: [u8; 16] = pbkdf2_hmac_array::<Sha1, 16>(key, b"saltysalt", self.rounds);

        // Create initialization vector filled with spaces
        let iv: [u8; 16] = [b' '; 16];

        // Create cipher instance
        let cipher = Aes128Cbc::new_from_slices(&derived_key, &iv)?;

        // Decrypt the value
        let decrypted = cipher.decrypt(&mut value)?;

        // Get the number of padding bytes to remove from the last byte
        let padding_len = *decrypted.last().ok_or("Empty decrypted data")? as usize;

        // Remove padding
        if padding_len > decrypted.len() {
            return Err("Invalid padding length".into());
        }
        let decrypted = &decrypted[..decrypted.len() - padding_len];
        let pattern = b"xoxd-";
        let index = decrypted
            .windows(pattern.len())
            .position(|window| window == pattern)
            .unwrap();
        Ok(decrypted[index..].to_vec())
    }
}
