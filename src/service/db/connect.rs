use std::collections::HashMap;
use regex::Regex;

use crate::utils::*;

// 获得查询表的字符串
pub fn get_table_conn_string() -> String {
    let mut config_map = get_config_map();
    config_map.insert("schema".to_string(), "information_schema".to_string());
    // 获得链接串
    let conn = get_config_str(config_map);
    conn
}

// 得到配置文件map
pub fn get_config_map() -> HashMap<String, String> {
    // 获取配置文件
    let config_str = file_util::get_content(&String::from("config.ini"));
    // 简单校验合法性
    if !check_valid(&config_str) {
        panic!("database config not valid");
    }
    // 过滤掉非数数据库配置
    let database_config_str = filter_config(&config_str);
    // 得到配置数组
    let config_map = parse_config(database_config_str);
    config_map
}


// 获得配置文件
pub fn get_config_schame() -> String {
    let config_map = get_config_map();
    let schema = config_map.get("schema").unwrap();
    String::from(schema)
}


// 简单检查文件合法性
pub fn check_valid(config_str: &str) -> bool {
    !string_util::is_empty(config_str)
}


// 过滤非数据库配置
pub fn filter_config(config_str: &str) -> String {
    let split_arr: Vec<&str> = config_str.split("\n").collect();
    let mut res = String::from("");
    let r = Regex::new("database").unwrap();
    for line in split_arr {
        if r.is_match(line) {
            res.push_str(line);
            res.push_str("\n")
        }
    }
    res
}

// 解析配置文件
pub fn parse_config(config_str: String) -> HashMap<String, String> {
    let split_arr: Vec<&str> = config_str.split("\n").collect();
    let mut res = HashMap::new();
    for line in split_arr {
        if string_util::is_empty(line) {
            continue;
        }

        let kv: Vec<&str> = line.split("=").collect();
        let key = kv[0];
        let value = kv[1];

        match key {
            "database.host" => res.insert(String::from("host"), value.to_string()),
            "database.port" => res.insert(String::from("port"), value.to_string()),
            "database.user" => res.insert(String::from("user"), value.to_string()),
            "database.password" => res.insert(String::from("password"), value.to_string()),
            "database.schema" => res.insert(String::from("schema"), value.to_string()),
            _ => None,
        };
    }
    res
}

// 从配置文件得到链接字符串
pub fn get_config_str(config_map: HashMap<String, String>) -> String {
    let host = config_map.get("host").unwrap();
    if string_util::is_empty(host) {
        panic!("host error")
    }
    let port = config_map.get("port").unwrap();
    if string_util::is_empty(port) {
        panic!("port error")
    }
    let user = config_map.get("user").unwrap();
    if string_util::is_empty(user) {
        panic!("user error")
    }
    let password = config_map.get("password").unwrap();
    if string_util::is_empty(password) {
        panic!("password error")
    }
    let schema = config_map.get("schema").unwrap();
    if string_util::is_empty(schema) {
        panic!("schema error")
    }
    let mut res = String::from("mysql://");
    res.push_str(user);
    res.push_str(":");
    res.push_str(password);
    res.push_str("@");
    res.push_str(host);
    res.push_str(":");
    res.push_str(port);
    res.push_str("/");
    res.push_str(schema);
    res
}