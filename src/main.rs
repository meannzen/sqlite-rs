use codecrafters_sqlite::db;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Ok(());
    }
    let path = &args[1];
    let command = args[2].as_str();

    match command {
        ".dbinfo" => {
            let info = db::db_info(path)?;
            println!("database page size: {}", info.page_size);
            println!("number of tables: {}", info.number_of_tables);
        }
        ".tables" => {
            let tables = db::list_tables(path)?;
            if !tables.is_empty() {
                println!("{}", tables.join(" "));
            }
        }
        string => {
            dbg!(string);
        }
    }
    Ok(())
}
