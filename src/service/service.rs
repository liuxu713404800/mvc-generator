use crate::model::column::Column;
use crate::utils::string_util;
use crate::config::{java, db};
use crate::service::output as output_service;

const FOUR_SPACE: &str = "    ";
const EIGHT_SPACE: &str = "        ";
const TWELVE_SPACE: &str = "            ";
const TWENTY_SPACE: &str = "                    ";

pub fn gen_service(table: &str, column_list: &Vec<Column>) {
    let mut content = String::from("");

    content = content + &get_package_line() + "\n";
    content = content + &get_import_lines(table) + "\n";
    content = content + &get_annotation_lines();
    content = content + &get_class_name_line(table) + "\n";
    content = content + &get_resource_lines(table) + "\n";
    content = content + &get_function_lines(table, column_list) + "\n";
    content = content + &get_tail_line();
    let filename = string_util::get_hump_class_name(table) + "Service.java";
    output_service::write_result("service", &filename, &content);
}

// 包名
fn get_package_line() -> String {
    let package_name = java::get_package_name();
    String::from("package ") + &package_name + ".service;\n"
}


// 需要导入的包
fn get_import_lines(table: &str) -> String {
    let mut res = String::from("");
    let mut libs: Vec<String> = Vec::new();   
    let package_name = java::get_package_name();
    let class_hump = string_util::get_hump_class_name(table);

    libs.push(String::from(&package_name) + ".domain." + &class_hump + "Entry");
    libs.push(String::from(&package_name) + ".mapper." + &class_hump + "Mapper");
    libs.push(String::from(&package_name) + ".dto.filter." + &class_hump + "Filter");
    libs.push(String::from(&package_name) + ".vo.BasePageListVO");
    libs.push(String::from(&package_name) + ".vo." + &class_hump + "VO");
    libs.push(String::from("org.springframework.stereotype.Service"));
    libs.push(String::from("javax.annotation.Resource"));
    libs.push(String::from("java.util.ArrayList"));
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
    res = res + "@Service\n";
    res
}

// 类定义行
fn get_class_name_line(table: &str) -> String {
    let mut res = String::from("");
    res = res + "public class " + &string_util::get_hump_class_name(table) + "Service" + " {\n";
    res
}

// 内部资源
fn get_resource_lines(table: &str) -> String {
    let mut res = String::from("");
    let mapper_type = string_util::get_hump_class_name(table) + "Mapper";
    let mapper_var = string_util::get_hump_variable_name(table) + "Mapper";
    res = res + FOUR_SPACE + "@Resource\n";
    res = res + FOUR_SPACE + "private " + &mapper_type + " " + &mapper_var + ";\n";
    res
}

// 四个常用函数，getPageList, getDetail, add, update
fn get_function_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");
    res = res + &get_page_list_lines(table) + "\n";
    res = res + &get_detail_lines(table, column_list) + "\n";
    res = res + &add_lines(table, column_list) + "\n";
    res = res + &update_lines(table, column_list) + "\n";
    res
}

// 分页列表
fn get_page_list_lines(table: &str) -> String {
    let mut res = String::from("");

    let filter_type = string_util::get_hump_class_name(table) + "Filter";

    let mapper_var = string_util::get_hump_variable_name(table) + "Mapper";

    let entry_type = string_util::get_hump_class_name(table) + "Entry";
    let entry_var = string_util::get_hump_variable_name(table);
    let entry_var_list = entry_var.clone() + "List";

    let vo_type = string_util::get_hump_class_name(table) + "VO";

    res = res + FOUR_SPACE + "public BasePageListVO getPageList(Integer pageNum, Integer pageSize) { \n"; 
    res = res + EIGHT_SPACE + "List<" + &vo_type +"> voList = new ArrayList<>();\n";
    res = res + EIGHT_SPACE + "Integer limit = pageSize; // TODO construct limit \n";
    res = res + EIGHT_SPACE + "Integer offset = pageNum * pageSize; // TODO construct offset \n";

    res = res + EIGHT_SPACE + &filter_type + " filter = " + "new " + &filter_type + "(); // TODO construct filter\n";
    res = res + EIGHT_SPACE + "List<" + &entry_type + "> " + &entry_var_list + " = " + &mapper_var + ".getPageList(filter, limit, offset);\n";
    res = res + EIGHT_SPACE + "Integer count = " + &mapper_var + ".getCount(filter);\n";

    res = res + EIGHT_SPACE + "for (" + &entry_type + " " + &entry_var + ": " + &entry_var_list + ") {\n";
    res = res + TWELVE_SPACE + "voList.add(new " + &vo_type + "(" + &entry_var +"));\n";
    res = res + EIGHT_SPACE + "}\n";
    
    res = res + EIGHT_SPACE + "return new BasePageListVO<>(voList, count);\n";
    res = res + FOUR_SPACE + "}\n";
    res
}

