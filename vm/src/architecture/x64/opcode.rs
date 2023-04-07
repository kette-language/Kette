pub struct Opcode;

impl Opcode {
    pub const MOV_REG_REG: u32 = 0x89;
    pub const MOV_REG_CONST_B1: u32 = 0xb0;
    pub const MOV_REG_CONST_B248: u32 = 0xb8;
    pub const MOV_REG_CONST_B8: u32 = 0xc7;

    pub const PUSH_REG: u32 = 0x50;
    pub const PUSH_CONST_B1: u32 = 0x6a;
    pub const PUSH_CONST_B4: u32 = 0x68;

    pub const POP_REG: u32 = 0x58;
}
