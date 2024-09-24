use serde::Serialize;
#[derive(Serialize)]
pub struct IdResponse {
    pub id: i32,
}

#[derive(Serialize)]
pub enum Status {
    SUCCESS,
    FAIL,
}

#[derive(Serialize)]
pub struct Response<T> {
    pub status: Status,
    pub code: i64,
    pub message: String,
    pub data: Option<T>,
}

pub fn json<T: Serialize>(res_body: T) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .json(res_body)
}

pub fn success_json<T: Serialize>(data: T) -> actix_web::HttpResponse {
    json(Response {
        status: Status::SUCCESS,
        code: 200,
        message: "".to_string(),
        data: Some(data),
    })
}