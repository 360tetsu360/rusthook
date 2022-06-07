#[link(name = "core", kind = "static")]
extern "C" {
  fn fncore();
}

pub fn test() {
    unsafe {
        fncore();
    }
}

mod test {
    use crate::test;

    #[test]
    fn call_cxx_asm() {
        test()
    }
}