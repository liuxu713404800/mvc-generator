use crate::utils::string_util;
use crate::config::java;
use crate::service::output as output_service;

// 生成entry文件
pub fn gen_filter(table: &str) {
    let mut content = String::from("");
    let package_line = get_package_line();
    let import_lines = get_import_lines();
    let annotation_lines = get_annotation_lines();
    let class_name_line = get_class_name_line(table);
    let empty_constructor_lines = get_empty_constructor_lines(table);
    let end_line = get_end_line();
    content = content + &package_line + "\n\n" + &import_lines + "\n" + &annotation_lines
              + &class_name_line + "\n\n" + &empty_constructor_lines + "\n" + &end_line + "\n";
    let filename = string_util::get_hump_class_name(table) + "Filter.java";           
    output_service::write_result("dto/filter",  &filename , &content);
}

// 包名
fn get_package_line() -> String {
    let package_name = java::get_package_name();
    String::from("package ") + &package_name + ".dto.filter;"
}


// 需要导入的包
fn get_import_lines() -> String {
    let mut res = String::from("");
    let mut libs: Vec<String> = Vec::new();
    libs.push(String::from("lombok.Data"));
    // 拼接字符串
    for lib in libs {
        let line = String::from("import ") + &lib + ";\n";
        res = res + &line;
    }
    res
}

// 注解行
fn get_annotation_lines() -> String {
    let mut res = String::from("") ;
    res = res + "@Data\n";
    res
}

// 类定义行
fn get_class_name_line(table: &str) -> String {
    let mut res = String::from("");
    res = res + "public class " + &string_util::get_hump_class_name(table) + "Filter" + " {";
    res
}

// 实际上的filter的内容自己编写
// fn get_function_lines(table: &str) -> String {

// }

// 空构造函数
fn get_empty_constructor_lines(table: &str) -> String {
    let mut res = String::from("");
    res = res + "    public " + &string_util::get_hump_class_name(table) + "Filter()" + " {  }\n";
    res
}

// 类结束行
fn get_end_line() -> String{
    let mut res = String::from("") ;
    res = res + "}\n";
    res
}
