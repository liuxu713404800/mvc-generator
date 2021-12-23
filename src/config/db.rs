use std::collections::HashMap;

// TODO rust hashMap简明初始化 ，全局变量
pub fn get_type_map() -> HashMap<String, String>{

    let mut res = HashMap::new();
    res.insert("bigint".to_string(), "BIGINT".to_string());
    res.insert("char".to_string(), "STRING".to_string());
    res.insert("date".to_string(), "TIMESTAMP".to_string());
    res.insert("datetime".to_string(), "TIMESTAMP".to_string());
    res.insert("decimal".to_string(), "FLOAT".to_string());
    res.insert("double".to_string(), "FLOAT".to_string());
    res.insert("float".to_string(), "FLOAT".to_string());
    res.insert("int".to_string(), "INTEGER".to_string());
    res.insert("integer".to_string(), "INTEGER".to_string());
    res.insert("longtext".to_string(), "STRING".to_string());
    res.insert("mediumint".to_string(), "INTEGER".to_string());
    res.insert("mediumtext".to_string(), "STRING".to_string());
    res.insert("numeric".to_string(), "FLOAT".to_string());
    res.insert("smallint".to_string(), "INTEGER".to_string());
    res.insert("text".to_string(), "STRING".to_string());
    res.insert("time".to_string(), "TIMESTAMP".to_string());
    res.insert("timestamp".to_string(), "TIMESTAMP".to_string());
    res.insert("tinyint".to_string(), "INTEGER".to_string());
    res.insert("tinytext".to_string(), "STRING".to_string());
    res.insert("varchar".to_string(), "STRING".to_string());
    res
}