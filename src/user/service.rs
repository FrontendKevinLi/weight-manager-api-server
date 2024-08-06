use super::User;
use sqlx::{MySql, Pool};

pub async fn fetch_users(pool: &Pool<MySql>) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "
        SELECT id, username, create_time, update_time FROM user
        ;
        "
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}
