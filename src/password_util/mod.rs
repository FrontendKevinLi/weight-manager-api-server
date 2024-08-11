use argon2::{PasswordHasher, PasswordVerifier};

pub fn generate_argon2_context() -> argon2::Argon2<'static> {
    let argon2_params = argon2::ParamsBuilder::new()
        .m_cost((2 as u32).pow(14))
        .t_cost(2)
        .p_cost(3)
        .data(argon2::AssociatedData::new(&[0x0f; 6]).unwrap())
        .keyid(argon2::KeyId::new(&[0xf0; 4]).unwrap())
        .output_len(16)
        .build()
        .unwrap();

    argon2::Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2_params,
    )
}

pub fn hash(
    argon2_context: &argon2::Argon2<'static>,
    password: &str,
) -> Result<String, argon2::password_hash::Error> {
    let salt_string = argon2::password_hash::SaltString::generate(rand::thread_rng());

    let hashed_password = argon2_context.hash_password(password.as_bytes(), &salt_string)?;

    Ok(hashed_password.to_string())
}

pub fn verify(
    argon2_context: &argon2::Argon2<'static>,
    hashed_password: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let hash = argon2::PasswordHash::new(hashed_password)?;

    let verify_result = argon2_context.verify_password(hashed_password.as_bytes(), &hash);

    match verify_result {
        Ok(_) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(_) => Err(argon2::password_hash::Error::Password),
    }
}
