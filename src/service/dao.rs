use crate::model::column::Column;
use crate::utils::string_util;
use crate::config::{java, db};
use crate::service::output as output_service;

const FOUR_SPACE: &str = "    ";
const EIGHT_SPACE: &str = "        ";
const TWELVE_SPACE: &str = "            ";
const SIXTEEN_SPACE: &str = "                ";

pub fn gen_xml(table: &str, column_list: &Vec<Column>) {
    let mut content = String::from("");
    let header_lines = get_header_lines();
    let namespace_line = get_namespace_line(table);

    let base_result_map_lines = get_base_result_map_lines(table, column_list);
    let sql_lines = get_sql_lines(table, column_list);


    let tail_line = get_tail_line();
    content = content + &header_lines + "\n\n" + &namespace_line + "\n\n" + &base_result_map_lines  + "\n\n" + &sql_lines + "\n\n" + &tail_line;

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

    let db_type_map = db::get_db_map();

    let package_name = java::get_package_name(); // 包名
    let entry = string_util::get_hump_class_name(table) + "Entry"; // domain

    let key_column = db::get_key_column_name(table, column_list); // key字段
    let key_var = string_util::get_hump_variable_name(&key_column); // key字段转变量
    let key_type = db::get_key_db_type(table, column_list); // key字段类型

    // 首行声明resultMap
    res = res + FOUR_SPACE + "<resultMap id=\"ResultMap\" type=\"" + &package_name + ".domain." + &entry + "\">\n";
    res = res + EIGHT_SPACE + "<id column=\"" + &key_column + "\" property=\"" + &key_var + "\" jdbcType=\"" + &key_type + "\"/>\n";
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
                res = res + EIGHT_SPACE + "<result column=\"" + &coulmn_name + "\" property=\"" + &coulmn_var + "\" jdbcType=\"" + &t + "\"/>\n";
            },
            None => panic!("{}", table.to_string() + " primary key type not find")
        }
    }
    res = res + FOUR_SPACE + "</resultMap>\n";
    res
}

fn get_sql_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");
    res = res + &get_by_key_lines(table, column_list) + "\n\n";
    res = res + &get_by_keys_lines(table, column_list) + "\n\n";
    res = res + &get_page_list_lines(table) + "\n\n";
    res = res + &get_count_lines(table) + "\n\n";
    res = res + &get_by_filter_lines(table) + "\n\n";
    res = res + &add_lines(table, column_list) + "\n\n";
    res = res + &update_lines(table, column_list);
    res
}


fn get_by_key_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");

    let key_column = db::get_key_column_name(table, column_list);
    let key_var = string_util::get_hump_variable_name(&key_column);
    let key_up = string_util::trans_first_word_up(&key_var);

    res = res + FOUR_SPACE + "<select id=\"getBy" + &key_up + "\" resultMap=\"ResultMap\">\n";
    res = res + EIGHT_SPACE + "select * from " + table + " where " + &key_column + " = #{" + &key_var + "}\n";
    res = res + FOUR_SPACE + "</select>\n";
    res
}

fn get_by_keys_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");

    let key_column = db::get_key_column_name(table, column_list);
    let key_var = string_util::get_hump_variable_name(&key_column);
    let key_up = string_util::trans_first_word_up(&key_var);

    res = res + FOUR_SPACE + "<select id=\"getBy" + &key_up + "s\" resultMap=\"ResultMap\">\n";
    res = res + EIGHT_SPACE + "select * from " + table + " where " + &key_column + " in\n";
    res = res + EIGHT_SPACE + "<foreach collection=\"" + &key_var + "s\" separator=\",\" item=\"item\" open=\"(\" close=\")\"> #{item} </foreach>\n";
    res = res + FOUR_SPACE + "</select>\n";
    res
}

fn get_page_list_lines(table: &str) -> String {
    let mut res = String::from("");

    res = res + FOUR_SPACE + "<select id=\"getPageList\" resultMap=\"ResultMap\">\n";
    res = res + EIGHT_SPACE + "select * from " + table + " where 1\n";
    res = res + EIGHT_SPACE + "<!-- add filter condition -->\n";
    res = res + EIGHT_SPACE + "limit #{limit} offset #{offset}\n";
    res = res + FOUR_SPACE + "</select>\n";
    res
}

