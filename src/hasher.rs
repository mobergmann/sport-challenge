use argon2::password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};
use argon2::Argon2;

pub fn hash(password: &String) -> String {
    // Salt should be unique per password
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.clone().into_bytes().as_slice(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify(hash: &String, password: &String) -> bool {
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    Argon2::default()
        .verify_password(password.clone().into_bytes().as_slice(), &parsed_hash)
        .is_ok()
}
