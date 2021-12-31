use std::env;

mod config;
mod utils;
mod service;
mod model;

use service::db::table as table_service;

fn main() {
   let table_list: Vec<String> = table_service::get_tables();
   let need_gen_tables = get_need_gen_tables(&table_list);
   for table in table_list {
      if !need_gen_tables.contains(&table) {
         continue;
      }
      let column_list = table_service::get_table_columns(&table);
      service::entry::gen_entry(&table, &column_list);
      service::mapper::gen_mapper(&table, &column_list);
      service::dto::gen_filter(&table);
      service::dao::gen_xml(&table, &column_list);
      service::vo::gen_vo(&table, &column_list);
   }
}

// 得到需要生成的表
fn get_need_gen_tables(table_list: &Vec<String>) -> Vec<String> {
   let mut res = Vec::new();
   let args: Vec<String> = env::args().collect();
   if args.len() < 2 {
      res = table_list.clone();
   } else {
      let mut i = 1;
      while i < args.len() {
         if table_list.contains(&args[i]) {
            res.push(String::from(&args[i]));
         }
         i += 1;
      }
   }
   res
}