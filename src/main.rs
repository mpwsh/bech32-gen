use anyhow::Result;
use bech32::{self, convert_bits, u5, Variant};
use rand::Rng;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bech32 decoding error: {0}")]
    Bech32DecodingError(#[from] bech32::Error),
    #[error("Bit conversion error")]
    BitConversionError,
    #[error("Hex decoding error: {0}")]
    HexDecodingError(#[from] hex::FromHexError),
}

fn main() -> Result<()> {
    let size = 10 * 1024 / 8;
    let mut rng = rand::thread_rng();

    let bytes: Vec<u8> = (0..size).map(|_| rng.gen()).collect();

    // Convert bytes to hexadecimal string
    let hex = hex::encode(bytes);

    let encoded = encode_puzzle_hash(&hex, "offer")?;
    println!("{encoded}");
    println!("Valid bech32: {}", bech32::decode(&encoded).is_ok());
    Ok(())
}
pub fn encode_puzzle_hash(puzzle_hash: &str, prefix: &str) -> Result<String, Error> {
    let bytes = hex::decode(puzzle_hash.replace("0x", ""))?;

    // Convert bytes (Vec<u8>) to 5-bit words
    // Here, we can pass `&bytes`, which is a slice, directly to convert_bits
    let bits: Vec<u5> = convert_bits(&bytes, 8, 5, true)?
        .iter()
        .map(|b| u5::try_from_u8(*b).map_err(|_| Error::BitConversionError))
        .collect::<Result<Vec<u5>, Error>>()?;
    let encoded = bech32::encode(prefix, bits, Variant::Bech32m)?;
    Ok(encoded)
}
