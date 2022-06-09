mod hde_types;
use hde_types::*;

#[link(name = "core", kind = "static")]
extern "C" {
    fn disasm_c(fn_addr: *const (), hs: *mut hde64s) -> u32;
    fn fnasm();
}

fn disasm(fn_addr: *const ()) -> (hde64s, u32) {
    let mut hs = hde64s::default();
    unsafe {
        let len = disasm_c(fn_addr, &mut hs);
        (hs, len)
    }
}

pub fn test_asm() {
    unsafe {
        fnasm();
    }
}

mod test {
    use super::*;

    #[test]
    fn disasm_test() {
        // sub rsp,28
        let test_code: [u8; 4] = [0x48, 0x83, 0xEC, 0x28];
        let (hs, len) =
            disasm(unsafe { std::mem::transmute::<*const [u8; 4], *const ()>(&test_code) });
            
        dbg!(len, hs);
    }

    #[test]
    fn test_call_asm() {
        test_asm();
    }
}
