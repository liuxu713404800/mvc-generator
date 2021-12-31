mod config;
mod utils;
mod service;
mod model;

use service::db::table as table_service;

fn main() {

   let table_list: Vec<String> = table_service::get_tables();

   for table in table_list {
      let column_list = table_service::get_table_columns(&table);
      service::entry::gen_entry(&table, &column_list);
      service::mapper::gen_mapper(&table, &column_list);
      service::dto::gen_filter(&table);
      service::dao::gen_xml(&table, &column_list);
      service::vo::gen_vo(&table, &column_list);
      break;
   }
}