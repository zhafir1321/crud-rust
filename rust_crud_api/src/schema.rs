// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use diesel::mysql::sql_types::Decimal;

    products (id) {
        id -> Integer,
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Decimal,
    }
}
