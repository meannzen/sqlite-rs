use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return;
    }
    let path = &args[1];

    if args[2] == ".dbinfo" {
        let mut file = File::open(path).unwrap();
        let mut buffer = [0; 100];
        file.read_exact(&mut buffer).unwrap();
        let page_size = u16::from_be_bytes(buffer[16..18].try_into().unwrap());
        println!("database page size: {}", page_size);
        let mut buffer = vec![0; page_size as usize];
        file.seek(SeekFrom::Start(0)).unwrap();
        file.read_exact(&mut buffer).unwrap();
        let cell_count = u16::from_be_bytes(buffer[103..105].try_into().unwrap());
        println!("number of tables: {}", cell_count);
        return;
    }

    if args[2] != ".tables" {
        return;
    }

    let data = fs::read(path).unwrap();
    let page_size = u16::from_be_bytes([data[16], data[17]]) as usize;
    let page_size = if page_size == 1 { 65536 } else { page_size };
    let page = &data[0..page_size];

    let mut tables = Vec::new();
    let header_offset = 100;
    let cell_count =
        u16::from_be_bytes([page[header_offset + 3], page[header_offset + 4]]) as usize;

    for i in 0..cell_count {
        let ptr_offset = header_offset + 8 + i * 2;
        let cell_offset = u16::from_be_bytes([page[ptr_offset], page[ptr_offset + 1]]) as usize;

        if let Some(name) = parse_cell(&page[cell_offset..]) {
            if !name.starts_with("sqlite_") {
                tables.push(name);
            }
        }
    }

    tables.sort_unstable();
    if !tables.is_empty() {
        println!("{}", tables.join(" "));
    }
}

fn parse_cell(cell: &[u8]) -> Option<String> {
    let (_, mut i) = read_varint(cell);
    let (_, n) = read_varint(&cell[i..]);
    i += n;

    let (header_size, n) = read_varint(&cell[i..]);
    i += n;
    let mut serial_types = Vec::new();
    let mut remaining = header_size as usize - n;
    while remaining > 0 {
        let (st, n) = read_varint(&cell[i..]);
        serial_types.push(st);
        i += n;
        remaining -= n;
    }

    if serial_types.len() < 3 {
        return None;
    }
    if serial_types[0] < 13 || serial_types[0] % 2 == 0 {
        return None;
    }
    if serial_types[2] < 13 || serial_types[2] % 2 == 0 {
        return None;
    }

    let type_len = ((serial_types[0] - 13) / 2) as usize;
    let name_len = ((serial_types[1] - 13) / 2) as usize;
    let tbl_name_len = ((serial_types[2] - 13) / 2) as usize;

    i += type_len + name_len;
    if i + tbl_name_len > cell.len() {
        return None;
    }

    let type_str = String::from_utf8_lossy(&cell[i - type_len - name_len..i - name_len]);
    if type_str != "table" {
        return None;
    }

    Some(String::from_utf8_lossy(&cell[i..i + tbl_name_len]).to_string())
}

fn read_varint(data: &[u8]) -> (u64, usize) {
    let mut value = 0u64;
    let mut shift = 0;
    for (j, &b) in data.iter().enumerate().take(9) {
        value |= (b as u64 & 0x7F) << shift;
        shift += 7;
        if b & 0x80 == 0 {
            return (value, j + 1);
        }
    }
    (value, 9)
}
