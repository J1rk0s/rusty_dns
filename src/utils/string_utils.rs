use byteorder::WriteBytesExt;

pub fn str_dns_bytes(old: &str) -> std::io::Result<Vec<u8>> {
    let mut buff: Vec<u8> = vec![];
    let name_parts: Vec<&str> = old.split(".").collect();

    for part in name_parts {
        buff.write_u8(part.len() as u8)?;
        for c in part.chars() {
            buff.write_u8(c as u8)?;
        }
    }

    Ok(buff)
}