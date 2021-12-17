use mysql::*;
use mysql::prelude::*;

use super::connect;
use crate::config::db::Column;

// TODO 每次都解析成连接串有点蠢，学艺不精后续优化
pub fn get_table_conn() -> Result<PooledConn>{
    let conn_str = connect::get_table_conn_string();
    let opts = Opts::from_url(&conn_str).unwrap();
    let pool = Pool::new(opts).unwrap();
    pool.get_conn()
}

// 得到所有的表名
pub fn get_tables() -> Vec<String> {
    let schema = connect::get_config_schame();
    let sql = String::from("select TABLE_NAME from TABLES where TABLE_SCHEMA = '") + &schema + "'";
    let mut conn = get_table_conn().unwrap();
    let res: Vec<String> = conn.query(&sql).unwrap();
    res
}


// (!!!!TODO  bind params , 字符串拼接太太太太low了)
// 从表得到列名和类型
pub fn get_table_columns(table: String) -> Vec<Column> {

    let schema = connect::get_config_schame();
    let sql = String::from("select COLUMN_NAME column_name, DATA_TYPE data_type, COLUMN_KEY column_key from COLUMNS where TABLE_SCHEMA = '") +
        &schema + "' and TABLE_NAME = '" + &table +"'";
    let mut conn = get_table_conn().unwrap();
    
    let res: Vec<Column> = conn.query_map(sql, |(column_name, data_type, column_key)| Column{column_name: column_name, data_type: data_type , column_key: column_key}).unwrap();
    res
}