fn get_count_lines(table: &str) -> String {
    let mut res = String::from("");

    res = res + FOUR_SPACE + "<select id=\"getCount\" resultType=\"java.lang.Integer\">\n";
    res = res + EIGHT_SPACE + "select count(*) from " + table + " where 1\n";
    res = res + EIGHT_SPACE + "<!-- add filter condition -->\n";
    res = res + FOUR_SPACE + "</select>\n";
    res
}


fn get_by_filter_lines(table: &str) -> String {
    let mut res = String::from("");
    
    res = res + FOUR_SPACE + "<select id=\"getByFilter\" resultMap=\"ResultMap\">\n";
    res = res + EIGHT_SPACE + "select * from " + table + " where 1\n";
    res = res + EIGHT_SPACE + "<!-- add filter condition -->\n";
    res = res + FOUR_SPACE + "</select>\n";
    res
}


fn add_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");

    let package_name = java::get_package_name();
    let entry_type = string_util::get_hump_class_name(table) + "Entry";
    let entry_var = string_util::get_hump_variable_name(table);

    let key_column = db::get_key_column_name(table, column_list);
    let key_var = string_util::get_hump_variable_name(&key_column);

    res = res + FOUR_SPACE + "<insert id=\"add\" parameterType=\"" + &package_name + ".domain." + &entry_type + "\" useGeneratedKeys=\"true\" keyProperty=\"" + &key_var + "\">\n";
    res = res + EIGHT_SPACE + "insert into " + &table + "\n";
    res = res + EIGHT_SPACE + "(<trim suffixOverrides=\",\">\n";
    for column in column_list {
        if column.column_name == key_column {
            continue;
        }
        let column_var = string_util::get_hump_variable_name(&column.column_name);
        res = res + TWELVE_SPACE + "<if test=\"" + &entry_var + "." + &column_var + " != null\">\n";
        res = res + SIXTEEN_SPACE + &column.column_name + ",\n";
        res = res + TWELVE_SPACE + "</if>\n";
    }

    res = res + EIGHT_SPACE + "</trim>)\n";
    res = res + EIGHT_SPACE + "values\n";
    res = res + EIGHT_SPACE + "(<trim suffixOverrides=\",\">\n";  

    for column in column_list {
        if column.column_name == key_column {
            continue;
        }
        let column_var = string_util::get_hump_variable_name(&column.column_name);
        res = res + TWELVE_SPACE + "<if test=\"" + &entry_var + "." + &column_var + " != null\">\n";
        res = res + SIXTEEN_SPACE + "#{" + &entry_var + "." + &column_var + "},\n";
        res = res + TWELVE_SPACE + "</if>\n";
    }
    res = res + EIGHT_SPACE + "</trim>)\n";
    res = res + FOUR_SPACE + "</insert>\n";
    res
}

fn update_lines(table: &str, column_list: &Vec<Column>) -> String {
    let mut res = String::from("");

    let key_column = db::get_key_column_name(table, column_list);
    let key_var = string_util::get_hump_variable_name(&key_column);

    let package_name = java::get_package_name();
    let entry_type = string_util::get_hump_class_name(table) + "Entry";
    let entry_var = string_util::get_hump_variable_name(table);

    res = res + FOUR_SPACE + "<update id=\"update\" parameterType=\"" + &package_name + ".domain." + &entry_type + "\">\n";
    res = res + EIGHT_SPACE + "update " +  &table + "\n";
    res = res + EIGHT_SPACE + "<trim prefix=\"set\" suffixOverrides=\",\">\n";
    for column in column_list {
        if column.column_key == "PRI" {
            continue;
        }
        let column_var = string_util::get_hump_variable_name(&column.column_name);
        res = res + TWELVE_SPACE + "<if test=\"" + &entry_var + "." + &column_var + " != null\">\n";
        res = res + SIXTEEN_SPACE + &column.column_name + " = #{" + &entry_var + "." + &column_var + "},\n";
        res = res + TWELVE_SPACE + "</if>\n";
    }
    res = res + EIGHT_SPACE + "</trim>)\n";
    res = res + EIGHT_SPACE + "where " + &key_column + " = #{" + &entry_var + "." + &key_var + "} limit 1\n";
    res = res + FOUR_SPACE + "</update>\n";
    res
}

fn get_tail_line() -> String {
    String::from("</mapper>")
}