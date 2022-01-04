use crate::model::column::Column;
use crate::utils::string_util;
use crate::config::{java, db};
use crate::service::output as output_service;

const FOUR_SPACE: &str = "    ";
const EIGHT_SPACE: &str = "        ";
const TWENTY_SPACE: &str = "                    ";

pub fn gen_controller(table: &str, column_list: &Vec<Column>) {
    let mut content = String::from("");

    content = content + &get_package_line() + "\n";
    content = content + &get_import_lines(table) + "\n";
    content = content + &get_annotation_lines();
    content = content + &get_class_name_line(table) + "\n";
    content = content + &get_resource_lines(table) + "\n";
    content = content + &get_function_lines(table, column_list) + "\n";
    content = content + &get_tail_line();
    let filename = string_util::get_hump_class_name(table) + "Controller.java";
    output_service::write_result("controller", &filename, &content);
}

// 包名
fn get_package_line() -> String {
    let package_name = java::get_package_name();
    String::from("package ") + &package_name + ".controller;\n"
}


// 需要导入的包
fn get_import_lines(table: &str) -> String {
    let mut res = String::from("");
    let mut libs: Vec<String> = Vec::new();   
    let package_name = java::get_package_name();
    let class_hump = string_util::get_hump_class_name(table);

    libs.push(String::from(&package_name) + ".service." + &class_hump + "Service");
    libs.push(String::from(&package_name) + ".vo.BasePageListVO");
    libs.push(String::from(&package_name) + ".vo." + &class_hump + "VO");
    libs.push(String::from("org.springframework.web.bind.annotation.*"));
    libs.push(String::from("javax.annotation.Resource"));
  
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
    res = res + "@RestController\n";
    res = res + "@RequestMapping(\"\")\n";
    res
}

// 类定义行
fn get_class_name_line(table: &str) -> String {
    let mut res = String::from("");
    res = res + "public class " + &string_util::get_hump_class_name(table) + "Controller" + " {\n";
    res
}

// 内部资源
fn get_resource_lines(table: &str) -> String {
    let mut res = String::from("");
    let service_type = string_util::get_hump_class_name(table) + "Service";
    let service_var = string_util::get_hump_variable_name(table) + "Service";
    res = res + FOUR_SPACE + "@Resource\n";
    res = res + FOUR_SPACE + "private " + &service_type + " " + &service_var + ";\n";
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

    let service_var = string_util::get_hump_variable_name(table) + "Service";
    
    res = res + FOUR_SPACE + "@GetMapping(\"getPageList\")\n"; 
    res = res + FOUR_SPACE + "public void getPageList(@RequestParam(value = \"page_num\") Integer pageNum,\n";
    res = res + FOUR_SPACE + "                        @RequestParam(value = \"page_size\") Integer pageSize) { \n"; 
    res = res + EIGHT_SPACE + "BasePageListVO res = " + &service_var + ".getPageList(pageNum, pageSize);\n";
    res = res + FOUR_SPACE + "}\n";
    res
}

// 查询详情
fn get_detail_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");

    let service_var = string_util::get_hump_variable_name(table) + "Service";

    let key_column = db::get_key_column_name(table, column_list); // key字段
    let key_var = string_util::get_hump_variable_name(&key_column); // key字段转变量
    let key_up = string_util::trans_first_word_up(&key_var); // 首字母大写
    let key_type = java::get_key_java_type(table, column_list); // key字段类型

    let vo_type = string_util::get_hump_class_name(table) + "VO";
    
    res = res + FOUR_SPACE + "@GetMapping(\"getDetail\")\n"; 
    res = res + FOUR_SPACE + "public void getDetail(@RequestParam(value = \"" + &key_var + "\")" + &key_type + " " + &key_var + ") { \n"; 
    res = res + EIGHT_SPACE + &vo_type + " " + " res = " + &service_var + ".getBy" + &key_up + "(" + &key_var + ");\n";
    res = res + FOUR_SPACE + "}\n";
    res
}

