use anyhow::{bail, Result};
use codecrafters_sqlite::db::Db;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let _db = Db::from_file(&args[1])?;
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
