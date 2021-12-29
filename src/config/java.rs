use std::collections::HashMap;
use crate::model::column::Column;
use regex::Regex;

use crate::utils::*;


// 获得包名
pub fn get_package_name() -> String {
    let config_str = file_util::get_content(&String::from("config.ini"));
    let split_arr: Vec<&str> = config_str.split("\n").collect();
    let mut res = String::from("");
    let r = Regex::new("app.packagename").unwrap();
    for line in split_arr {
        if r.is_match(line) {
            let kv: Vec<&str> = line.split("=").collect();
            res = String::from(kv[1]);
            break;
        }
    }
    res
}


// 数据库与java 类型转化，只支持自己的常用类型
pub fn get_java_map() -> HashMap<String, String> {
    let res: HashMap<String, String> = [
        ("bigint".to_string(), "String".to_string()),
        ("char".to_string(), "String".to_string()),
        ("date".to_string(), "Timestamp".to_string()),
        ("datetime".to_string(), "Timestamp".to_string()),
        ("decimal".to_string(), "Float".to_string()),
        ("double".to_string(), "Float".to_string()),
        ("float".to_string(), "Float".to_string()),
        ("int".to_string(), "Integer".to_string()),
        ("integer".to_string(), "Integer".to_string()),
        ("longtext".to_string(), "String".to_string()),
        ("mediumint".to_string(), "Integer".to_string()),
        ("mediumtext".to_string(), "String".to_string()),
        ("numeric".to_string(), "Float".to_string()),
        ("smallint".to_string(), "Integer".to_string()),
        ("text".to_string(), "String".to_string()),
        ("time".to_string(), "Timestamp".to_string()),
        ("timestamp".to_string(), "Timestamp".to_string()),
        ("tinyint".to_string(), "Integer".to_string()),
        ("tinytext".to_string(), "String".to_string()),
        ("varchar".to_string(), "String".to_string()),
        ].iter().cloned().collect();
    res
}

// 得到需要引入java库的类型map
pub fn get_need_lib_type_map() -> HashMap<String, String> {
    let res: HashMap<String, String> = [
        ("Timestamp".to_string(), "java.sql.Timestamp".to_string())
        ].iter().cloned().collect();
    res
}


// 查询主键java类型
pub fn get_key_java_type(table: &str, column_list: &Vec<Column>) -> String {
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
    let java_map = get_java_map();
    let java_type = java_map.get(&key);
    match java_type {
        Some(t) => String::from(t),
        None => panic!("{}", table.to_string() + " primary key type not find")
    }
}