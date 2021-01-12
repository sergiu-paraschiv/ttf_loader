#[cfg(test)]
mod tests {
    use ttf_loader::TTF;

    #[test]
    fn it_loads_bytes() {
        const FONT: &[u8] = include_bytes!("./fonts/Roboto-Regular.ttf");

        let view = TTF::from_bytes(FONT).unwrap();
        assert_eq!(0x00010000, view.header.sfnt_version.to_int());
        assert_eq!(18, view.header.num_tables.to_int());
        assert_eq!(256, view.header.search_range.to_int());
        assert_eq!(4, view.header.entry_selector.to_int());
        assert_eq!(32, view.header.range_shift.to_int());

        let first_table = view.read_table().unwrap();
        assert_eq!(*b"GDEF", first_table.tag);
    }
}
