use std::collections::HashMap;
use regex::Regex;

pub fn is_empty(input_str: &str) -> bool {
    let mut result = true;
    if input_str.len() > 0 {
        result = false;
    }
    result
}

fn get_character_map() -> HashMap<String, String> {
    let res: HashMap<String, String> = [
        ("a".to_string(), "A".to_string()),
        ("b".to_string(), "B".to_string()),
        ("c".to_string(), "C".to_string()),
        ("d".to_string(), "D".to_string()),
        ("e".to_string(), "E".to_string()),
        ("f".to_string(), "F".to_string()),
        ("g".to_string(), "G".to_string()),
        ("h".to_string(), "H".to_string()),
        ("i".to_string(), "I".to_string()),
        ("j".to_string(), "J".to_string()),
        ("k".to_string(), "K".to_string()),
        ("l".to_string(), "L".to_string()),
        ("m".to_string(), "M".to_string()),
        ("n".to_string(), "N".to_string()),
        ("o".to_string(), "O".to_string()),
        ("p".to_string(), "P".to_string()),
        ("q".to_string(), "Q".to_string()),
        ("r".to_string(), "R".to_string()),
        ("s".to_string(), "S".to_string()),
        ("t".to_string(), "T".to_string()),
        ("u".to_string(), "U".to_string()),
        ("v".to_string(), "V".to_string()),
        ("w".to_string(), "W".to_string()),
        ("x".to_string(), "X".to_string()),
        ("y".to_string(), "Y".to_string()),
        ("z".to_string(), "Z".to_string()),
        ].iter().cloned().collect();
    res
}


// 变量名驼峰
pub fn get_hump_variable_name(name: &str) -> String {
    let mut res = String::from("");
    let split_arr: Vec<String> = split_str_to_list(name);
    if split_arr.len() == 0 {
        return res;
    }
    let mut index = 1; // 变量首字母不大写
    for elem in split_arr {
        if index == 1 {
            res = res + &elem;
        } else {
            res = res + &trans_first_word_up(&elem);
        }
        index = index + 1;
    }
    res
}

// 类名驼峰
pub fn get_hump_class_name(name: &str) -> String {
    let mut res = String::from("");
    let split_arr: Vec<String> = split_str_to_list(name);
    if split_arr.len() == 0 {
        return res;
    }
    for elem in split_arr {
        res = res + &trans_first_word_up(&elem);
    }
    res
}

// 按照"_"切割数组
fn split_str_to_list(name: &str) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let split_arr: Vec<&str> = name.split("_").collect();
    for elem in split_arr {
        if is_empty(elem) {
            continue;
        }
        res.push(String::from(elem))
    }
    res
}


// TODO 切割字符串优雅方案
// 将单次首字母大写
pub fn trans_first_word_up(word: &str) -> String {
    let res = String::from("");
   if is_empty(word) { 
       return res;
   }
   let len = word.len();
   let first = &word[0..1];
   let next = &word[1..len];
   let map = get_character_map();
   match map.get(first) {
      Some(v) => res + &v + &next,
      None => word.to_string(),
   }
}


pub fn check_conation_underline(word: &str) -> bool {
    let r = Regex::new("_").unwrap();
    if r.is_match(word) {
        return true;
    }
    false
}