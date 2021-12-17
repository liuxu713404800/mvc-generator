mod config;
mod utils;
mod service;


use service::db::table as table_service;

fn main() {
   let table_list: Vec<String> = table_service::get_tables();

   for table in table_list {
      let res = table_service::get_table_columns(table);
      println!("{:?}", res);
      break;


   }
}
