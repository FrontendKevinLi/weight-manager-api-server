use sqlx::{MySql, Pool};
use crate::employees::Employee;

pub async fn fetch_employees(pool: &Pool<MySql>) -> Result<Vec<Employee>, sqlx::Error> {
    let employees: Vec<Employee> = sqlx::query_as!(
            Employee,
            "SELECT employee_id, first_name, last_name, job_title, email, hire_date, salary FROM employees;"
        )
        .fetch_all(pool)
        .await?;

    // let employees: Vec<Employee> = sqlx::query_as!(Employee, "SELECT * FROM employees;")
    //     .fetch_all(pool)
    //     .await?;

    Ok(employees)
}

pub async fn fetch_employee_by_id(pool: &Pool<MySql>, id: i64) -> Result<Employee, sqlx::Error> {
  let employee: Employee = sqlx::query_as!(
      Employee,
      "SELECT employee_id, first_name, last_name, job_title, email, hire_date, salary FROM employees WHERE employee_id =?;",
      id
  )
  .fetch_one(pool)
  .await?;

  Ok(employee)
}

// pub async fn insert_employee(pool: &Pool<MySql>) -> Result<i64, sqlx::Error> {
//     let sqlx::query!(
//         "INSERT INTO employees (first_name, last_name, email, hire_date, job_title, salary) VALUES (?, ? ,? ,? ,? ,?);",
//         &"Test",
//         &"name",
//         &"test.name@example.com",
//         &"2021-01-12",
//         &"Test Job",
//         123.45
//     )
// }
