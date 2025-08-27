use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use rand::rngs::OsRng;

/// Argon2id configured with no secret/AD; safe to return as 'static.
fn argon2id() -> Argon2<'static> {
    // ~19 MiB, 2 iters, 1 lane — tune for your hardware
    let params = Params::new(19 * 1024, 2, 1, None).expect("valid Argon2 params");
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

/// Produce a PHC-formatted Argon2id hash for storage.
pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2id();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

/// Verify plaintext against a PHC-formatted Argon2 hash.
pub fn verify_password(password: &str, phc_hash: &str) -> Result<bool, password_hash::Error> {
    let parsed = PasswordHash::new(phc_hash)?;
    let argon2 = argon2id();
    Ok(argon2.verify_password(password.as_bytes(), &parsed).is_ok())
}

