use crate::model::column::Column;
use crate::utils::string_util;
use crate::config::java;
use crate::service::output as output_service;

// 生成entry文件
pub fn gen_entry(table: &str, column_list: &Vec<Column>) {
    let mut content = String::from("");
    let package_line = get_package_line();
    let import_lines = get_import_lines(column_list);
    let annotation_lines = get_annotation_lines();
    let class_name_line = get_class_name_line(table);
    let field_lines = get_field_lines(column_list);
    let end_line = get_end_line();
    content = content + &package_line + "\n\n" + &import_lines + "\n" + &annotation_lines
              + &class_name_line + "\n" + "\n" + &field_lines + "\n" + &end_line + "\n";
    let filename = string_util::get_hump_class_name(table) + "Entry.java";           
    output_service::write_result("domain",  &filename , &content);
}

fn get_package_line() -> String {
    let package_name = java::get_package_name();
    String::from("package ") + &package_name + ".domain;"
}


// 需要导入的包
fn get_import_lines(column_list: &Vec<Column>) -> String {
    let mut res = String::from("");
    let mut libs: Vec<String> = Vec::new();
    libs.push(String::from("lombok.Data"));
    if column_list.len() == 0 {
        return res;
    }
    let db_java_map = java::get_java_map(); // db->java类型转化
    let lib_map = java::get_need_lib_type_map(); // 需要格外引入库
    // 获取需要额外import的库
    for column in column_list {
        let data_type = &column.data_type; // 列的数据库类型
        let java_type = db_java_map.get(data_type).unwrap(); // 对应的java类型
        let type_res = lib_map.get(java_type);
        match type_res {
            Some(t) => {
                if !libs.contains(t) {
                    libs.push(String::from(t));
                }
            },
            None => {},
        }
    }
    // 拼接字符串
    for lib in libs {
        let line = String::from("import ") + &lib + ";\n";
        res = res + &line;
    }
    res
}

// 注解行，entry中一般有@Data
fn get_annotation_lines() -> String {
    let mut res = String::from("") ;
    res = res + "@Data\n";
    res
}

// 类定义行
fn get_class_name_line(table: &str) -> String {
    let mut res = String::from("");
    res = res + "public class " + &string_util::get_hump_class_name(table) + "Entry" + " {";
    res
}

// 字段行
fn get_field_lines(column_list: &Vec<Column>) -> String {
    let mut res = String::from("");
    let db_java_map = java::get_java_map(); // db->java类型转化

    for column in column_list {
        let hump_name = string_util::get_hump_variable_name(&column.column_name);
        let data_type = &column.data_type; // 列的数据库类型
        let java_type = db_java_map.get(data_type); // 对应的java类型
        match java_type {
            Some(t) => {res = res + "    private " + t + " " + &hump_name + ";\n";},
            None => continue,
        }
    }
    res
}

// 类结束行
fn get_end_line() -> String{
    let mut res = String::from("") ;
    res = res + "}\n";
    res
}
