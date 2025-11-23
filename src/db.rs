use std::{
    fs::File,
    io::{Read, Seek},
};

use anyhow::Context;

use crate::{
    page::DbHeader,
    pager::{read_cell_count, read_header, HEADER_SIZE},
};

pub struct Db {
    pub header: DbHeader,
}

impl Db {
    pub fn from_file(file_name: &str) -> anyhow::Result<Db> {
        let mut file = File::open(file_name).context("open db file")?;
        let mut buffer = [0; HEADER_SIZE];

        file.read_exact(&mut buffer).context("Reading header")?;
        let header = read_header(&buffer)?;
        let page_size = header.page_size as usize;

        println!("database page size: {}", page_size);

        let mut v = vec![0; page_size];
        file.seek(std::io::SeekFrom::Start(0)).context(" seek")?;
        file.read_exact(&mut v).context("WTF")?;
        let number_table = read_cell_count(&v);
        println!("number of tables: {}", number_table);
        Ok(Db { header })
    }
}
