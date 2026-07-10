use alloc::vec::Vec;
use minicbor::Decoder;

/// Decodes a definite length CBOR map with `u8` keys.
pub(super) fn decode_definite_u8_map<'b, T, F>(
    d: &mut Decoder<'b>,
    obj: &mut T,
    message: &'static str,
    mut decode_entry: F,
) -> Result<(), minicbor::decode::Error>
where
    F: FnMut(u8, &mut T, &mut Decoder<'b>) -> Result<(), minicbor::decode::Error>,
{
    let len = d
        .map()?
        .ok_or_else(|| minicbor::decode::Error::message(message).at(d.position()))?;
    for _ in 0..len {
        let key = d.u8()?;
        decode_entry(key, obj, d)?;
    }
    Ok(())
}

pub(super) fn reject_duplicate_key(
    seen_keys: &mut Vec<u8>,
    key: u8,
    d: &Decoder<'_>,
    message: &'static str,
) -> Result<(), minicbor::decode::Error> {
    if seen_keys.contains(&key) {
        return Err(minicbor::decode::Error::message(message).at(d.position()));
    }
    seen_keys.push(key);
    Ok(())
}

pub(super) fn require_key(
    seen_keys: &[u8],
    key: u8,
    d: &Decoder<'_>,
    message: &'static str,
) -> Result<(), minicbor::decode::Error> {
    if seen_keys.contains(&key) {
        Ok(())
    } else {
        Err(minicbor::decode::Error::message(message).at(d.position()))
    }
}
