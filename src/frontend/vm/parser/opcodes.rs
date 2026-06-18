#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Stop
    Stop = 0x00,

    // Memory
    LoadConst = 0x01,
    LoadVar = 0x02,
    StoreVar = 0x03,

    // Math Int (i64)
    IAdd = 0x04,
    ISub = 0x05,
    IMul = 0x06,
    IDiv = 0x07,
    IMod = 0x08,
    IPow = 0x09,
    INegate = 0x0A,

    // Math Float (f64)
    FAdd = 0x0B,
    FSub = 0x0C,
    FMul = 0x0D,
    FDiv = 0x0E,
    FMod = 0x0F,
    FPow = 0x10,
    FNegate = 0x11,

    // Compare Int
    IEqual = 0x12,
    INotEqual = 0x13,
    ILess = 0x14,
    ILessEq = 0x15,
    IGreater = 0x16,
    IGreaterEq = 0x17,

    // Compare Float
    FEqual = 0x18,
    FNotEqual = 0x19,
    FLess = 0x1A,
    FLessEq = 0x1B,
    FGreater = 0x1C,
    FGreaterEq = 0x1D,

    // Control flow
    Jump = 0x1E,
    JumpIfFalse = 0x1F,

    // Logic / Bool
    And = 0x2A,
    Or = 0x2B,
    Not = 0x2C,

    // IO
    Print = 0x2D,
    Input = 0x2E,
}
