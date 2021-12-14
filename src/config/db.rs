use std::collections::HashMap;
use regex::Regex;

use crate::utils::*;

pub fn get_conn_string() -> String {
    let config_str = file_util::get_content(&String::from("config.ini")); // 获取配置文件
    // 简单校验合法性
    if !check_valid(&config_str) {
        panic!("database config not valid");
    }
    // 过滤掉非数数据库配置
    let database_config_str = filter_config(&config_str);
    // 得到配置数组
    let config_map = parse_config(&database_config_str);
    // 获得链接串
    let conn = get_config_str(config_map);
    conn
}

pub fn check_valid(config_str: &str) -> bool {
    !string_util::is_empty(config_str)
}

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

pub fn parse_config(config_str: &str) -> HashMap<&str, &str> {
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
            "database.host" => res.insert("host", value),
            "database.port" => res.insert("port", value),
            "database.user" => res.insert("user", value),
            "database.password" => res.insert("password", value),
            "database.database" => res.insert("database", value),
            _ => None,
        };
    }
    res
}

pub fn get_config_str(config_map: HashMap<&str, &str>) -> String {
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
    let database = config_map.get("database").unwrap();
    if string_util::is_empty(database) {
        panic!("database error")
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
    res.push_str(database);
    res
}