use warp::Filter;

const HEADER_XAUTH : &str = "X-Auth";

pub fn check_auth() -> impl Filter<Extract=((),), Error=warp::Rejection>{

  warp::any()
  .and(warp::header::<String>(HEADER_XAUTH))
  .and_then(|xauth: String| async move {
    if !xauth.ends_with(".exp.signature"){
      warp::reply::Rejection("xxx")
    }

    Ok<(), warp::Rejection>
  })
}