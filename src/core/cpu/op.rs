use lazy_static::lazy_static;

use crate::core::cpu::AddressingMode;

#[derive(Clone, Copy, Debug)]
pub struct Op<'a> {
    pub hex: u8,
    pub name: &'a str,
    pub addressing_mode: AddressingMode,
    pub cycles: u8,
    pub size: u16,
}

impl Op<'_> {
    fn new(hex: u8, name: &str, addressing_mode: AddressingMode, cycles: u8, size: u16) -> Op {
        Op {
            hex,
            name,
            addressing_mode,
            cycles,
            size,
        }
    }
}

lazy_static! {
    pub static ref OPS: Vec<Op<'static>> = vec![
        Op::new(0x00, "BRK", AddressingMode::Implicit, 7, 1),
        Op::new(0x01, "ORA", AddressingMode::IndexedIndirect, 6, 2),
        Op::new(0x03, "*SLO", AddressingMode::IndexedIndirect, 8, 2),
        Op::new(0x04, "*NOP", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x05, "ORA", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x06, "ASL", AddressingMode::ZeroPage, 5, 2),
        Op::new(0x07, "*SLO", AddressingMode::ZeroPage, 5, 2),
        Op::new(0x08, "PHP", AddressingMode::Implicit, 3, 1),
        Op::new(0x09, "ORA", AddressingMode::Immediate, 2, 2),
        Op::new(0x0a, "ASL", AddressingMode::Accumulator, 2, 1),
        Op::new(0x0b, "*ANC", AddressingMode::Immediate, 2, 2),
        Op::new(0x0c, "*NOP", AddressingMode::Absolute, 4, 3),
        Op::new(0x0d, "ORA", AddressingMode::Absolute, 4, 3),
        Op::new(0x0e, "ASL", AddressingMode::Absolute, 6, 3),
        Op::new(0x0f, "*SLO", AddressingMode::Absolute, 6, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0x10, "BPL", AddressingMode::Relative, 2, 2),
        Op::new(0x11, "ORA", AddressingMode::IndirectIndexed, 5, 2),
        Op::new(0x13, "*SLO", AddressingMode::IndirectIndexedW, 8, 2),
        Op::new(0x14, "*NOP", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0x15, "ORA", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0x16, "ASL", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0x17, "*SLO", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0x18, "CLC", AddressingMode::Implicit, 2, 1),
        Op::new(0x19, "ORA", AddressingMode::AbsoluteY, 4, 3),
        Op::new(0x1a, "*NOP", AddressingMode::Implicit, 2, 1),
        Op::new(0x1b, "*SLO", AddressingMode::AbsoluteYW, 7, 3),
        Op::new(0x1c, "*NOP", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0x1d, "ORA", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0x1e, "ASL", AddressingMode::AbsoluteXW, 7, 3),
        Op::new(0x1f, "*SLO", AddressingMode::AbsoluteXW, 7, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0x20, "JSR", AddressingMode::Absolute, 6, 3),
        Op::new(0x21, "AND", AddressingMode::IndexedIndirect, 6, 2),
        Op::new(0x23, "*RLA", AddressingMode::IndexedIndirect, 8, 2),
        Op::new(0x24, "BIT", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x25, "AND", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x26, "ROL", AddressingMode::ZeroPage, 5, 2),
        Op::new(0x27, "*RLA", AddressingMode::ZeroPage, 5, 2),
        Op::new(0x28, "PLP", AddressingMode::Implicit, 4, 1),
        Op::new(0x29, "AND", AddressingMode::Immediate, 2, 2),
        Op::new(0x2a, "ROL", AddressingMode::Accumulator, 2, 1),
        Op::new(0x2b, "*ANC", AddressingMode::Immediate, 2, 2),
        Op::new(0x2c, "BIT", AddressingMode::Absolute, 4, 3),
        Op::new(0x2d, "AND", AddressingMode::Absolute, 4, 3),
        Op::new(0x2e, "ROL", AddressingMode::Absolute, 6, 3),
        Op::new(0x2f, "*RLA", AddressingMode::Absolute, 6, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0x30, "BMI", AddressingMode::Relative, 2, 2),
        Op::new(0x31, "AND", AddressingMode::IndirectIndexed, 5, 2),
        Op::new(0x33, "*RLA", AddressingMode::IndirectIndexedW, 8, 2),
        Op::new(0x34, "*NOP", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0x35, "AND", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0x36, "ROL", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0x37, "*RLA", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0x38, "SEC", AddressingMode::Implicit, 2, 1),
        Op::new(0x39, "AND", AddressingMode::AbsoluteY, 4, 3),
        Op::new(0x3a, "*NOP", AddressingMode::Implicit, 2, 1),
        Op::new(0x3b, "*RLA", AddressingMode::AbsoluteYW, 7, 3),
        Op::new(0x3c, "*NOP", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0x3d, "AND", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0x3e, "ROL", AddressingMode::AbsoluteXW, 7, 3),
        Op::new(0x3f, "*RLA", AddressingMode::AbsoluteXW, 7, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0x40, "RTI", AddressingMode::Implicit, 6, 1),
        Op::new(0x41, "EOR", AddressingMode::IndexedIndirect, 6, 2),
        Op::new(0x43, "*SRE", AddressingMode::IndexedIndirect, 8, 2),
        Op::new(0x44, "*NOP", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x45, "EOR", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x46, "LSR", AddressingMode::ZeroPage, 5, 2),
        Op::new(0x47, "*SRE", AddressingMode::ZeroPage, 5, 2),
        Op::new(0x48, "PHA", AddressingMode::Implicit, 3, 1),
        Op::new(0x49, "EOR", AddressingMode::Immediate, 2, 2),
        Op::new(0x4a, "LSR", AddressingMode::Accumulator, 2, 1),
        Op::new(0x4b, "*ASR", AddressingMode::Immediate, 2, 2),
        Op::new(0x4c, "JMP", AddressingMode::Absolute, 3, 3),
        Op::new(0x4d, "EOR", AddressingMode::Absolute, 4, 3),
        Op::new(0x4e, "LSR", AddressingMode::Absolute, 6, 3),
        Op::new(0x4f, "*SRE", AddressingMode::Absolute, 6, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0x50, "BVC", AddressingMode::Relative, 2, 2),
        Op::new(0x51, "EOR", AddressingMode::IndirectIndexed, 5, 2),
        Op::new(0x53, "*SRE", AddressingMode::IndirectIndexedW, 8, 2),
        Op::new(0x54, "*NOP", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0x55, "EOR", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0x56, "LSR", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0x57, "*SRE", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0x58, "CLI", AddressingMode::Implicit, 2, 1),
        Op::new(0x59, "EOR", AddressingMode::AbsoluteY, 4, 3),
        Op::new(0x5a, "*NOP", AddressingMode::Implicit, 2, 1),
        Op::new(0x5b, "*SRE", AddressingMode::AbsoluteYW, 7, 3),
        Op::new(0x5c, "*NOP", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0x5d, "EOR", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0x5e, "LSR", AddressingMode::AbsoluteXW, 7, 3),
        Op::new(0x5f, "*SRE", AddressingMode::AbsoluteXW, 7, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0x60, "RTS", AddressingMode::Implicit, 6, 1),
        Op::new(0x61, "ADC", AddressingMode::IndexedIndirect, 6, 2),
        Op::new(0x63, "*RRA", AddressingMode::IndexedIndirect, 8, 2),
        Op::new(0x64, "*NOP", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x65, "ADC", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x66, "ROR", AddressingMode::ZeroPage, 5, 2),
        Op::new(0x67, "*RRA", AddressingMode::ZeroPage, 5, 2),
        Op::new(0x68, "PLA", AddressingMode::Implicit, 4, 1),
        Op::new(0x69, "ADC", AddressingMode::Immediate, 2, 2),
        Op::new(0x6a, "ROR", AddressingMode::Accumulator, 2, 1),
        Op::new(0x6b, "*ARR", AddressingMode::Immediate, 2, 2),
        Op::new(0x6c, "JMP", AddressingMode::Indirect, 5, 3),
        Op::new(0x6d, "ADC", AddressingMode::Absolute, 4, 3),
        Op::new(0x6e, "ROR", AddressingMode::Absolute, 6, 3),
        Op::new(0x6f, "*RRA", AddressingMode::Absolute, 6, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0x70, "BVS", AddressingMode::Relative, 2, 2),
        Op::new(0x71, "ADC", AddressingMode::IndirectIndexed, 5, 2),
        Op::new(0x73, "*RRA", AddressingMode::IndirectIndexedW, 8, 2),
        Op::new(0x74, "*NOP", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0x75, "ADC", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0x76, "ROR", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0x77, "*RRA", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0x78, "SEI", AddressingMode::Implicit, 2, 1),
        Op::new(0x79, "ADC", AddressingMode::AbsoluteY, 4, 3),
        Op::new(0x7a, "*NOP", AddressingMode::Implicit, 2, 1),
        Op::new(0x7b, "*RRA", AddressingMode::AbsoluteYW, 7, 3),
        Op::new(0x7c, "*NOP", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0x7d, "ADC", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0x7e, "ROR", AddressingMode::AbsoluteXW, 7, 3),
        Op::new(0x7f, "*RRA", AddressingMode::AbsoluteXW, 7, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0x80, "*NOP", AddressingMode::Immediate, 2, 2),
        Op::new(0x81, "STA", AddressingMode::IndexedIndirect, 6, 2),
        Op::new(0x82, "*NOP", AddressingMode::Immediate, 2, 2),
        Op::new(0x83, "*SAX", AddressingMode::IndexedIndirect, 6, 2),
        Op::new(0x84, "STY", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x85, "STA", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x86, "STX", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x87, "*SAX", AddressingMode::ZeroPage, 3, 2),
        Op::new(0x88, "DEY", AddressingMode::Implicit, 2, 1),
        Op::new(0x89, "*NOP", AddressingMode::Immediate, 2, 2),
        Op::new(0x8a, "TXA", AddressingMode::Implicit, 2, 1),
        Op::new(0x8c, "STY", AddressingMode::Absolute, 4, 3),
        Op::new(0x8d, "STA", AddressingMode::Absolute, 4, 3),
        Op::new(0x8e, "STX", AddressingMode::Absolute, 4, 3),
        Op::new(0x8f, "*SAX", AddressingMode::Absolute, 4, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0x90, "BCC", AddressingMode::Relative, 2, 2),
        Op::new(0x91, "STA", AddressingMode::IndirectIndexedW, 6, 2),
        Op::new(0x94, "STY", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0x95, "STA", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0x96, "STX", AddressingMode::ZeroPageY, 4, 2),
        Op::new(0x97, "*SAX", AddressingMode::ZeroPageY, 4, 2),
        Op::new(0x98, "TYA", AddressingMode::Implicit, 2, 1),
        Op::new(0x99, "STA", AddressingMode::AbsoluteYW, 5, 3),
        Op::new(0x9a, "TXS", AddressingMode::Implicit, 2, 1),
        Op::new(0x9c, "*SHY", AddressingMode::AbsoluteXW, 5, 3),
        Op::new(0x9d, "STA", AddressingMode::AbsoluteXW, 5, 3),
        Op::new(0x9e, "*SHX", AddressingMode::AbsoluteYW, 5, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0xa0, "LDY", AddressingMode::Immediate, 2, 2),
        Op::new(0xa1, "LDA", AddressingMode::IndexedIndirect, 6, 2),
        Op::new(0xa2, "LDX", AddressingMode::Immediate, 2, 2),
        Op::new(0xa3, "*LAX", AddressingMode::IndexedIndirect, 6, 2),
        Op::new(0xa4, "LDY", AddressingMode::ZeroPage, 3, 2),
        Op::new(0xa5, "LDA", AddressingMode::ZeroPage, 3, 2),
        Op::new(0xa6, "LDX", AddressingMode::ZeroPage, 3, 2),
        Op::new(0xa7, "*LAX", AddressingMode::ZeroPage, 3, 2),
        Op::new(0xa8, "TAY", AddressingMode::Implicit, 2, 1),
        Op::new(0xa9, "LDA", AddressingMode::Immediate, 2, 2),
        Op::new(0xaa, "TAX", AddressingMode::Implicit, 2, 1),
        Op::new(0xab, "*LXA", AddressingMode::Immediate, 2, 2),
        Op::new(0xac, "LDY", AddressingMode::Absolute, 4, 3),
        Op::new(0xad, "LDA", AddressingMode::Absolute, 4, 3),
        Op::new(0xae, "LDX", AddressingMode::Absolute, 4, 3),
        Op::new(0xaf, "*LAX", AddressingMode::Absolute, 4, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0xb0, "BCS", AddressingMode::Relative, 2, 2),
        Op::new(0xb1, "LDA", AddressingMode::IndirectIndexed, 5, 2),
        Op::new(0xb3, "*LAX", AddressingMode::IndirectIndexed, 5, 2),
        Op::new(0xb4, "LDY", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0xb5, "LDA", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0xb6, "LDX", AddressingMode::ZeroPageY, 4, 2),
        Op::new(0xb7, "*LAX", AddressingMode::ZeroPageY, 4, 2),
        Op::new(0xb8, "CLV", AddressingMode::Implicit, 2, 1),
        Op::new(0xb9, "LDA", AddressingMode::AbsoluteY, 4, 3),
        Op::new(0xba, "TSX", AddressingMode::Implicit, 2, 1),
        Op::new(0xbc, "LDY", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0xbd, "LDA", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0xbe, "LDX", AddressingMode::AbsoluteY, 4, 3),
        Op::new(0xbf, "*LAX", AddressingMode::AbsoluteY, 4, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0xc0, "CPY", AddressingMode::Immediate, 2, 2),
        Op::new(0xc1, "CMP", AddressingMode::IndexedIndirect, 6, 2),
        Op::new(0xc2, "*NOP", AddressingMode::Immediate, 2, 2),
        Op::new(0xc3, "*DCP", AddressingMode::IndexedIndirect, 8, 2),
        Op::new(0xc4, "CPY", AddressingMode::ZeroPage, 3, 2),
        Op::new(0xc5, "CMP", AddressingMode::ZeroPage, 3, 2),
        Op::new(0xc6, "DEC", AddressingMode::ZeroPage, 5, 2),
        Op::new(0xc7, "*DCP", AddressingMode::ZeroPage, 5, 2),
        Op::new(0xc8, "INY", AddressingMode::Implicit, 2, 1),
        Op::new(0xc9, "CMP", AddressingMode::Immediate, 2, 2),
        Op::new(0xca, "DEX", AddressingMode::Implicit, 2, 1),
        Op::new(0xcb, "*AXS", AddressingMode::Immediate, 2, 2),
        Op::new(0xcc, "CPY", AddressingMode::Absolute, 4, 3),
        Op::new(0xcd, "CMP", AddressingMode::Absolute, 4, 3),
        Op::new(0xce, "DEC", AddressingMode::Absolute, 6, 3),
        Op::new(0xcf, "*DCP", AddressingMode::Absolute, 6, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0xd0, "BNE", AddressingMode::Relative, 2, 2),
        Op::new(0xd1, "CMP", AddressingMode::IndirectIndexed, 5, 2),
        Op::new(0xd3, "*DCP", AddressingMode::IndirectIndexedW, 8, 2),
        Op::new(0xd4, "*NOP", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0xd5, "CMP", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0xd6, "DEC", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0xd7, "*DCP", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0xd8, "CLD", AddressingMode::Implicit, 2, 1),
        Op::new(0xd9, "CMP", AddressingMode::AbsoluteY, 4, 3),
        Op::new(0xda, "*NOP", AddressingMode::Implicit, 2, 1),
        Op::new(0xdb, "*DCP", AddressingMode::AbsoluteYW, 7, 3),
        Op::new(0xdc, "*NOP", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0xdd, "CMP", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0xde, "DEC", AddressingMode::AbsoluteXW, 7, 3),
        Op::new(0xdf, "*DCP", AddressingMode::AbsoluteXW, 7, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0xe0, "CPX", AddressingMode::Immediate, 2, 2),
        Op::new(0xe1, "SBC", AddressingMode::IndexedIndirect, 6, 2),
        Op::new(0xe2, "*NOP", AddressingMode::Immediate, 2, 2),
        Op::new(0xe3, "*ISB", AddressingMode::IndexedIndirect, 8, 2),
        Op::new(0xe4, "CPX", AddressingMode::ZeroPage, 3, 2),
        Op::new(0xe5, "SBC", AddressingMode::ZeroPage, 3, 2),
        Op::new(0xe6, "INC", AddressingMode::ZeroPage, 5, 2),
        Op::new(0xe7, "*ISB", AddressingMode::ZeroPage, 5, 2),
        Op::new(0xe8, "INX", AddressingMode::Implicit, 2, 1),
        Op::new(0xe9, "SBC", AddressingMode::Immediate, 2, 2),
        Op::new(0xea, "NOP", AddressingMode::Implicit, 2, 1),
        Op::new(0xeb, "*SBC", AddressingMode::Immediate, 2, 2),
        Op::new(0xec, "CPX", AddressingMode::Absolute, 4, 3),
        Op::new(0xed, "SBC", AddressingMode::Absolute, 4, 3),
        Op::new(0xee, "INC", AddressingMode::Absolute, 6, 3),
        Op::new(0xef, "*ISB", AddressingMode::Absolute, 6, 3),
        // ---------------------------------------------------------------------------------------------
        Op::new(0xf0, "BEQ", AddressingMode::Relative, 2, 2),
        Op::new(0xf1, "SBC", AddressingMode::IndirectIndexed, 5, 2),
        Op::new(0xf3, "*ISB", AddressingMode::IndirectIndexedW, 8, 2),
        Op::new(0xf4, "*NOP", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0xf5, "SBC", AddressingMode::ZeroPageX, 4, 2),
        Op::new(0xf6, "INC", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0xf7, "*ISB", AddressingMode::ZeroPageX, 6, 2),
        Op::new(0xf8, "SED", AddressingMode::Implicit, 2, 1),
        Op::new(0xf9, "SBC", AddressingMode::AbsoluteY, 4, 3),
        Op::new(0xfa, "*NOP", AddressingMode::Implicit, 2, 1),
        Op::new(0xfb, "*ISB", AddressingMode::AbsoluteYW, 7, 3),
        Op::new(0xfc, "*NOP", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0xfd, "SBC", AddressingMode::AbsoluteX, 4, 3),
        Op::new(0xfe, "INC", AddressingMode::AbsoluteXW, 7, 3),
        Op::new(0xff, "*ISB", AddressingMode::AbsoluteXW, 7, 3),
    ];
}
