#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Default, Debug)]
pub struct imm {
    imm8: u8,
    imm16: u16,
    imm32: u32,
    imm64: u64,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Default, Debug)]
pub struct disp {
    disp8: u8,
    disp16: u16,
    disp32: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Default, Debug)]
pub struct hde64s {
    len: u8,
    p_rep: u8,
    p_lock: u8,
    p_seg: u8,
    p_66: u8,
    p_67: u8,
    rex: u8,
    rex_w: u8,
    rex_r: u8,
    rex_x: u8,
    rex_b: u8,
    pub opcode: u8,
    opcode2: u8,
    modrm: u8,
    modrm_mod: u8,
    modrm_reg: u8,
    modrm_rm: u8,
    sib: u8,
    sib_scale: u8,
    sib_index: u8,
    sib_base: u8,
    imm: imm,
    disp: disp,
    flags: u32,
}
