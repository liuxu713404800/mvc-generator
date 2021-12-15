use mysql::*;
// use chrono::prelude::*;
// use mysql::prelude::*;

use crate::config::db;

pub fn get_data_conn() -> Result<PooledConn> {
    let conn_str = db::get_data_conn_string();
    let opts = Opts::from_url(&conn_str).unwrap();
    let pool = Pool::new(opts).unwrap();
    pool.get_conn()
}


pub fn get_table_conn() -> Result<PooledConn>{
    let conn_str = db::get_table_conn_string();
    let opts = Opts::from_url(&conn_str).unwrap();
    let pool = Pool::new(opts).unwrap();
    pool.get_conn()
}