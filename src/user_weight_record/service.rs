use super::UserWeightRecord;
use sqlx::{Executor, MySql, Pool};

pub async fn fetch_all(pool: &Pool<MySql>) -> Result<Vec<UserWeightRecord>, sqlx::Error> {
    let user_weight_records = sqlx::query_as!(
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
        "
    )
    .fetch_all(pool)
    .await?;

    Ok(user_weight_records)
}

pub async fn fetch_by_id(
    pool: &Pool<MySql>,
    id: u64,
) -> Result<Vec<UserWeightRecord>, sqlx::Error> {
    let user_weight_records = sqlx::query_as!(
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
        where user_weight_record.id = ?
        ",
        id
    )
    .fetch_all(pool)
    .await?;

    Ok(user_weight_records)
}

pub async fn insert_user_weight_record<'a, E>(
    executor: E,
    user_id: u64,
    weight_record_id: u64,
) -> Result<u64, sqlx::Error>
where
    E: Executor<'a, Database = MySql>,
{
    let result = sqlx::query!(
        "
        insert into user_weight_record (user_id, weight_record_id)
        values (?, ?)
        ",
        user_id,
        weight_record_id
    )
    .execute(executor)
    .await?;

    Ok(result.last_insert_id())
}
