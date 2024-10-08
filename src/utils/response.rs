use serde::Serialize;

/// Standard API response structure
#[derive(Debug, Serialize)]
pub struct StandardResponse<T>
where
    T: Serialize,
{
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> StandardResponse<T>
where
    T: Serialize,
{
    /// Creates a new `StandardResponse` with data
    pub fn new_success(message: String, data: T) -> Self {
        Self {
            code: 200,
            message,
            data: Some(data),
        }
    }

    /// Creates a new `StandardResponse` without data
    pub fn new_success_no_data(message: String) -> Self {
        Self {
            code: 200,
            message,
            data: None,
        }
    }

    /// Creates a new `StandardResponse` for errors
    pub fn new_error(code: u16, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}