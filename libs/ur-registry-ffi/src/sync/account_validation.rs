use anyhow::{ensure, Result};
use ur_registry::{crypto_hd_key::CryptoHDKey, crypto_key_path::CryptoKeyPath};

pub fn allowed_path(path: &CryptoKeyPath) -> bool {
    let components = path.get_components();
    let mut values = Vec::new();
    for c in components {
        let Some(i) = c.get_index() else { return false };
        if i >= 0x80000000 { return false; }
        values.push((i, c.is_hardened()));
    }
    match values.as_slice() {
        [(p, true), (c, true), (0, true)] if [44,49,84,86].contains(p) && [0,1].contains(c) => true,
        [(44, true), (c, true), (0, true)] if [60,607,195,144,3,145,5,133,931].contains(c) => true,
        [(p, true), (2, true), (0, true)] if [49,84].contains(p) => true,
        [(44, true), (60, true), (0, true), (0, false)] => true,
        [(44, true), (60, true), (a, true), (0, false), (0, false)] if *a <= 9 => true,
        [(44, true), (501, true)] => true,
        [(44, true), (501, true), (a, true)] if *a <= 9 => true,
        [(44, true), (501, true), (a, true), (0, true)] if *a <= 9 => true,
        [(44, true), (c, true), (a, true), (0, true), (0, true)] if [784,637,4218].contains(c) && *a <= 9 => true,
        [(1852, true), (1815, true), (a, true)] if *a <= 23 => true,
        [(44, true), (148, true), (a, true)] if *a <= 4 => true,
        [(44, true), (c, true), (0, true), (0, false), (i, false)] if [118,529,394,234,564,459,60,330].contains(c) && *i <= 999999999 => true,
        _ => false,
    }
}

pub fn validate_key(key: &CryptoHDKey, fingerprint: Option<[u8; 4]>) -> Result<()> {
    ensure!(!key.is_master() && !key.is_private_key(), "invalid account");
    let origin = key.get_origin().ok_or_else(|| anyhow::anyhow!("invalid account"))?;
    ensure!(allowed_path(&origin), "account path is not allowed");
    if let (Some(expected), Some(actual)) = (fingerprint, origin.get_source_fingerprint()) {
        ensure!(expected == actual, "invalid account");
    }
    let public = key.get_key();
    ensure!(public.len() == 32 || (public.len() == 33 && matches!(public[0], 2 | 3)), "invalid account");
    if let Some(chain) = key.get_chain_code() {
        ensure!(chain.len() == 32, "invalid account");
        ensure!(public.len() == 33 || key.get_parent_fingerprint().is_none(), "invalid account");
    }
    if let Some(depth) = origin.get_depth() { ensure!(depth <= 255, "invalid account"); }
    if let Some(name) = key.get_name() { validate_text(&name, 128)?; }
    if let Some(note) = key.get_note() { validate_text(&note, 1024)?; }
    Ok(())
}

pub fn validate_text(text: &str, limit: usize) -> Result<()> {
    ensure!(text.len() <= limit && !text.chars().any(|c| c < ' ' || c == '\u{7f}'), "invalid account");
    Ok(())

}
