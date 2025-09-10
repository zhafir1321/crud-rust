use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncMysqlConnection, AsyncConnection};
use diesel_async::pooled_connection::bb8::Pool;
use diesel::dsl::last_insert_id;

use crate::models::{Product, NewProduct};
use crate::schema::products;

// Type alias for the connection pool
type DbPool = Pool<AsyncMysqlConnection>;

// Handler to create a new product
pub async fn create_product(
    pool: web::Data<DbPool>,
    new_product: web::Json<NewProduct>,
) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let product_data = new_product.into_inner();

    // Transaction to ensure atomicity: insert, get last ID, then fetch the new record.
    let result = conn.transaction(|conn| Box::pin(async move {
        // Execute the insert
        diesel::insert_into(products::table)
            .values(&product_data)
            .execute(conn)
            .await?;

        // Get the last inserted ID (MySQL specific)
        let last_id_result = diesel::select(last_insert_id).get_result::<u64>(conn).await?;

        // Fetch the newly created product
        products::table
            .find(last_id_result as i32)
            .get_result::<Product>(conn)
            .await
    })).await;

    match result {
        Ok(product) => HttpResponse::Created().json(product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Handler to get all products
pub async fn get_products(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let result = products::table
        .load::<Product>(&mut conn)
        .await;

    match result {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Handler to get a product by its ID
pub async fn get_product_by_id(
    pool: web::Data<DbPool>,
    product_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let result = products::table
        .find(product_id.into_inner())
        .get_result::<Product>(&mut conn)
        .await;

    match result {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(diesel::NotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Handler to update a product by its ID
pub async fn update_product_by_id(
    pool: web::Data<DbPool>,
    product_id: web::Path<i32>,
    product_update: web::Json<NewProduct>,
) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let id = product_id.into_inner();
    let product_data = product_update.into_inner();

    // Transaction to ensure atomicity: update, then fetch the updated record.
    let result = conn.transaction(|conn| Box::pin(async move {
        diesel::update(products::table.find(id))
            .set((
                products::name.eq(&product_data.name),
                products::description.eq(&product_data.description),
                products::price.eq(&product_data.price),
            ))
            .execute(conn)
            .await?;

        // Fetch the updated product
        products::table
            .find(id)
            .get_result::<Product>(conn)
            .await
    })).await;

    match result {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(diesel::NotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Handler to delete a product by its ID
pub async fn delete_product_by_id(
    pool: web::Data<DbPool>,
    product_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let result = diesel::delete(products::table.find(product_id.into_inner()))
        .execute(&mut conn)
        .await;

    match result {
        Ok(0) => HttpResponse::NotFound().finish(), // No rows deleted
        Ok(_) => HttpResponse::NoContent().finish(), // 204 No Content
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
