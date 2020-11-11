extern crate actix_web;

use actix_web::{HttpResponse, web};
use chrono::DateTime;
use serde::Serialize;
use sqlx::{Done, Error, Sqlite};
use sqlx::query::{Query, QueryAs};
use sqlx::sqlite::{SqliteArguments, SqlitePool};
use sqlx::types::chrono::Utc;

// 动态指定Pool
pub type Pool = SqlitePool;

pub mod config {
    pub mod config_meta;
}

pub fn config(cfg: &mut web::ServiceConfig) {
    println!("lib config");
    config::config_meta::config(cfg);
}

pub enum DataType<'a> {
    Integer(&'a i64),
    String(&'a String),
    DateTime(&'a DateTime<Utc>),
}

pub fn bind_data_type<'a>(data_type: DataType<'a>, query: Query<'a, Sqlite, SqliteArguments<'a>>) -> Query<'a, Sqlite, SqliteArguments<'a>> {
    match data_type {
        DataType::Integer(v) => query.bind(v),
        DataType::String(v) => query.bind(v),
        DataType::DateTime(v) => query.bind(v)
    }
}

pub fn bind_data_type_as<'a, O>(data_type: DataType<'a>, query: QueryAs<'a, Sqlite, O, SqliteArguments<'a>>) -> QueryAs<'a, Sqlite, O, SqliteArguments<'a>> {
    match data_type {
        DataType::Integer(v) => query.bind(v),
        DataType::String(v) => query.bind(v),
        DataType::DateTime(v) => query.bind(v)
    }
}

pub fn fetch_response<T: Serialize>(result: Result<T, Error>) -> HttpResponse {
    match result {
        Ok(o) => HttpResponse::Ok().json(o),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e))
    }
}

pub fn execute_response<T: Done>(result: Result<T, Error>) -> HttpResponse {
    match result {
        Ok(o) => HttpResponse::Ok().json(o.rows_affected()),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e))
    }
}
