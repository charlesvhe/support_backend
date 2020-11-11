use actix_web::{HttpResponse, web};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{bind_data_type, bind_data_type_as, DataType, execute_response, fetch_response, Pool};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConfigMeta {
    pub id: Option<i64>,
    pub app_id: Option<String>,
    pub code: Option<String>,
    pub property: Option<String>,
    pub column_name: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i64>,
    pub gmt_create: Option<DateTime<Utc>>,
    pub gmt_modified: Option<DateTime<Utc>>,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    println!("config_meta config");

    cfg.service(
        web::resource("/config_meta")
            .route(web::post().to(post))
            .route(web::get().to(get_all)),
    );
    cfg.service(
        web::resource("/config_meta/{id}")
            .route(web::delete().to(delete))
            .route(web::put().to(put))
            .route(web::get().to(get_one)),
    );
}

async fn post(config_meta: web::Json<ConfigMeta>, pool: web::Data<Pool>) -> HttpResponse {
    let mut sql = String::from("INSERT INTO config_meta (");
    let mut values = String::from(" VALUES (");

    let mut binds: Vec<DataType> = Vec::new();

    if let Some(v) = &config_meta.app_id {
        sql.push_str(" app_id,");
        values.push_str(" ?,");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.code {
        sql.push_str(" code,");
        values.push_str(" ?,");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.property {
        sql.push_str(" property,");
        values.push_str(" ?,");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.column_name {
        sql.push_str(" column_name,");
        values.push_str(" ?,");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.description {
        sql.push_str(" description,");
        values.push_str(" ?,");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.sort {
        sql.push_str(" sort,");
        values.push_str(" ?,");
        binds.push(DataType::Integer(v));
    }
    if let Some(v) = &config_meta.gmt_create {
        sql.push_str(" gmt_create,");
        values.push_str(" ?,");
        binds.push(DataType::DateTime(v));
    }
    if let Some(v) = &config_meta.gmt_modified {
        sql.push_str(" gmt_modified,");
        values.push_str(" ?,");
        binds.push(DataType::DateTime(v));
    }

    sql.remove(sql.len() - 1);
    sql.push(')');

    values.remove(values.len() - 1);
    values.push(')');

    sql.push_str(values.as_str());

    let mut query = sqlx::query(sql.as_str());
    for bind in binds {
        query = bind_data_type(bind, query)
    }

    execute_response(query.execute(pool.get_ref()).await)
}

async fn delete(id: web::Path<i64>, pool: web::Data<Pool>) -> HttpResponse {
    execute_response(
        sqlx::query("DELETE FROM config_meta WHERE id = ?")
            .bind(id.into_inner()).execute(pool.get_ref()).await
    )
}

async fn put(id: web::Path<i64>, config_meta: web::Json<ConfigMeta>, pool: web::Data<Pool>) -> HttpResponse {
    let mut sql = String::from("UPDATE config_meta SET ");

    let mut binds: Vec<DataType> = Vec::new();

    if let Some(v) = &config_meta.app_id {
        sql.push_str(" app_id=?,");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.code {
        sql.push_str(" code=?,");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.property {
        sql.push_str(" property=?,");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.column_name {
        sql.push_str(" column_name=?,");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.description {
        sql.push_str(" description=?,");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.sort {
        sql.push_str(" sort=?,");
        binds.push(DataType::Integer(v));
    }
    if let Some(v) = &config_meta.gmt_create {
        sql.push_str(" gmt_create=?,");
        binds.push(DataType::DateTime(v));
    }
    if let Some(v) = &config_meta.gmt_modified {
        sql.push_str(" gmt_modified=?,");
        binds.push(DataType::DateTime(v));
    }

    sql.remove(sql.len() - 1);
    sql.push_str(" WHERE id=?");

    let mut query = sqlx::query(sql.as_str());
    for bind in binds {
        query = bind_data_type(bind, query)
    }

    query = query.bind(id.into_inner());

    execute_response(query.execute(pool.get_ref()).await)
}

async fn get_all(config_meta: web::Query<ConfigMeta>, pool: web::Data<Pool>) -> HttpResponse {
    let mut sql = String::from("SELECT * FROM config_meta WHERE 1=1 ");

    let mut binds: Vec<DataType> = Vec::new();

    if let Some(v) = &config_meta.app_id {
        sql.push_str(" AND app_id=?");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.code {
        sql.push_str(" AND code=?");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.property {
        sql.push_str(" AND property=?");
        binds.push(DataType::String(v));
    }
    if let Some(v) = &config_meta.column_name {
        sql.push_str(" AND column_name=?");
        binds.push(DataType::String(v));
    }
    let mut keyword = String::new();
    if let Some(v) = &config_meta.description {
        sql.push_str(" AND description like ?");
        keyword.push_str("%");
        keyword.push_str(v);
        keyword.push_str("%");
        binds.push(DataType::String(&keyword));
    }

    let mut query = sqlx::query_as::<_, ConfigMeta>(sql.as_str());
    for bind in binds {
        query = bind_data_type_as(bind, query)
    }

    fetch_response(query.fetch_all(pool.get_ref()).await)
}

async fn get_one(id: web::Path<i64>, pool: web::Data<Pool>) -> HttpResponse {
    fetch_response(
        sqlx::query_as::<_, ConfigMeta>("SELECT * FROM config_meta WHERE id = ?")
            .bind(id.into_inner()).fetch_one(pool.get_ref()).await
    )
}


