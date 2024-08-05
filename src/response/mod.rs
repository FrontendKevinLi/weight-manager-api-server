use serde::Serialize;

#[derive(Serialize)]
pub struct StandardResponse<TData> {
    pub message: String,
    pub data: TData,
}

impl<TData> StandardResponse<TData> {
    pub fn success(data: TData) -> axum::Json<StandardResponse<TData>> {
        axum::Json(StandardResponse {
            message: String::from("success"),
            data,
        })
    }

    pub fn failed(err: sqlx::Error, data: TData) -> axum::Json<StandardResponse<TData>> {
        axum::Json(StandardResponse {
            message: err.to_string(),
            data,
        })
    }
}
