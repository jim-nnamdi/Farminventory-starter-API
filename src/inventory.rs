use warp::Filter; 
use serde_json::{json, Value};
use warp::reply::Json;

pub fn inventory_filter() -> impl Filter<Extract = impl warp::Reply, Error=warp::Rejection> + Clone {

  let inventory_base_path = warp::path("inventory");

  let inventorylist = inventory_base_path
  .and(warp::get())
  .and(warp::path::end())
  .and_then(inventory_list);

  let singleinventory = inventory_base_path
  .and(warp::get())
  .and(warp::path::param())
  .and_then(single_inventory);

  let createinventory = inventory_base_path
  .and(warp::post())
  .and(warp::body::json())
  .and_then(create_inventory);

  inventorylist.or(singleinventory).or(createinventory)
}

async fn inventory_list() -> Result<Json, warp::Rejection>{
  
  let inventory = json!([
    {"id": 1, "farm_id": 1, "product_count": 234, "product_price_total":300,    "status": "high", "updated_by": "midwestmetro", "farm_active": true},
    {"id": 2, "farm_id": 2, "product_count": 235, "product_price_total":301, "status": "high", "updated_by": "midwestmetro", "farm_active": false}
  ]);

  let inventory = warp::reply::json(&inventory);

  Ok(inventory)
}

async fn single_inventory(id: i64) -> Result<Json, warp::Rejection>{
  let inventory = json!([
    {"id": 1, "farm_id": format!("{}", id), "product_count": 234, "product_price_total":300, "status": "high", "updated_by": "midwestmetro", "farm_active": true},
  ]);

  let inventory = warp::reply::json(&inventory);

  Ok(inventory)
}

async fn create_inventory(data: Value) -> Result<Json, warp::Rejection>{
  let inventory = data;

  let inventory = warp::reply::json(&inventory);

  Ok(inventory)
}