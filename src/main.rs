mod inventory;

use warp::Filter; 
use crate::inventory::inventory_filter;

const TEMPLATE_DIR : &str = "templates/";

#[tokio::main]
async fn main(){
    let hello_world = warp::path::end()
    .and(warp::get()).map(|| "Hello world");

    let _check_status = warp::path("status")
    .and(warp::get()).map(|| "status Ok");

    let content = warp::fs::dir(TEMPLATE_DIR);

    let root = warp::get()
    .and(warp::path::end()).and(warp::fs::file(format!("{}/index.html", TEMPLATE_DIR)));

    let api_routes = hello_world.or(inventory_filter());

    let static_routes = content.or(root);

    let general_routes = api_routes.or(static_routes);

    warp::serve(general_routes).run(([127,0,0,1], 7800)).await;

}