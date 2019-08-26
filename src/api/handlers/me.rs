use warp::http::Response;

pub fn me() -> impl warp::Reply {
    Response::builder().body("me")
}
