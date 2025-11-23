#[derive(Debug, Clone)]
pub struct DbHeader {
    pub page_size: u32,
}

pub struct TableLeafPage {
    pub header: PageHeader,
}

pub enum Page {
    TableLeaf(TableLeafPage),
}

pub struct PageHeader {
    cell_count: u16,
}
