use super::{CreateWeightRecord, WeightRecord};
use sqlx::{Executor, MySql, Pool};

pub async fn fetch_weight_records(pool: &Pool<MySql>) -> Result<Vec<WeightRecord>, sqlx::Error> {
    let weight_records = sqlx::query_as!(
        WeightRecord,
        "SELECT id, weight, date, create_time, update_time FROM weight_record;"
    )
    .fetch_all(pool)
    .await?;

    Ok(weight_records)
}

// pub async fn insert_weight_record(
//     pool: &Pool<MySql>,
//     user_id: u64,
//     weight_record: CreateWeightRecord,
// ) -> Result<u64, sqlx::Error> {
//     let mut transaction = pool.begin().await?;

//     let insert_weight_record_result = sqlx::query!(
//         "
//         INSERT INTO weight_record (weight, date)
//         VALUES (?, ?)
//         ",
//         weight_record.weight,
//         weight_record.date
//     )
//     .execute(&mut *transaction)
//     .await?;

//     let user_weight_record_last_insert_id = insert_user_weight_record(
//         &mut *transaction,
//         user_id,
//         insert_weight_record_result.last_insert_id(),
//     )
//     .await?;

//     transaction.commit().await?;

//     Ok(user_weight_record_last_insert_id)
// }

pub async fn insert_weight_record<'a, E>(
    executor: E,
    weight_record: CreateWeightRecord,
) -> Result<u64, sqlx::Error>
where
    E: Executor<'a, Database = MySql>,
{
    let insert_weight_record_result = sqlx::query!(
        "
        INSERT INTO weight_record (weight, date) 
        VALUES (?, ?)
        ",
        weight_record.weight,
        weight_record.date
    )
    .execute(executor)
    .await?;

    Ok(insert_weight_record_result.last_insert_id())
}
