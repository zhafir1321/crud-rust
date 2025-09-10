use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::products;
use bigdecimal::BigDecimal;

// Struct for reading data from the database
#[derive(Queryable, Selectable, Serialize, Debug, PartialEq)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
}

// Struct for creating a new product (no id)
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
}
