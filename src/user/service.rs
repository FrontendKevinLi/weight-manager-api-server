use super::{CreateUser, User};
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

pub async fn fetch_users_by_id(pool: &Pool<MySql>, id: i64) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "
        SELECT id, username, create_time, update_time FROM user
        WHERE id = ?
        ",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn insert_user(pool: &Pool<MySql>, user: CreateUser) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("INSERT INTO user (username) VALUES (?)", user.username)
        .execute(pool)
        .await?;

    Ok(result.last_insert_id())
}

pub async fn update_user(
    pool: &Pool<MySql>,
    user: CreateUser,
    id: u64,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE user SET username = ? WHERE id = ?",
        user.username,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
