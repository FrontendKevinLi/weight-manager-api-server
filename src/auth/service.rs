use super::AuthPayload;
use crate::user::User;
use sqlx::{MySql, Pool};

pub async fn verify_user(
    pool: &Pool<MySql>,
    auth_payload: &AuthPayload,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "
        SELECT id, username, email, create_time, update_time FROM user 
        WHERE email = ? AND password = ?
        ",
        auth_payload.email,
        auth_payload.password
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
