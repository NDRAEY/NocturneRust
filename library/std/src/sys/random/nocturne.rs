unsafe extern "C" {
    fn untrusted_randomgen() -> u32;
}

pub fn fill_bytes(bytes: &mut [u8]) {
    let mut chunks = &mut bytes.chunks_mut(4);

    for i in &mut chunks {
        let r: u32 = unsafe { untrusted_randomgen() };
        let bytes: &[u8] = &r.to_le_bytes();
        let copylen = i.len();

        i.copy_from_slice(&bytes[..copylen]);
    }
}
