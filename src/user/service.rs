use crate::user_weight_record::UserWeightRecord;
use crate::weight_record;
use crate::{user_weight_record, weight_record::CreateWeightRecord};

use super::{CreateUser, DateRange, User};
use crate::password_util;
use sqlx::{MySql, Pool};

pub async fn fetch_users(pool: &Pool<MySql>) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "
        SELECT id, username, email, create_time, update_time FROM user
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
        SELECT id, username, email, create_time, update_time FROM user
        WHERE id = ?
        ",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn is_user_exist(pool: &Pool<MySql>, email: &str) -> Result<bool, sqlx::Error> {
    let is_exist_record = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM user WHERE user.email = ?) as is_exist",
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(is_exist_record.is_exist == 1)
}

pub async fn insert_user(
    pool: &Pool<MySql>,
    argon2_context: &argon2::Argon2<'static>,
    user: CreateUser,
) -> Result<u64, sqlx::Error> {
    //TODO: Change to a correct error
    let hashed_password = password_util::hash(&argon2_context, &user.password)
        .map_err(|_| sqlx::Error::RowNotFound)?;

    let result = sqlx::query!(
        "INSERT INTO user (username, email, password) VALUES (?, ?, ?)",
        user.username,
        user.email,
        hashed_password
    )
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
        "
        UPDATE user 
        SET username = ?, email = ?
        WHERE id = ?
        ",
        user.username,
        user.email,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn fetch_weight_record_by_user_id(
    pool: &Pool<MySql>,
    user_id: u64,
    date_range: DateRange,
) -> Result<Vec<UserWeightRecord>, sqlx::Error> {
    let records = sqlx::query_as!(
        UserWeightRecord,
        "
        select 
            user_weight_record.id as id,
            user.id as user_id,
            user.username as username,
            weight_record.weight as weight,
            weight_record.date as date
        from user_weight_record
        join user on user_weight_record.user_id = user.id
        join weight_record on user_weight_record.weight_record_id = weight_record.id
        where user_weight_record.user_id = ?
        and weight_record.date between ? and ?
        ",
        user_id,
        date_range.start_date,
        date_range.end_date
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn create_weight_record_by_user_id(
    pool: &Pool<MySql>,
    user_id: u64,
    weight_record: CreateWeightRecord,
) -> Result<u64, sqlx::Error> {
    let mut transaction = pool.begin().await?;

    let weight_record_id =
        weight_record::insert_weight_record(&mut *transaction, weight_record).await?;

    let id =
        user_weight_record::insert_user_weight_record(&mut *transaction, user_id, weight_record_id)
            .await?;

    transaction.commit().await?;

    Ok(id)
}