// 添加
fn add_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");

    let service_var = string_util::get_hump_variable_name(table) + "Service";

    let java_map = java::get_java_map();

    res = res + FOUR_SPACE + "@PostMapping(\"add\")\n";
    res = res + FOUR_SPACE + "public void add(";

    // 获得需要添加的参数列表
    let mut add_list: Vec<(String, String, String)> = Vec::new();
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
            Some(t) => add_list.push((String::from(t), String::from(column_name), column_var)),
            None => panic!("column java type not find")
        }
    }
    // 添加参数
    let mut i = 0;
    let params_len = add_list.len();
    let last_idx = params_len - 1;
    while i < params_len {
        let data_type = &add_list[i].0;
        let data_name = &add_list[i].1;
        let data_var = &add_list[i].2;
        if i == 0 {
            res = res + "@RequestParam(value = \"" + data_name + "\") " + data_type + " " + data_var + ",\n"
        } 
        if 0 < i && i < last_idx {
            res = res + TWENTY_SPACE + "@RequestParam(value = \"" + data_name + "\") " + data_type + " " + data_var + ",\n"
        }
        if i == last_idx {
            res = res + TWENTY_SPACE + "@RequestParam(value = \"" + data_name + "\") " + data_type + " " + data_var + ") {\n"
        }
        i = i + 1
    }

    res = res + EIGHT_SPACE + &service_var + ".add(";
    i = 0;
    while i < params_len {
        let data_var = &add_list[i].2;
        if i == last_idx {
            res = res + &data_var + ");\n";
        } else {
            res = res + &data_var + ", ";
        }
        i = i + 1
    }
    res = res + FOUR_SPACE + "}\n";
    res
}

// 更新
fn update_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");

    let service_var = string_util::get_hump_variable_name(table) + "Service";

    let java_map = java::get_java_map();

    res = res + FOUR_SPACE + "@PostMapping(\"update\")\n";
    res = res + FOUR_SPACE + "public void update(";

    // 获得需要添加的参数列表
    let mut add_list: Vec<(String, String, String)> = Vec::new();
    for column in column_list {
        let column_name = &column.column_name;
        let data_type: &str = &column.data_type;
        // 忽略字段不加入函数
        if column_name == "create_time" || column_name == "update_time" {
            continue;
        }
        let column_var = string_util::get_hump_variable_name(column_name);
        let java_type = java_map.get(data_type);
        match java_type {
            Some(t) => add_list.push((String::from(t), String::from(column_name), column_var)),
            None => panic!("column java type not find")
        }
    }
    // 添加参数
    let mut i = 0;
    let params_len = add_list.len();
    let last_idx = params_len - 1;
    while i < params_len {
        let data_type = &add_list[i].0;
        let data_name = &add_list[i].1;
        let data_var = &add_list[i].2;
        if i == 0 {
            res = res + "@RequestParam(value = \"" + data_name + "\") " + data_type + " " + data_var + ",\n"
        } 
        if 0 < i && i < last_idx {
            res = res + TWENTY_SPACE + "   @RequestParam(value = \"" + data_name + "\") " + data_type + " " + data_var + ",\n"
        }
        if i == last_idx {
            res = res + TWENTY_SPACE + "   @RequestParam(value = \"" + data_name + "\") " + data_type + " " + data_var + ") {\n"
        }
        i = i + 1
    }

    res = res + EIGHT_SPACE + &service_var + ".update(";
    i = 0;
    while i < params_len {
        let data_var = &add_list[i].2;
        if i == last_idx {
            res = res + &data_var + ");\n";
        } else {
            res = res + &data_var + ", ";
        }
        i = i + 1
    }
    res = res + FOUR_SPACE + "}\n";
    res
}


// 行结束
fn get_tail_line() -> String {
    String::from("}\n")
}
