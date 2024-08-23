use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct DisplayedSkinParts: u8 {
        const CAPE = 0b0000_0001;
        const JACKET = 0b0000_0010;
        const LEFT_SLEEVE = 0b0000_0100;
        const RIGHT_SLEEVE = 0b0000_1000;
        const LEFT_PANTS = 0b0001_0000;
        const RIGHT_PANTS = 0b0010_0000;
        const HAT = 0b0100_0000;
    }
}
