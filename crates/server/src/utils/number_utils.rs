pub fn u16_to_bytes(num: u16) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    res.push(((num >> 8) & 0xffu16) as u8);
    res.push((num & 0xffu16) as u8);

    res
}

pub fn u32_to_bytes(num: u32) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    res.push(((num >> 24) & 0xff) as u8);
    res.push(((num >> 16) & 0xff) as u8);
    res.push(((num >> 8) & 0xff) as u8);
    res.push((num & 0xff) as u8);

    res
}