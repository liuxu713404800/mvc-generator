use std::collections::HashMap;
use crate::model::column::Column;
use crate::utils::string_util;

// TODO rust hashMap简明初始化 ，全局变量
pub fn get_db_map() -> HashMap<String, String> {
    let res: HashMap<String, String> = [
        ("bigint".to_string(), "BIGINT".to_string()),
        ("char".to_string(), "STRING".to_string()),
        ("date".to_string(), "TIMESTAMP".to_string()),
        ("datetime".to_string(), "TIMESTAMP".to_string()),
        ("decimal".to_string(), "FLOAT".to_string()),
        ("double".to_string(), "FLOAT".to_string()),
        ("float".to_string(), "FLOAT".to_string()),
        ("int".to_string(), "INTEGER".to_string()),
        ("integer".to_string(), "INTEGER".to_string()),
        ("longtext".to_string(), "STRING".to_string()),
        ("mediumint".to_string(), "INTEGER".to_string()),
        ("mediumtext".to_string(), "STRING".to_string()),
        ("numeric".to_string(), "FLOAT".to_string()),
        ("smallint".to_string(), "INTEGER".to_string()),
        ("text".to_string(), "STRING".to_string()),
        ("time".to_string(), "TIMESTAMP".to_string()),
        ("timestamp".to_string(), "TIMESTAMP".to_string()),
        ("tinyint".to_string(), "INTEGER".to_string()),
        ("tinytext".to_string(), "STRING".to_string()),
        ("varchar".to_string(), "STRING".to_string()),
        ].iter().cloned().collect();
    res
}

// 查询主键名称
pub fn get_key_column_name(table: &str, column_list: &Vec<Column>) -> String {
    let mut key: String = String::from("");
    for colum in column_list {
        if colum.column_key == "PRI" {
            key = colum.column_name.clone();
            break;
        }
    }
    if string_util::is_empty(&key) {
        panic!("{}", table.to_string() + " has no primary key");
    }
    key
}


// 查询主键db类型
pub fn get_key_db_type(table: &str, column_list: &Vec<Column>) -> String {
    let mut key: String = String::from("");
    for colum in column_list {
        if colum.column_key == "PRI" {
            key = colum.data_type.clone();
            break;
        }
    }
    if string_util::is_empty(&key) {
        panic!("{}", table.to_string() + " has no primary key");
    }
    let db_map = get_db_map();
    let db_type = db_map.get(&key);
    match db_type {
        Some(t) => String::from(t),
        None => panic!("{}", table.to_string() + " primary key type not find")
    }
}