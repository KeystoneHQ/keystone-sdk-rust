use alloc::vec::Vec;
use minicbor::Decoder;

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
