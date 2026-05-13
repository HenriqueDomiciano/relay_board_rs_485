#[must_use]
pub fn mod_bus_crc_calculation(command: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    let crc_polinomial: u16 = 0xA001;
    for &byte in command {
        crc ^= u16::from(byte);
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc = (crc >> 1) ^ crc_polinomial;
            } else {
                crc >>= 1;
            }
        }
    }
    u16::swap_bytes(crc)
}
