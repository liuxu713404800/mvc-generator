use crate::model::column::Column;
use crate::utils::string_util;
use crate::config::{java, db};
use crate::service::output as output_service;


pub fn gen_xml(table: &str, column_list: &Vec<Column>) {
    let mut content = String::from("");
    let header_lines = get_header_lines();
    let namespace_line = get_namespace_line(table);

    let base_result_map_lines = get_base_result_map_lines(table, column_list);

    let tail_line = get_tail_line();
    content = content + &header_lines + "\n\n" + &namespace_line + "\n\n" + &base_result_map_lines  + "\n\n"    + &tail_line;

    let filename = String::from(table) + ".xml";   
    output_service::write_result("dao", &filename, &content);
}

// xml头
fn get_header_lines() -> String {
    let mut res = String::from("");
    res = res + "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
    <!DOCTYPE mapper PUBLIC \"-//mybatis.org//DTD Mapper 3.0//EN\" \"http://mybatis.org/dtd/mybatis-3-mapper.dtd\">";
    res
}

// xml 对应的mapper
fn get_namespace_line(table: &str) -> String {
    let mut res = String::from("");
    let clazz = java::get_package_name() + ".mapper." + &string_util::get_hump_class_name(table) + "Mapper";
    
    res = res + "<mapper namespace=\"" + &clazz + "\">";
    res
}


// 得到基础Map表
fn get_base_result_map_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");
    let four_space = String::from("    ");
    let eight_space = String::from("        ");

    let db_type_map = db::get_db_map();

    let package_name = java::get_package_name();
    let entry = string_util::get_hump_class_name(table) + "Entry";

    let key_column = db::get_key_column_name(table, column_list);
    let key_var = string_util::get_hump_variable_name(&key_column);
    let key_type = db::get_key_db_type(table, column_list);

    // 首行声明resultMap
    res = res + &four_space + "<resultMap id=\"ResultMap\" type=\"" + &package_name + ".domain." + &entry + "\">\n";
    res = res + &eight_space + "<id column=\"" + &key_column + "\" property=\"" + &key_var + "\" jdbcType=\"" + &key_type + "\"/>\n";
    for column in column_list {
        let data_type = &column.data_type;
        let coulmn_name = &column.column_name;
        let coulmn_var = string_util::get_hump_variable_name(&coulmn_name);
        if column.column_key == "PRI" {
            continue; // 已经写过了
        }
        let db_type = db_type_map.get(data_type);
        match db_type {
            Some(t) => {
                res = res + &eight_space + "<result column=\"" + &coulmn_name + "\" property=\"" + &coulmn_var + "\" jdbcType=\"" + &t + "\"/>\n";
            },
            None => panic!("{}", table.to_string() + " primary key type not find")
        }
    }
    res = res + &four_space + "</resultMap>\n";
    res
}



fn get_tail_line() -> String {
    String::from("</mapper>")
}