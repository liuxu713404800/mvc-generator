mod config;
mod utils;
mod service;
mod model;

use service::db::table as table_service;
use service::entry as entry_service;
use service::mapper as mapper_service;
use service::dto as dto_service;
use service::dao as dao_service;

fn main() {

   let table_list: Vec<String> = table_service::get_tables();

   for table in table_list {
      let column_list = table_service::get_table_columns(&table);
      entry_service::gen_entry(&table, &column_list);
      mapper_service::gen_mapper(&table, &column_list);
      dto_service::gen_filter(&table);
      dao_service::gen_xml(&table, &column_list);
      break;
   }
}