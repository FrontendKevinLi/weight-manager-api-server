use super::AuthPayload;
use crate::{password_util};
use sqlx::{MySql, Pool};

pub async fn verify_user(
    pool: &Pool<MySql>,
    argon2_context: &argon2::Argon2<'static>,
    auth_payload: &AuthPayload,
) -> Result<AuthPayload, sqlx::Error> {
    let user = sqlx::query_as!(
        AuthPayload,
        "
        SELECT email, password FROM user 
        WHERE email = ?
        ",
        auth_payload.email
    )
    .fetch_one(pool)
    .await?;

    dbg!("{}", &user.password);

    if user.password.starts_with("$argon2")
        && password_util::verify(argon2_context, &auth_payload.password).is_ok()
    {
        return Ok(user);
    }

    Ok(user)
}