// 查询详情
fn get_detail_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");

    let mapper_var = string_util::get_hump_variable_name(table) + "Mapper";

    let entry_type = string_util::get_hump_class_name(table) + "Entry";
    let entry_var = string_util::get_hump_variable_name(table);

    let vo_type = string_util::get_hump_class_name(table) + "VO";

    let key_column = db::get_key_column_name(table, column_list); // key字段
    let key_var = string_util::get_hump_variable_name(&key_column); // key字段转变量
    let key_up = string_util::trans_first_word_up(&key_var); // 首字母大写
    let key_type = java::get_key_java_type(table, column_list); // key字段类型
    
    res = res + FOUR_SPACE + "public " + &vo_type + " getDetail(" + &key_type + " " + &key_var + ") { \n"; 
    res = res + EIGHT_SPACE + &entry_type + " " + &entry_var + " = " + &mapper_var + ".getBy" + &key_up + "(" + &key_var + ");\n";
    res = res + EIGHT_SPACE + "return new " + &vo_type + "(" + &entry_var + ");\n";
    res = res + FOUR_SPACE + "}\n";
    res
}

// 添加
fn add_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");

    let entry_type = string_util::get_hump_class_name(table) + "Entry";
    let entry_var = string_util::get_hump_variable_name(table);

    let entry_mapper = string_util::get_hump_variable_name(table) + "Mapper";

    let java_map = java::get_java_map();

    res = res + FOUR_SPACE + "public void add(";

    // 获得需要添加的参数列表
    let mut add_list: Vec<(String, String)> = Vec::new();
    for column in column_list {
        let column_name = &column.column_name;
        let data_type: &str = &column.data_type;
        // 主键或者忽略字段不加入函数
        if column.column_key == "PRI" || column_name == "create_time" || column_name == "update_time" {
            continue;
        }
        let column_var = string_util::get_hump_variable_name(column_name);
        let java_type = java_map.get(data_type);
        match java_type {
            Some(t) => add_list.push((String::from(t), column_var)),
            None => panic!("column java type not find")
        }
    }
    // 添加参数
    let mut i = 0;
    let params_len = add_list.len();
    let last_idx = params_len - 1;
    while i < params_len {
        let data_type = &add_list[i].0;
        let data_var = &add_list[i].1;
        if i == 0 {
            res = res + data_type + " " + data_var + ",\n"
        } 
        if 0 < i && i < last_idx {
            res = res + TWENTY_SPACE + data_type + " " + data_var + ",\n"
        }
        if i == last_idx {
            res = res + TWENTY_SPACE + data_type + " " + data_var + ") {\n"
        }
        i = i + 1
    }

    res = res + EIGHT_SPACE + &entry_type + " " + &entry_var + " = new " + &entry_type + "();\n";

    i = 0;
    while i < params_len {
        let data_var = &add_list[i].1;
        res = res + EIGHT_SPACE + &entry_var + ".set" + &string_util::trans_first_word_up(data_var) + "(" + &data_var + ");\n";
        i = i + 1
    }

    res = res + EIGHT_SPACE + &entry_mapper + ".add(" + &entry_var + ");\n";
    res = res + FOUR_SPACE + "}\n";
    res
}

// 更新
fn update_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");

    let entry_type = string_util::get_hump_class_name(table) + "Entry";
    let entry_var = string_util::get_hump_variable_name(table);

    let entry_mapper = string_util::get_hump_variable_name(table) + "Mapper";

    let java_map = java::get_java_map();

    res = res + FOUR_SPACE + "public void update(";

    // 获得需要添加的参数列表
    let mut add_list: Vec<(String, String)> = Vec::new();
    for column in column_list {
        let column_name = &column.column_name;
        let data_type: &str = &column.data_type;
        // 主键或者忽略字段不加入函数
        if column_name == "create_time" || column_name == "update_time" {
            continue;
        }
        let column_var = string_util::get_hump_variable_name(column_name);
        let java_type = java_map.get(data_type);
        match java_type {
            Some(t) => add_list.push((String::from(t), column_var)),
            None => panic!("column java type not find")
        }
    }
    // 添加参数
    let mut i = 0;
    let params_len = add_list.len();
    let last_idx = params_len - 1;
    while i < params_len {
        let data_type = &add_list[i].0;
        let data_var = &add_list[i].1;
        if i == 0 {
            res = res + data_type + " " + data_var + ",\n"
        } 
        if 0 < i && i < last_idx {
            res = res + TWENTY_SPACE + "   " + data_type + " " + data_var + ",\n"
        }
        if i == last_idx {
            res = res + TWENTY_SPACE + "   " + data_type + " " + data_var + ") {\n"
        }
        i = i + 1
    }

    res = res + EIGHT_SPACE + &entry_type + " " + &entry_var + " = new " + &entry_type + "();\n";

    i = 0;
    while i < params_len {
        let data_var = &add_list[i].1;
        res = res + EIGHT_SPACE + &entry_var + ".set" + &string_util::trans_first_word_up(data_var) + "(" + &data_var + ");\n";
        i = i + 1
    }

    res = res + EIGHT_SPACE + &entry_mapper + ".update(" + &entry_var + ");\n";
    res = res + FOUR_SPACE + "}\n";
    res
}


// 行结束
fn get_tail_line() -> String {
    String::from("}\n")
}
