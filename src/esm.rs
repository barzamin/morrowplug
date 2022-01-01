use bitflags::bitflags;

bitflags! {
    struct RecordFlags: u32 {
        const DELETED      = 0x0020;
        const PRESISTENT   = 0x0400;
        const INIT_DISABLE = 0x0800;
        const BLOCKED      = 0x2000;
    }
}
