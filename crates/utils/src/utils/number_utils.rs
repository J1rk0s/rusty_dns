pub fn u16_to_bytes(num: u16) -> Vec<u8> {
    let res: Vec<u8> = vec![
        ((num >> 8) & 0xffu16) as u8,
        (num & 0xffu16) as u8
    ];

    res
}

pub fn u32_to_bytes(num: u32) -> Vec<u8> {
    let res: Vec<u8> = vec![
        ((num >> 24) & 0xff) as u8,
        ((num >> 16) & 0xff) as u8,
        ((num >> 8) & 0xff) as u8,
        (num & 0xff) as u8
    ];

    res
}