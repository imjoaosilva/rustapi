use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct JsonResponse<T> {
    pub status: i32,
    pub data: T
}
 
#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String
}