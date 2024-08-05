use super::InsertEmployee;
use crate::employees::Employee;
use sqlx::{MySql, Pool};

pub async fn fetch_employees(pool: &Pool<MySql>) -> Result<Vec<Employee>, sqlx::Error> {
    let employees: Vec<Employee> = sqlx::query_as!(
            Employee,
            "SELECT employee_id, first_name, last_name, job_title, email, hire_date, salary FROM employees;"
        )
        .fetch_all(pool)
        .await?;

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

pub async fn insert_employee(
    pool: &Pool<MySql>,
    employee: InsertEmployee,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO employees (first_name, last_name, email, hire_date, job_title, salary) VALUES (?, ? ,? ,? ,? ,?);",
    )
        .bind(employee.first_name)
        .bind(employee.last_name)
        .bind(employee.email)
        .bind(employee.hire_date)
        .bind(employee.job_title)
        .bind(employee.salary)
        .execute(pool)
        .await?;

    Ok(result.last_insert_id())
}

pub async fn put_employee(
    pool: &Pool<MySql>,
    employee: InsertEmployee,
    user_id: i64,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "
            UPDATE todos.employees 
            SET first_name = ?, last_name = ?, email = ?, hire_date = ?, job_title = ?, salary = ?
            WHERE employee_id = ?
            ;",
    )
    .bind(employee.first_name)
    .bind(employee.last_name)
    .bind(employee.email)
    .bind(employee.hire_date)
    .bind(employee.job_title)
    .bind(employee.salary)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
