use mysql::prelude::*;


mod config;
mod utils;
mod service;


use service::db_service;

fn main() {
    let mut data_conn = db_service::get_data_conn().unwrap();

    let res:Vec<(String, String)> = data_conn.query("Select account, password from ad_user").unwrap();
    for r in res {
        println!("id={},name={}", r.0, r.1);
    }

    let mut table_conn = db_service::get_table_conn().unwrap();
    let res:Vec<(String, String)> = table_conn.query("Select TABLE_SCHEMA, TABLE_NAME from TABLES where TABLE_SCHEMA = 'union'").unwrap();
    for r in res {
        println!("TABLE_SCHEMA={},TABLE_NAME={}", r.0, r.1);
    }

}
