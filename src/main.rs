mod config;
mod utils;
mod service;
mod model;

use service::db::table as table_service;
use service::entry as entry_service;

fn main() {

   let table_list: Vec<String> = table_service::get_tables();

   for table in table_list {
      let column_list = table_service::get_table_columns(&table);
      entry_service::gen_entry(&table, column_list);
      break;
   }
}