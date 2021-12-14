// use chrono::prelude::*;
// use mysql::prelude::*;
use mysql::*; // 用来处理日期

mod config;
mod utils;

use config::db;

fn main() {
    let conn_str = db::get_conn_string();
    let opts = Opts::from_url(&conn_str).unwrap();
    let pool = Pool::new(opts).unwrap();
    let _conn = pool.get_conn().unwrap(); // 获取链接
}
