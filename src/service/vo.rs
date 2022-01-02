use crate::model::column::Column;
use crate::utils::string_util;
use crate::config::java;
use crate::service::output as output_service;

// 生成entry文件
pub fn gen_vo(table: &str, column_list: &Vec<Column>) {
    let mut content = String::from("");
    let package_line = get_package_line();
    let import_lines = get_import_lines(table, column_list);
    let annotation_lines = get_annotation_lines();
    let class_name_line = get_class_name_line(table);
    let field_lines = get_field_lines(column_list);
    let empty_constructor_lines = get_empty_constructor_lines(table);
    let entry_constructor_lines = get_entry_constructor_lines(table, column_list);
    let end_line = get_end_line();
    content = content + &package_line + "\n\n" + &import_lines + "\n" + &annotation_lines + &class_name_line + "\n" + "\n" + &field_lines + "\n" 
              + &empty_constructor_lines + " \n" + &entry_constructor_lines + "\n" + &end_line + "\n";
    let filename = string_util::get_hump_class_name(table) + "VO.java";           
    output_service::write_result("vo",  &filename , &content);
    
}

fn get_package_line() -> String {
    let package_name = java::get_package_name();
    String::from("package ") + &package_name + ".vo;"
}


// 需要导入的包
fn get_import_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");
    let mut libs: Vec<String> = Vec::new();
    libs.push(java::get_package_name() + ".domain." + &string_util::get_hump_class_name(table) + "Entry");
    libs.push(String::from("com.fasterxml.jackson.annotation.JsonProperty"));
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
    res = res + "public class " + &string_util::get_hump_class_name(table) + "VO" + " {";
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
            Some(t) => {
                if string_util::check_conation_underline(&column.column_name) {
                    res = res + "    @JsonProperty(\"" + &column.column_name + "\")\n";
                }
                res = res + "    private " + t + " " + &hump_name + ";\n";
            },
            None => continue,
        }
    }
    res
}


// 空构造函数
fn get_empty_constructor_lines(table: &str) -> String {
    let mut res = String::from("");
    res = res + "    public " + &string_util::get_hump_class_name(table) + "VO()" + " {  }\n";
    res
}


// 来自实体的造函数
fn get_entry_constructor_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");

    let entry_type = string_util::get_hump_class_name(table) + "Entry";
    let entry_var = string_util::get_hump_variable_name(table);

    res = res + "    public " + &string_util::get_hump_class_name(table) + "VO(" + &entry_type + " " + &entry_var + ")" + " {\n";
    for column in column_list {
        let column_var = string_util::get_hump_variable_name(&column.column_name);
        res = res + "        this." + &column_var + " = " + &entry_type + ".get" + &string_util::trans_first_word_up(&column_var) + "();\n";
    }
    res = res + "    }\n";
    res
}


// 类结束行
fn get_end_line() -> String{
    let mut res = String::from("") ;
    res = res + "}\n";
    res
}


// 基础列表函数
pub fn gen_base_page_list() {
    let package_line = get_package_line();
    let mut content = String::from("");
    content = content + &package_line + "\n\n";
    content = content + "import lombok.Data;\n\n";
    content = content + "import java.util.List;\n\n";
    content = content + "@Data\n";
    content = content + "public class BasePageListVO<T> {\n\n";
    content = content + "    private List<T> list;\n";
    content = content + "    private Integer count;\n\n";
    content = content + "    public BasePageListVO() { }\n\n";
    content = content + "    public BasePageListVO(List<T> list, Integer count) { \n";
    content = content + "        this.list = list;\n";
    content = content + "        this.count = count;\n";
    content = content + "    }\n";
    content = content + "}\n";
    output_service::write_result("vo",  "BasePageListVO.java" , &content);
}