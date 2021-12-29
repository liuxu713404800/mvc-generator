use crate::model::column::Column;
use crate::config::db;
use crate::utils::string_util;
use crate::config::java;
use crate::service::output as output_service;

// 生成entry文件
pub fn gen_mapper(table: &str, column_list: &Vec<Column>) {
    let mut content = String::from("");
    let package_line = get_package_line();
    let import_lines = get_import_lines(table);
    let annotation_lines = get_annotation_lines();
    let class_name_line = get_class_name_line(table);
    let interface_lines = get_interface_lines(table, column_list);
    let end_line = get_end_line();
    content = content + &package_line + "\n\n" + &import_lines + "\n" + &annotation_lines
              + &class_name_line + "\n" + "\n" + &interface_lines + "\n" + &end_line + "\n";
    let filename = string_util::get_hump_class_name(table) + "Mapper.java";           
    output_service::write_result("mapper",  &filename , &content);
}

// 包名
fn get_package_line() -> String {
    let package_name = java::get_package_name();
    String::from("package ") + &package_name + ".mapper;"
}


// 需要导入的包
fn get_import_lines(table: &str) -> String {
    let mut res = String::from("");
    let mut libs: Vec<String> = Vec::new();
    libs.push(String::from("org.apache.ibatis.annotations.Mapper"));
    libs.push(String::from("org.apache.ibatis.annotations.Param"));
   
    let package_name = java::get_package_name();
    let class_hump = string_util::get_hump_class_name(table);

    libs.push(String::from(&package_name) + ".domain." + &class_hump + "Entry");
    libs.push(String::from(&package_name) + ".dto.filter." + &class_hump + "Filter");

    libs.push(String::from("java.util.List"));
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
    res = res + "@Mapper\n";
    res
}

// 类定义行
fn get_class_name_line(table: &str) -> String {
    let mut res = String::from("");
    res = res + "public interface " + &string_util::get_hump_class_name(table) + "Mapper" + " {";
    res
}

// 函数行
fn get_interface_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");
    let four_space = String::from("    ");

    let entry_type = string_util::get_hump_class_name(table) + "Entry";
    let entry_var = string_util::get_hump_variable_name(table);

    let mut key = db::get_key_column_name(table, column_list);
    key = string_util::get_hump_variable_name(&key);
    let key_up = string_util::get_hump_class_name(&key);
    let key_type = db::get_key_java_type(table, column_list);

    let entry_list = String::from("List<") + &entry_type + ">";
    let entry_param = String::from("@Param(\"") + &entry_var +"\") " + &entry_type + " " + &entry_var;

    let filter_type = string_util::get_hump_class_name(&table) + "Filter";

    let get_by_key = four_space.clone() + &entry_type + " getBy" + &key_up + "(@Param(\"" + &key +"\") " + &key_type + " " + &key + ");";
    let get_by_keys = four_space.clone() + &entry_list + " getBy" + &key_up + "s" + "(@Param(\""+ &key + "s\") " + &key_type + " " + &key + "s);";
    let get_page_list = four_space.clone() + &entry_list + " getPageList(@Param(\"filter\") " + &filter_type + " filter, @Param(\"limit\") Integer limit, @Param(\"offset\") Integer offset);";
    let get_count = four_space.clone() + "Integer getCount(@Param(\"filter\") " + &filter_type + " filter);";
    let get_by_filter = four_space.clone() + &entry_list + " getByFilter(@Param(\"filter\") " + &filter_type + " filter);";
    let add = four_space.clone() + "void add(" + &entry_param + ");";
    let update = four_space.clone() + "void update(" + &entry_param + ");";

    res = res + &get_by_key + "\n\n" + &get_by_keys + "\n\n" + &get_page_list + "\n\n" + &get_count + "\n\n" + &get_by_filter + "\n\n" + &add + "\n\n" + &update + "\n\n";
    res
}

// 类结束行
fn get_end_line() -> String{
    let mut res = String::from("") ;
    res = res + "}\n";
    res
}
