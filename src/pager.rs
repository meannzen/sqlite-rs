use crate::page::DbHeader;

pub const HEADER_SIZE: usize = 100;
const HEADER_PAGE_SIZE_OFFET: usize = 16;
const MAX_PAGE_SIZE: u32 = 32768;

const CELL_COUNT_OFFSET: usize = HEADER_SIZE + 3;

pub fn read_header(buffer: &[u8]) -> anyhow::Result<DbHeader> {
    let page_size_at = read_u16(buffer, HEADER_PAGE_SIZE_OFFET);

    let page_size = match page_size_at {
        1 => MAX_PAGE_SIZE,
        n if ((n & (n - 1)) == 0) && n != 0 => n as u32,
        _ => anyhow::bail!("page size is not a power of 2: {}", page_size_at),
    };

    Ok(DbHeader { page_size })
}

pub fn read_cell_count(buffer: &[u8]) -> u16 {
    read_u16(buffer, CELL_COUNT_OFFSET)
}

fn read_u16(buffer: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes(buffer[offset..offset + 2].try_into().unwrap())
}
