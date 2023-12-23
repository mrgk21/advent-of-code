
pub fn is_number(val: u8) -> bool {
    val >= b'0' && val <= b'9'
}

pub fn is_symbol(val: u8) -> bool {
    !is_number(val) && !is_period(val)
}

pub fn is_period(val: u8) -> bool {
    val == b'.'
}
