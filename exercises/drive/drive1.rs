// drive1.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.
fn modify_by_address(address: usize) {
    unsafe {
        let value_ptr: *mut u32 = address as *mut u32;
        *value_ptr = 0xAABBCCDD;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t:u32 = 0x12345678;
        modify_by_address(&mut t as *mut u32 as usize);
        assert!(t == 0xAABBCCDD);
    }
}
