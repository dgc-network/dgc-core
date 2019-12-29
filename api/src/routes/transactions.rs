// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#![feature(proc_macro_hygiene, decl_macro)]

//#[macro_use] extern crate rocket;
/*
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
*/
extern crate rocket;

//use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use guard::validator_conn::ValidatorConn;
use submit::{submit_batches, check_batch_status, BatchStatus};
use submit::TransactionError as error;

#[derive(FromForm)]
struct TxnQuery {
    wait: u32
}

#[derive(FromForm)]
struct StatusQuery {
    wait: Option<u32>,
    ids: String
}
/*
// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/<id>", format = "json", data = "<message>")]
fn new(id: ID, message: Json<Message>, map: State<'_, MessageMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        json!({
            "status": "error",
            "reason": "ID exists. Try put."
        })
    } else {
        hashmap.insert(id, message.0.contents);
        json!({ "status": "ok" })
    }
}
*/
#[post("/batches?<query>", format = "application/octet-stream", data = "<data>")]
pub fn submit_txns_wait(
    conn: ValidatorConn,
    data: Vec<u8>,
    //query: TxnQuery) -> Result<Custom<Json<Vec<BatchStatus>>>, Custom<Json>> {
    query: TxnQuery) -> Result<Json<Vec<BatchStatus>>, Custom<Json>> {

    let batch_status_list = submit_batches(&mut conn.clone(), &data, query.wait)
        .map_err(map_error)?;

    if batch_status_list
            .iter()
            .all(|x| x.status == "COMMITTED") {

        Ok(Custom(Status::Created, Json(batch_status_list)))
    } else {
        Ok(Custom(Status::Accepted, Json(batch_status_list)))
    }
}

#[post("/batches", format = "application/octet-stream", data = "<data>")]
pub fn submit_txns(
    conn: ValidatorConn, 
    data: Vec<u8>) -> Result<Json<Vec<BatchStatus>>, Custom<Json>> {

    submit_batches(&mut conn.clone(), &data, 0)
        .map_err(map_error)
        .and_then(|b| Ok(Json(b)))
}

#[get("/batch_status?<query>")]
pub fn get_batch_status(
    conn: ValidatorConn,
    query: StatusQuery) -> Result<Json<Vec<BatchStatus>>, Custom<Json>> {

    let wait = query.wait.unwrap_or(0);
    let ids: Vec<String> = query.ids
        .split(",")
        .map(String::from)
        .collect();

    check_batch_status(&mut conn.clone(), ids, wait)
        .map_err(map_error)
        .and_then(|b| Ok(Json(b)))
}

fn map_error(err: error) -> Custom<Json> {
    let message = Json(
        json!({
            "message": format!("{:?}", err)
        })
    );

    match err {
        error::BatchParseError(_) |
        error::InvalidBatch(_) |
        error::NoResource(_) |
        error::InvalidId(_) => Custom(Status::BadRequest, message),
        _ => Custom(Status::InternalServerError, message)
    }
}
