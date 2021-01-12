use structview::{u32_be, u16_be, View};

#[derive(Clone, Copy, View)]
#[repr(C)]
pub struct TTFHeader {
    pub sfnt_version: u32_be,
    pub num_tables: u16_be,
    pub search_range: u16_be,
    pub entry_selector: u16_be,
    pub range_shift: u16_be,
}

impl TTFHeader {
    pub fn from_bytes(data: &[u8]) -> Result<&TTFHeader, structview::Error> {
        TTFHeader::view(&data)
    }
}

#[derive(Clone, Copy, View)]
#[repr(C)]
pub struct TTFTable {
    pub tag: [u8; 4],
}


pub struct TTF<'a, 'b> {
    data: &'a [u8],
    table_index: u8,
    pub header: &'b TTFHeader,
}

impl TTF<'_, '_> {
    pub fn from_bytes<'a>(data: &[u8]) -> Result<TTF, structview::Error> {
        let header = TTFHeader::from_bytes(data)?;

        Ok(
            TTF {
                data,
                table_index: 0,
                header
            }
        )
    }

    pub fn read_table(&self) -> Result<&TTFTable, structview::Error> {
        TTFTable::view(&self.data[12..])
    }
}
