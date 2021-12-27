use std::collections::HashMap;


// static TYPE_MAP: HashMap<String, String> =
// [("bigint".to_string(), "BIGINT".to_string())]
//  .iter().cloned().collect();


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