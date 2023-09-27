pub mod program {
    use crate::utils::{CowBytes, CowString};
    use crate::varint::{read_varint, write_varint, MAX_VARINT_LENGTH};
    use core::ops::Range;
    pub enum ExternTy {
        I32 = 1,
        I64 = 2,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ExternTy {}
    #[automatically_derived]
    impl ::core::clone::Clone for ExternTy {
        #[inline]
        fn clone(&self) -> ExternTy {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ExternTy {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ExternTy {
        #[inline]
        fn eq(&self, other: &ExternTy) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for ExternTy {}
    #[automatically_derived]
    impl ::core::cmp::Eq for ExternTy {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ExternTy {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ExternTy::I32 => "I32",
                    ExternTy::I64 => "I64",
                },
            )
        }
    }
    impl core::fmt::Display for ExternTy {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            let name = match *self {
                ExternTy::I32 => "i32",
                ExternTy::I64 => "i64",
            };
            fmt.write_str(name)
        }
    }
    impl ExternTy {
        pub fn try_deserialize(value: u8) -> Option<Self> {
            use ExternTy::*;
            match value {
                1 => Some(I32),
                2 => Some(I64),
                _ => None,
            }
        }
    }
    #[repr(u8)]
    pub enum Reg {
        Zero = 0,
        RA = 1,
        SP = 2,
        T0 = 3,
        T1 = 4,
        T2 = 5,
        S0 = 6,
        S1 = 7,
        A0 = 8,
        A1 = 9,
        A2 = 10,
        A3 = 11,
        A4 = 12,
        A5 = 13,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Reg {}
    #[automatically_derived]
    impl ::core::clone::Clone for Reg {
        #[inline]
        fn clone(&self) -> Reg {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Reg {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Reg {
        #[inline]
        fn eq(&self, other: &Reg) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Reg {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Reg {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Reg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Reg::Zero => "Zero",
                    Reg::RA => "RA",
                    Reg::SP => "SP",
                    Reg::T0 => "T0",
                    Reg::T1 => "T1",
                    Reg::T2 => "T2",
                    Reg::S0 => "S0",
                    Reg::S1 => "S1",
                    Reg::A0 => "A0",
                    Reg::A1 => "A1",
                    Reg::A2 => "A2",
                    Reg::A3 => "A3",
                    Reg::A4 => "A4",
                    Reg::A5 => "A5",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Reg {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    impl Reg {
        #[inline]
        pub const fn from_u8(value: u8) -> Option<Reg> {
            match value {
                0 => Some(Reg::Zero),
                1 => Some(Reg::RA),
                2 => Some(Reg::SP),
                3 => Some(Reg::T0),
                4 => Some(Reg::T1),
                5 => Some(Reg::T2),
                6 => Some(Reg::S0),
                7 => Some(Reg::S1),
                8 => Some(Reg::A0),
                9 => Some(Reg::A1),
                10 => Some(Reg::A2),
                11 => Some(Reg::A3),
                12 => Some(Reg::A4),
                13 => Some(Reg::A5),
                _ => None,
            }
        }
        pub const fn name(self) -> &'static str {
            use Reg::*;
            match self {
                Zero => "zero",
                RA => "ra",
                SP => "sp",
                T0 => "t0",
                T1 => "t1",
                T2 => "t2",
                S0 => "s0",
                S1 => "s1",
                A0 => "a0",
                A1 => "a1",
                A2 => "a2",
                A3 => "a3",
                A4 => "a4",
                A5 => "a5",
            }
        }
        /// List of all of the VM's registers, except the zero register.
        pub const ALL_NON_ZERO: [Reg; 13] = {
            use Reg::*;
            [RA, SP, T0, T1, T2, S0, S1, A0, A1, A2, A3, A4, A5]
        };
        /// List of all argument registers.
        pub const ARG_REGS: [Reg; 6] = [Reg::A0, Reg::A1, Reg::A2, Reg::A3, Reg::A4, Reg::A5];
    }
    impl core::fmt::Display for Reg {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.write_str(self.name())
        }
    }
    pub trait InstructionVisitor {
        type ReturnTy;
        fn trap(&mut self) -> Self::ReturnTy;
        fn jump_target(&mut self, imm: u32) -> Self::ReturnTy;
        fn ecalli(&mut self, imm: u32) -> Self::ReturnTy;
        fn and(&mut self, reg1: Reg, reg2: Reg, reg3: Reg) -> Self::ReturnTy;
        fn add_imm(&mut self, reg1: Reg, reg2: Reg, imm: u32) -> Self::ReturnTy;
    }
    impl RawInstruction {
        pub fn visit<T>(self, visitor: &mut T) -> T::ReturnTy
        where
            T: InstructionVisitor,
        {
            match self.op {
                0b00_000000 => visitor.trap(),
                0b01_000000 => visitor.jump_target(self.imm_or_reg),
                0b01_111111 => visitor.ecalli(self.imm_or_reg),
                0b10_000110 => visitor.and(self.reg1(), self.reg2(), self.reg3()),
                0b11_001000 => visitor.add_imm(self.reg1(), self.reg2(), self.imm_or_reg),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
    impl core::fmt::Display for RawInstruction {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            self.visit(fmt)
        }
    }
    #[allow(non_camel_case_types)]
    #[repr(u8)]
    pub enum Opcode {
        trap = 0b00_000000,
        jump_target = 0b01_000000,
        ecalli = 0b01_111111,
        and = 0b10_000110,
        add_imm = 0b11_001000,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for Opcode {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for Opcode {
        #[inline]
        fn clone(&self) -> Opcode {
            *self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for Opcode {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for Opcode {
        #[inline]
        fn eq(&self, other: &Opcode) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralEq for Opcode {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::Eq for Opcode {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::fmt::Debug for Opcode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Opcode::trap => "trap",
                    Opcode::jump_target => "jump_target",
                    Opcode::ecalli => "ecalli",
                    Opcode::and => "and",
                    Opcode::add_imm => "add_imm",
                },
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::hash::Hash for Opcode {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    impl Opcode {
        pub fn from_u8(byte: u8) -> Option<Opcode> {
            match byte {
                0b00_000000 => Some(Opcode::trap),
                0b01_000000 => Some(Opcode::jump_target),
                0b01_111111 => Some(Opcode::ecalli),
                0b10_000110 => Some(Opcode::and),
                0b11_001000 => Some(Opcode::add_imm),
                _ => None,
            }
        }
    }
    const IS_INSTRUCTION_VALID_CONST: [bool; 256] = {
        let mut is_valid = [false; 256];
        is_valid[0b00_000000] = true;
        is_valid[0b01_000000] = true;
        is_valid[0b01_111111] = true;
        is_valid[0b10_000110] = true;
        is_valid[0b11_001000] = true;
        is_valid
    };
    #[cfg(not(feature = "alloc"))]
    use IS_INSTRUCTION_VALID_CONST as IS_INSTRUCTION_VALID;
    pub const MAX_INSTRUCTION_LENGTH: usize = MAX_VARINT_LENGTH + 2;
    pub struct RawInstruction {
        op: u8,
        regs: u8,
        imm_or_reg: u32,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for RawInstruction {}
    #[automatically_derived]
    impl ::core::clone::Clone for RawInstruction {
        #[inline]
        fn clone(&self) -> RawInstruction {
            let _: ::core::clone::AssertParamIsClone<u8>;
            let _: ::core::clone::AssertParamIsClone<u32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RawInstruction {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RawInstruction {
        #[inline]
        fn eq(&self, other: &RawInstruction) -> bool {
            self.op == other.op && self.regs == other.regs && self.imm_or_reg == other.imm_or_reg
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for RawInstruction {}
    #[automatically_derived]
    impl ::core::cmp::Eq for RawInstruction {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u8>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    impl core::fmt::Debug for RawInstruction {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.write_fmt(format_args!(
                "({0:02x} {1:02x} {2:08x}) {3}",
                self.op, self.regs, self.imm_or_reg, self
            ))
        }
    }
    impl<'a> InstructionVisitor for core::fmt::Formatter<'a> {
        type ReturnTy = core::fmt::Result;
        fn trap(&mut self) -> Self::ReturnTy {
            self.write_fmt(format_args!("trap"))
        }
        fn jump_target(&mut self, pcrel: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!("@{0:x}:", pcrel * 4))
        }
        fn ecalli(&mut self, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!("ecalli {0}", imm))
        }
        fn set_less_than_unsigned(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} <u {2}", d, s1, s2))
        }
        fn set_less_than_signed(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} <s {2}", d, s1, s2))
        }
        fn shift_logical_right(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} >> {2}", d, s1, s2))
        }
        fn shift_arithmetic_right(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} >>a {2}", d, s1, s2))
        }
        fn shift_logical_left(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} << {2}", d, s1, s2))
        }
        fn xor(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} ^ {2}", d, s1, s2))
        }
        fn and(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} & {2}", d, s1, s2))
        }
        fn or(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} | {2}", d, s1, s2))
        }
        fn add(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} + {2}", d, s1, s2))
        }
        fn sub(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} - {2}", d, s1, s2))
        }
        fn mul(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} * {2}", d, s1, s2))
        }
        fn mul_upper_signed_signed(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!(
                "{0} = ({1} as i64 * {2} as i64) >> 32",
                d, s1, s2
            ))
        }
        fn mul_upper_unsigned_unsigned(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!(
                "{0} = ({1} as u64 * {2} as u64) >> 32",
                d, s1, s2
            ))
        }
        fn mul_upper_signed_unsigned(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!(
                "{0} = ({1} as i64 * {2} as u64) >> 32",
                d, s1, s2
            ))
        }
        fn div_unsigned(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} /u {2}", d, s1, s2))
        }
        fn div_signed(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} /s {2}", d, s1, s2))
        }
        fn rem_unsigned(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} %u {2}", d, s1, s2))
        }
        fn rem_signed(&mut self, d: Reg, s1: Reg, s2: Reg) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} %s {2}", d, s1, s2))
        }
        fn set_less_than_unsigned_imm(&mut self, dst: Reg, src: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} <u 0x{2:x}", dst, src, imm))
        }
        fn set_less_than_signed_imm(&mut self, dst: Reg, src: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} <s {2}", dst, src, imm as i32))
        }
        fn shift_logical_right_imm(&mut self, d: Reg, s: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} >> {2}", d, s, imm))
        }
        fn shift_arithmetic_right_imm(&mut self, d: Reg, s: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} >>a {2}", d, s, imm))
        }
        fn shift_logical_left_imm(&mut self, d: Reg, s: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} << {2}", d, s, imm))
        }
        fn or_imm(&mut self, d: Reg, s: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} | 0x{2:x}", d, s, imm))
        }
        fn and_imm(&mut self, d: Reg, s: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} & 0x{2:x}", d, s, imm))
        }
        fn xor_imm(&mut self, d: Reg, s: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!("{0} = {1} ^ 0x{2:x}", d, s, imm))
        }
        fn add_imm(&mut self, d: Reg, s: Reg, imm: u32) -> Self::ReturnTy {
            if d == Reg::Zero && s == Reg::Zero && imm == 0 {
                self.write_fmt(format_args!("nop"))
            } else if imm == 0 {
                self.write_fmt(format_args!("{0} = {1}", d, s))
            } else if (imm as i32) < 0 && (imm as i32) > -4096 {
                let imm_s = -(imm as i32);
                if s == Reg::Zero {
                    self.write_fmt(format_args!("{0} = -{1}", d, imm_s))
                } else {
                    self.write_fmt(format_args!("{0} = {1} - {2}", d, s, imm_s))
                }
            } else if s == Reg::Zero {
                self.write_fmt(format_args!("{0} = 0x{1:x}", d, imm))
            } else {
                self.write_fmt(format_args!("{0} = {1} + 0x{2:x}", d, s, imm))
            }
        }
        fn store_u8(&mut self, src: Reg, base: Reg, offset: u32) -> Self::ReturnTy {
            if base != Reg::Zero {
                if offset != 0 {
                    self.write_fmt(format_args!("u8 [{0} + {1}] = {2}", base, offset, src))
                } else {
                    self.write_fmt(format_args!("u8 [{0}] = {1}", base, src))
                }
            } else {
                self.write_fmt(format_args!("u8 [0x{0:x}] = {1}", offset, src))
            }
        }
        fn store_u16(&mut self, src: Reg, base: Reg, offset: u32) -> Self::ReturnTy {
            if base != Reg::Zero {
                if offset != 0 {
                    self.write_fmt(format_args!("u16 [{0} + {1}] = {2}", base, offset, src))
                } else {
                    self.write_fmt(format_args!("u16 [{0}] = {1}", base, src))
                }
            } else {
                self.write_fmt(format_args!("u16 [0x{0:x}] = {1}", offset, src))
            }
        }
        fn store_u32(&mut self, src: Reg, base: Reg, offset: u32) -> Self::ReturnTy {
            if base != Reg::Zero {
                if offset != 0 {
                    self.write_fmt(format_args!("u32 [{0} + {1}] = {2}", base, offset, src))
                } else {
                    self.write_fmt(format_args!("u32 [{0}] = {1}", base, src))
                }
            } else {
                self.write_fmt(format_args!("u32 [0x{0:x}] = {1}", offset, src))
            }
        }
        fn load_u8(&mut self, dst: Reg, base: Reg, offset: u32) -> Self::ReturnTy {
            if base != Reg::Zero {
                if offset != 0 {
                    self.write_fmt(format_args!("{0} = u8 [{1} + {2}]", dst, base, offset))
                } else {
                    self.write_fmt(format_args!("{0} = u8 [{1}]", dst, base))
                }
            } else {
                self.write_fmt(format_args!("{0} = u8 [0x{1:x}]", dst, offset))
            }
        }
        fn load_i8(&mut self, dst: Reg, base: Reg, offset: u32) -> Self::ReturnTy {
            if base != Reg::Zero {
                if offset != 0 {
                    self.write_fmt(format_args!("{0} = i8 [{1} + {2}]", dst, base, offset))
                } else {
                    self.write_fmt(format_args!("{0} = i8 [{1}]", dst, base))
                }
            } else {
                self.write_fmt(format_args!("{0} = i8 [0x{1:x}]", dst, offset))
            }
        }
        fn load_u16(&mut self, dst: Reg, base: Reg, offset: u32) -> Self::ReturnTy {
            if base != Reg::Zero {
                if offset != 0 {
                    self.write_fmt(format_args!("{0} = u16 [{1} + {2}]", dst, base, offset))
                } else {
                    self.write_fmt(format_args!("{0} = u16 [{1} ]", dst, base))
                }
            } else {
                self.write_fmt(format_args!("{0} = u16 [0x{1:x}]", dst, offset))
            }
        }
        fn load_i16(&mut self, dst: Reg, base: Reg, offset: u32) -> Self::ReturnTy {
            if base != Reg::Zero {
                if offset != 0 {
                    self.write_fmt(format_args!("{0} = i16 [{1} + {2}]", dst, base, offset))
                } else {
                    self.write_fmt(format_args!("{0} = i16 [{1}]", dst, base))
                }
            } else {
                self.write_fmt(format_args!("{0} = i16 [0x{1:x}]", dst, offset))
            }
        }
        fn load_u32(&mut self, dst: Reg, base: Reg, offset: u32) -> Self::ReturnTy {
            if base != Reg::Zero {
                if offset != 0 {
                    self.write_fmt(format_args!("{0} = u32 [{1} + {2}]", dst, base, offset))
                } else {
                    self.write_fmt(format_args!("{0} = u32 [{1}]", dst, base))
                }
            } else {
                self.write_fmt(format_args!("{0} = u32 [0x{1:x}]", dst, offset))
            }
        }
        fn branch_less_unsigned(&mut self, s1: Reg, s2: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!(
                "if {0} <u {1} -> jump @{2:x}",
                s1,
                s2,
                imm * 4
            ))
        }
        fn branch_less_signed(&mut self, s1: Reg, s2: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!(
                "if {0} <s {1} -> jump @{2:x}",
                s1,
                s2,
                imm * 4
            ))
        }
        fn branch_greater_or_equal_unsigned(
            &mut self,
            s1: Reg,
            s2: Reg,
            imm: u32,
        ) -> Self::ReturnTy {
            self.write_fmt(format_args!(
                "if {0} >=u {1} -> jump @{2:x}",
                s1,
                s2,
                imm * 4
            ))
        }
        fn branch_greater_or_equal_signed(&mut self, s1: Reg, s2: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!(
                "if {0} >=s {1} -> jump @{2:x}",
                s1,
                s2,
                imm * 4
            ))
        }
        fn branch_eq(&mut self, s1: Reg, s2: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!(
                "if {0} == {1} -> jump @{2:x}",
                s1,
                s2,
                imm * 4
            ))
        }
        fn branch_not_eq(&mut self, s1: Reg, s2: Reg, imm: u32) -> Self::ReturnTy {
            self.write_fmt(format_args!(
                "if {0} != {1} -> jump @{2:x}",
                s1,
                s2,
                imm * 4
            ))
        }
        fn jump_and_link_register(&mut self, ra: Reg, base: Reg, offset: u32) -> Self::ReturnTy {
            use Reg::*;
            match (ra, base, offset) {
                (Zero, RA, 0) => self.write_fmt(format_args!("ret")),
                (Zero, Zero, _) => self.write_fmt(format_args!("jump @{0:x}", offset * 4)),
                (Zero, _, 0) => self.write_fmt(format_args!("jump [{0}]", base)),
                (Zero, _, _) => self.write_fmt(format_args!("jump [{0} + {1}]", base, offset * 4)),
                (RA, Zero, _) => self.write_fmt(format_args!("call @{0:x}", offset * 4)),
                (RA, _, 0) => self.write_fmt(format_args!("call [{0}]", base)),
                (RA, _, _) => self.write_fmt(format_args!("call [{0} + {1}]", base, offset * 4)),
                (_, Zero, _) => self.write_fmt(format_args!("call @{0:x}, {1}", offset * 4, ra)),
                (_, _, 0) => self.write_fmt(format_args!("call [{0}], {1}", base, ra)),
                (_, _, _) => {
                    self.write_fmt(format_args!("call [{0} + {1}], {2}", base, offset * 4, ra))
                }
            }
        }
    }
    impl RawInstruction {
        #[inline]
        pub fn new_argless(op: Opcode) -> Self {
            match (&(op as u8 & 0b11_000000), &0b00_000000) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            RawInstruction {
                op: op as u8,
                regs: 0,
                imm_or_reg: 0,
            }
        }
        #[inline]
        pub fn new_with_imm(op: Opcode, imm: u32) -> Self {
            match (&(op as u8 & 0b11_000000), &0b01_000000) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            RawInstruction {
                op: op as u8,
                regs: 0,
                imm_or_reg: imm,
            }
        }
        #[inline]
        pub fn new_with_regs3(op: Opcode, reg1: Reg, reg2: Reg, reg3: Reg) -> Self {
            match (&(op as u8 & 0b11_000000), &0b10_000000) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            RawInstruction {
                op: op as u8,
                regs: reg1 as u8 | (reg2 as u8) << 4,
                imm_or_reg: reg3 as u32,
            }
        }
        #[inline]
        pub fn new_with_regs2_imm(op: Opcode, reg1: Reg, reg2: Reg, imm: u32) -> Self {
            match (&(op as u8 & 0b11_000000), &0b11_000000) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            RawInstruction {
                op: op as u8,
                regs: reg1 as u8 | (reg2 as u8) << 4,
                imm_or_reg: imm,
            }
        }
        #[inline]
        pub fn op(self) -> Opcode {
            if let Some(op) = Opcode::from_u8(self.op) {
                op
            } else {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        #[inline]
        fn reg1(self) -> Reg {
            Reg::from_u8(self.regs & 0b00001111).unwrap_or_else(|| {
                ::core::panicking::panic("internal error: entered unreachable code")
            })
        }
        #[inline]
        fn reg2(self) -> Reg {
            Reg::from_u8(self.regs >> 4).unwrap_or_else(|| {
                ::core::panicking::panic("internal error: entered unreachable code")
            })
        }
        #[inline]
        fn reg3(self) -> Reg {
            Reg::from_u8(self.imm_or_reg as u8).unwrap_or_else(|| {
                ::core::panicking::panic("internal error: entered unreachable code")
            })
        }
        #[inline]
        pub fn raw_op(self) -> u8 {
            self.op
        }
        #[inline]
        pub fn raw_imm_or_reg(self) -> u32 {
            self.imm_or_reg
        }
        pub fn deserialize(input: &[u8]) -> Option<(usize, Self)> {
            let op = *input.get(0)?;
            if !IS_INSTRUCTION_VALID[op as usize] {
                return None;
            }
            let mut position = 1;
            let mut output = RawInstruction {
                op,
                regs: 0,
                imm_or_reg: 0,
            };
            if op & 0b10000000 != 0 {
                output.regs = *input.get(position)?;
                if (match output.regs & 0b1111 {
                    14 | 15 => true,
                    _ => false,
                }) || match output.regs >> 4 {
                    14 | 15 => true,
                    _ => false,
                } {
                    return None;
                }
                position += 1;
            }
            if op & 0b11000000 != 0 {
                let first_byte = *input.get(position)?;
                position += 1;
                if op & 0b11_000000 == 0b10_000000 {
                    if first_byte > 13 {
                        return None;
                    }
                    output.imm_or_reg = first_byte as u32;
                } else {
                    let (length, imm_or_reg) = read_varint(&input[position..], first_byte)?;
                    position += length;
                    output.imm_or_reg = imm_or_reg;
                }
            }
            Some((position, output))
        }
        #[inline]
        pub fn serialize_into(self, buffer: &mut [u8]) -> usize {
            if !(buffer.len() >= MAX_INSTRUCTION_LENGTH) {
                ::core::panicking::panic("assertion failed: buffer.len() >= MAX_INSTRUCTION_LENGTH")
            };
            buffer[0] = self.op;
            let mut length = 1;
            if self.op & 0b10000000 != 0 {
                buffer[1] = self.regs;
                length += 1;
            }
            if self.op & 0b11000000 != 0 {
                length += write_varint(self.imm_or_reg, &mut buffer[length..]);
            }
            length
        }
    }
    pub struct ProgramParseError(ProgramParseErrorKind);
    #[automatically_derived]
    impl ::core::fmt::Debug for ProgramParseError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ProgramParseError", &&self.0)
        }
    }
    enum ProgramParseErrorKind {
        FailedToReadVarint {
            offset: usize,
        },
        FailedToReadStringNonUtf {
            offset: usize,
        },
        UnexpectedSection {
            offset: usize,
            section: u8,
        },
        UnexpectedInstruction {
            offset: usize,
        },
        UnexpectedEnd {
            offset: usize,
            expected_count: usize,
            actual_count: usize,
        },
        UnsupportedVersion {
            version: u8,
        },
        Other(&'static str),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ProgramParseErrorKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ProgramParseErrorKind::FailedToReadVarint { offset: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "FailedToReadVarint",
                        "offset",
                        &__self_0,
                    )
                }
                ProgramParseErrorKind::FailedToReadStringNonUtf { offset: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "FailedToReadStringNonUtf",
                        "offset",
                        &__self_0,
                    )
                }
                ProgramParseErrorKind::UnexpectedSection {
                    offset: __self_0,
                    section: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "UnexpectedSection",
                    "offset",
                    __self_0,
                    "section",
                    &__self_1,
                ),
                ProgramParseErrorKind::UnexpectedInstruction { offset: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UnexpectedInstruction",
                        "offset",
                        &__self_0,
                    )
                }
                ProgramParseErrorKind::UnexpectedEnd {
                    offset: __self_0,
                    expected_count: __self_1,
                    actual_count: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "UnexpectedEnd",
                    "offset",
                    __self_0,
                    "expected_count",
                    __self_1,
                    "actual_count",
                    &__self_2,
                ),
                ProgramParseErrorKind::UnsupportedVersion { version: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UnsupportedVersion",
                        "version",
                        &__self_0,
                    )
                }
                ProgramParseErrorKind::Other(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Other", &__self_0)
                }
            }
        }
    }
    impl core::fmt::Display for ProgramParseError {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self . 0 { ProgramParseErrorKind :: FailedToReadVarint { offset } => { fmt . write_fmt (format_args ! ("failed to parse program blob: failed to parse a varint at offset 0x{0:x}" , offset)) } ProgramParseErrorKind :: FailedToReadStringNonUtf { offset } => { fmt . write_fmt (format_args ! ("failed to parse program blob: failed to parse a string at offset 0x{0:x} (not valid UTF-8)" , offset)) } ProgramParseErrorKind :: UnexpectedSection { offset , section } => { fmt . write_fmt (format_args ! ("failed to parse program blob: found unexpected section as offset 0x{0:x}: 0x{1:x}" , offset , section)) } ProgramParseErrorKind :: UnexpectedInstruction { offset } => { fmt . write_fmt (format_args ! ("failed to parse program blob: failed to parse instruction at offset 0x{0:x}" , offset)) } ProgramParseErrorKind :: UnexpectedEnd { offset , expected_count , actual_count } => { fmt . write_fmt (format_args ! ("failed to parse program blob: unexpected end of file at offset 0x{0:x}: expected to be able to read at least {1} bytes, found {2} bytes" , offset , expected_count , actual_count)) } ProgramParseErrorKind :: UnsupportedVersion { version } => { fmt . write_fmt (format_args ! ("failed to parse program blob: unsupported version: {0}" , version)) } ProgramParseErrorKind :: Other (error) => { fmt . write_fmt (format_args ! ("failed to parse program blob: {0}" , error)) } }
        }
    }
    pub struct ProgramExport<'a> {
        address: u32,
        prototype: ExternFnPrototype<'a>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for ProgramExport<'a> {
        #[inline]
        fn clone(&self) -> ProgramExport<'a> {
            ProgramExport {
                address: ::core::clone::Clone::clone(&self.address),
                prototype: ::core::clone::Clone::clone(&self.prototype),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for ProgramExport<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for ProgramExport<'a> {
        #[inline]
        fn eq(&self, other: &ProgramExport<'a>) -> bool {
            self.address == other.address && self.prototype == other.prototype
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralEq for ProgramExport<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::Eq for ProgramExport<'a> {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
            let _: ::core::cmp::AssertParamIsEq<ExternFnPrototype<'a>>;
        }
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for ProgramExport<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ProgramExport",
                "address",
                &self.address,
                "prototype",
                &&self.prototype,
            )
        }
    }
    impl<'a> ProgramExport<'a> {
        pub fn address(&self) -> u32 {
            self.address
        }
        pub fn prototype(&self) -> &ExternFnPrototype<'a> {
            &self.prototype
        }
    }
    pub struct ProgramImport<'a> {
        index: u32,
        prototype: ExternFnPrototype<'a>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for ProgramImport<'a> {
        #[inline]
        fn clone(&self) -> ProgramImport<'a> {
            ProgramImport {
                index: ::core::clone::Clone::clone(&self.index),
                prototype: ::core::clone::Clone::clone(&self.prototype),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for ProgramImport<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for ProgramImport<'a> {
        #[inline]
        fn eq(&self, other: &ProgramImport<'a>) -> bool {
            self.index == other.index && self.prototype == other.prototype
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralEq for ProgramImport<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::Eq for ProgramImport<'a> {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
            let _: ::core::cmp::AssertParamIsEq<ExternFnPrototype<'a>>;
        }
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for ProgramImport<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ProgramImport",
                "index",
                &self.index,
                "prototype",
                &&self.prototype,
            )
        }
    }
    impl<'a> ProgramImport<'a> {
        pub fn index(&self) -> u32 {
            self.index
        }
        pub fn prototype(&self) -> &ExternFnPrototype<'a> {
            &self.prototype
        }
    }
    pub struct ExternFnPrototype<'a> {
        name: CowString<'a>,
        arg_count: u32,
        args: [Option<ExternTy>; crate::abi::VM_MAXIMUM_EXTERN_ARG_COUNT],
        return_ty: Option<ExternTy>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for ExternFnPrototype<'a> {
        #[inline]
        fn clone(&self) -> ExternFnPrototype<'a> {
            ExternFnPrototype {
                name: ::core::clone::Clone::clone(&self.name),
                arg_count: ::core::clone::Clone::clone(&self.arg_count),
                args: ::core::clone::Clone::clone(&self.args),
                return_ty: ::core::clone::Clone::clone(&self.return_ty),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for ExternFnPrototype<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for ExternFnPrototype<'a> {
        #[inline]
        fn eq(&self, other: &ExternFnPrototype<'a>) -> bool {
            self.name == other.name
                && self.arg_count == other.arg_count
                && self.args == other.args
                && self.return_ty == other.return_ty
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralEq for ExternFnPrototype<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::Eq for ExternFnPrototype<'a> {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CowString<'a>>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
            let _: ::core::cmp::AssertParamIsEq<
                [Option<ExternTy>; crate::abi::VM_MAXIMUM_EXTERN_ARG_COUNT],
            >;
            let _: ::core::cmp::AssertParamIsEq<Option<ExternTy>>;
        }
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for ExternFnPrototype<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "ExternFnPrototype",
                "name",
                &self.name,
                "arg_count",
                &self.arg_count,
                "args",
                &self.args,
                "return_ty",
                &&self.return_ty,
            )
        }
    }
    impl<'a> ExternFnPrototype<'a> {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn args(&'_ self) -> impl ExactSizeIterator<Item = ExternTy> + Clone + '_ {
            struct ArgIter<'r> {
                position: usize,
                length: usize,
                args: &'r [Option<ExternTy>; crate::abi::VM_MAXIMUM_EXTERN_ARG_COUNT],
            }
            #[automatically_derived]
            impl<'r> ::core::clone::Clone for ArgIter<'r> {
                #[inline]
                fn clone(&self) -> ArgIter<'r> {
                    ArgIter {
                        position: ::core::clone::Clone::clone(&self.position),
                        length: ::core::clone::Clone::clone(&self.length),
                        args: ::core::clone::Clone::clone(&self.args),
                    }
                }
            }
            impl<'r> Iterator for ArgIter<'r> {
                type Item = ExternTy;
                fn next(&mut self) -> Option<Self::Item> {
                    if self.position >= self.length {
                        None
                    } else {
                        let ty = self.args[self.position].unwrap();
                        self.position += 1;
                        Some(ty)
                    }
                }
                fn size_hint(&self) -> (usize, Option<usize>) {
                    let remaining = self.length - self.position;
                    (remaining, Some(remaining))
                }
            }
            impl<'r> ExactSizeIterator for ArgIter<'r> {}
            ArgIter {
                position: 0,
                length: self.arg_count as usize,
                args: &self.args,
            }
        }
        pub fn return_ty(&self) -> Option<ExternTy> {
            self.return_ty
        }
    }
    /// A partially deserialized PolkaVM program.
    pub struct ProgramBlob<'a> {
        blob: CowBytes<'a>,
        bss_size: u32,
        stack_size: u32,
        ro_data: Range<usize>,
        rw_data: Range<usize>,
        exports: Range<usize>,
        imports: Range<usize>,
        code: Range<usize>,
        debug_strings: Range<usize>,
        debug_function_ranges: Range<usize>,
        debug_function_info: Range<usize>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for ProgramBlob<'a> {
        #[inline]
        fn clone(&self) -> ProgramBlob<'a> {
            ProgramBlob {
                blob: ::core::clone::Clone::clone(&self.blob),
                bss_size: ::core::clone::Clone::clone(&self.bss_size),
                stack_size: ::core::clone::Clone::clone(&self.stack_size),
                ro_data: ::core::clone::Clone::clone(&self.ro_data),
                rw_data: ::core::clone::Clone::clone(&self.rw_data),
                exports: ::core::clone::Clone::clone(&self.exports),
                imports: ::core::clone::Clone::clone(&self.imports),
                code: ::core::clone::Clone::clone(&self.code),
                debug_strings: ::core::clone::Clone::clone(&self.debug_strings),
                debug_function_ranges: ::core::clone::Clone::clone(&self.debug_function_ranges),
                debug_function_info: ::core::clone::Clone::clone(&self.debug_function_info),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::default::Default for ProgramBlob<'a> {
        #[inline]
        fn default() -> ProgramBlob<'a> {
            ProgramBlob {
                blob: ::core::default::Default::default(),
                bss_size: ::core::default::Default::default(),
                stack_size: ::core::default::Default::default(),
                ro_data: ::core::default::Default::default(),
                rw_data: ::core::default::Default::default(),
                exports: ::core::default::Default::default(),
                imports: ::core::default::Default::default(),
                code: ::core::default::Default::default(),
                debug_strings: ::core::default::Default::default(),
                debug_function_ranges: ::core::default::Default::default(),
                debug_function_info: ::core::default::Default::default(),
            }
        }
    }
    struct Reader<'a> {
        blob: &'a [u8],
        position: usize,
        previous_position: usize,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for Reader<'a> {
        #[inline]
        fn clone(&self) -> Reader<'a> {
            Reader {
                blob: ::core::clone::Clone::clone(&self.blob),
                position: ::core::clone::Clone::clone(&self.position),
                previous_position: ::core::clone::Clone::clone(&self.previous_position),
            }
        }
    }
    impl<'a> Reader<'a> {
        fn skip(&mut self, count: u32) -> Result<(), ProgramParseError> {
            self.read_slice_as_range(count).map(|_| ())
        }
        fn read_byte(&mut self) -> Result<u8, ProgramParseError> {
            Ok(self.blob[self.read_slice_as_range(1)?][0])
        }
        fn read_varint(&mut self) -> Result<u32, ProgramParseError> {
            let first_byte = self.read_byte()?;
            let (length, value) = read_varint(&self.blob[self.position..], first_byte).ok_or(
                ProgramParseError(ProgramParseErrorKind::FailedToReadVarint {
                    offset: self.previous_position,
                }),
            )?;
            self.position += length;
            Ok(value)
        }
        fn read_string_with_length(&mut self) -> Result<&'a str, ProgramParseError> {
            let length = self.read_varint()?;
            let range = self.read_slice_as_range(length)?;
            let slice = &self.blob[range];
            core::str::from_utf8(slice).ok().ok_or(ProgramParseError(
                ProgramParseErrorKind::FailedToReadStringNonUtf {
                    offset: self.previous_position,
                },
            ))
        }
        fn read_slice_as_range(&mut self, count: u32) -> Result<Range<usize>, ProgramParseError> {
            let range = self.position..self.position + count as usize;
            if self.blob.get(range.clone()).is_none() {
                return Err(ProgramParseError(ProgramParseErrorKind::UnexpectedEnd {
                    offset: self.position,
                    expected_count: count as usize,
                    actual_count: self.blob.len() - self.position,
                }));
            };
            self.previous_position = core::mem::replace(&mut self.position, range.end);
            Ok(range)
        }
        fn is_eof(&self) -> bool {
            self.position >= self.blob.len()
        }
        fn read_section_range_into(
            &mut self,
            out_section: &mut u8,
            out_range: &mut Range<usize>,
            expected_section: u8,
        ) -> Result<(), ProgramParseError> {
            if *out_section == expected_section {
                let section_length = self.read_varint()?;
                *out_range = self.read_slice_as_range(section_length)?;
                *out_section = self.read_byte()?;
            }
            Ok(())
        }
        fn read_extern_fn_prototype(&mut self) -> Result<ExternFnPrototype<'a>, ProgramParseError> {
            let name = self.read_string_with_length()?;
            let arg_count = self.read_varint()?;
            if arg_count > crate::abi::VM_MAXIMUM_EXTERN_ARG_COUNT as u32 {
                return Err (ProgramParseError (ProgramParseErrorKind :: Other ("found a function prototype which accepts more than the maximum allowed number of arguments"))) ;
            }
            let mut args: [Option<ExternTy>; crate::abi::VM_MAXIMUM_EXTERN_ARG_COUNT] =
                [None; crate::abi::VM_MAXIMUM_EXTERN_ARG_COUNT];
            for nth_arg in 0..arg_count {
                let ty = ExternTy::try_deserialize(self.read_byte()?).ok_or(ProgramParseError(
                    ProgramParseErrorKind::Other(
                        "found a function prototype with an unrecognized argument type",
                    ),
                ))?;
                args[nth_arg as usize] = Some(ty);
            }
            let return_ty = match self.read_byte()? {
                0 => None,
                return_ty => {
                    let ty = ExternTy::try_deserialize(return_ty).ok_or(ProgramParseError(
                        ProgramParseErrorKind::Other(
                            "found a function prototype with an unrecognized return type",
                        ),
                    ))?;
                    Some(ty)
                }
            };
            Ok(ExternFnPrototype {
                name: name.into(),
                arg_count,
                args,
                return_ty,
            })
        }
    }
    impl<'a> ProgramBlob<'a> {
        /// Parses the given bytes into a program blob.
        pub fn parse(bytes: impl Into<CowBytes<'a>>) -> Result<Self, ProgramParseError> {
            Self::parse_impl(bytes.into())
        }
        /// Returns the original bytes from which this program blob was created from.
        pub fn as_bytes(&self) -> &[u8] {
            &self.blob
        }
        #[inline(never)]
        fn parse_impl(blob: CowBytes<'a>) -> Result<Self, ProgramParseError> {
            if !blob.starts_with(&BLOB_MAGIC) {
                return Err(ProgramParseError(ProgramParseErrorKind::Other(
                    "blob doesn't start with the expected magic bytes",
                )));
            }
            let mut program = ProgramBlob {
                blob,
                ..ProgramBlob::default()
            };
            let mut reader = Reader {
                blob: &program.blob,
                position: BLOB_MAGIC.len(),
                previous_position: 0,
            };
            let blob_version = reader.read_byte()?;
            if blob_version != BLOB_VERSION_V1 {
                return Err(ProgramParseError(
                    ProgramParseErrorKind::UnsupportedVersion {
                        version: blob_version,
                    },
                ));
            }
            let mut section = reader.read_byte()?;
            if section == SECTION_MEMORY_CONFIG {
                let section_length = reader.read_varint()?;
                let position = reader.position;
                program.bss_size = reader.read_varint()?;
                program.stack_size = reader.read_varint()?;
                if position + section_length as usize != reader.position {
                    return Err(ProgramParseError(ProgramParseErrorKind::Other(
                        "the memory config section contains more data than expected",
                    )));
                }
                section = reader.read_byte()?;
            }
            reader.read_section_range_into(&mut section, &mut program.ro_data, SECTION_RO_DATA)?;
            reader.read_section_range_into(&mut section, &mut program.rw_data, SECTION_RW_DATA)?;
            reader.read_section_range_into(&mut section, &mut program.imports, SECTION_IMPORTS)?;
            reader.read_section_range_into(&mut section, &mut program.exports, SECTION_EXPORTS)?;
            reader.read_section_range_into(&mut section, &mut program.code, SECTION_CODE)?;
            reader.read_section_range_into(
                &mut section,
                &mut program.debug_strings,
                SECTION_OPT_DEBUG_STRINGS,
            )?;
            reader.read_section_range_into(
                &mut section,
                &mut program.debug_function_info,
                SECTION_OPT_DEBUG_FUNCTION_INFO,
            )?;
            reader.read_section_range_into(
                &mut section,
                &mut program.debug_function_ranges,
                SECTION_OPT_DEBUG_FUNCTION_RANGES,
            )?;
            while (section & 0b10000000) != 0 {
                let section_length = reader.read_varint()?;
                reader.skip(section_length)?;
                section = reader.read_byte()?;
            }
            if section == SECTION_END_OF_FILE {
                return Ok(program);
            }
            Err(ProgramParseError(
                ProgramParseErrorKind::UnexpectedSection {
                    offset: reader.previous_position,
                    section,
                },
            ))
        }
        /// Returns the contents of the read-only data section.
        pub fn ro_data(&self) -> &[u8] {
            &self.blob[self.ro_data.clone()]
        }
        /// Returns the contents of the read-write data section.
        pub fn rw_data(&self) -> &[u8] {
            &self.blob[self.rw_data.clone()]
        }
        /// Returns the initial size of the BSS section.
        pub fn bss_size(&self) -> u32 {
            self.bss_size
        }
        /// Returns the initial size of the stack.
        pub fn stack_size(&self) -> u32 {
            self.stack_size
        }
        /// Returns the program code in its raw form.
        pub fn code(&self) -> &[u8] {
            &self.blob[self.code.clone()]
        }
        fn get_section_reader(&self, range: Range<usize>) -> Reader {
            Reader {
                blob: &self.blob[..range.end],
                position: range.start,
                previous_position: 0,
            }
        }
        /// Returns an iterator over program imports.
        pub fn imports(
            &'_ self,
        ) -> impl Iterator<Item = Result<ProgramImport, ProgramParseError>> + Clone + '_ {
            enum State {
                Uninitialized,
                Pending(u32),
                Finished,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for State {
                #[inline]
                fn clone(&self) -> State {
                    match self {
                        State::Uninitialized => State::Uninitialized,
                        State::Pending(__self_0) => {
                            State::Pending(::core::clone::Clone::clone(__self_0))
                        }
                        State::Finished => State::Finished,
                    }
                }
            }
            struct ImportIterator<'a> {
                state: State,
                reader: Reader<'a>,
            }
            #[automatically_derived]
            impl<'a> ::core::clone::Clone for ImportIterator<'a> {
                #[inline]
                fn clone(&self) -> ImportIterator<'a> {
                    ImportIterator {
                        state: ::core::clone::Clone::clone(&self.state),
                        reader: ::core::clone::Clone::clone(&self.reader),
                    }
                }
            }
            impl<'a> ImportIterator<'a> {
                fn read_next(&mut self) -> Result<Option<ProgramImport<'a>>, ProgramParseError> {
                    let remaining = match core::mem::replace(&mut self.state, State::Finished) {
                        State::Uninitialized => self.reader.read_varint()?,
                        State::Pending(remaining) => remaining,
                        State::Finished => return Ok(None),
                    };
                    if remaining == 0 {
                        if !self.reader.is_eof() {
                            return Err(ProgramParseError(ProgramParseErrorKind::Other(
                                "the import section contains more data than expected",
                            )));
                        }
                        return Ok(None);
                    }
                    let index = self.reader.read_varint()?;
                    let prototype = self.reader.read_extern_fn_prototype()?;
                    let import = ProgramImport { index, prototype };
                    self.state = State::Pending(remaining - 1);
                    Ok(Some(import))
                }
            }
            impl<'a> Iterator for ImportIterator<'a> {
                type Item = Result<ProgramImport<'a>, ProgramParseError>;
                fn next(&mut self) -> Option<Self::Item> {
                    self.read_next().transpose()
                }
            }
            ImportIterator {
                state: if self.imports != (0_usize..0_usize) {
                    State::Uninitialized
                } else {
                    State::Finished
                },
                reader: self.get_section_reader(self.imports.clone()),
            }
        }
        /// Returns an iterator over program exports.
        pub fn exports(
            &'_ self,
        ) -> impl Iterator<Item = Result<ProgramExport, ProgramParseError>> + Clone + '_ {
            enum State {
                Uninitialized,
                Pending(u32),
                Finished,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for State {
                #[inline]
                fn clone(&self) -> State {
                    match self {
                        State::Uninitialized => State::Uninitialized,
                        State::Pending(__self_0) => {
                            State::Pending(::core::clone::Clone::clone(__self_0))
                        }
                        State::Finished => State::Finished,
                    }
                }
            }
            struct ExportIterator<'a> {
                state: State,
                reader: Reader<'a>,
            }
            #[automatically_derived]
            impl<'a> ::core::clone::Clone for ExportIterator<'a> {
                #[inline]
                fn clone(&self) -> ExportIterator<'a> {
                    ExportIterator {
                        state: ::core::clone::Clone::clone(&self.state),
                        reader: ::core::clone::Clone::clone(&self.reader),
                    }
                }
            }
            impl<'a> ExportIterator<'a> {
                fn read_next(&mut self) -> Result<Option<ProgramExport<'a>>, ProgramParseError> {
                    let remaining = match core::mem::replace(&mut self.state, State::Finished) {
                        State::Uninitialized => self.reader.read_varint()?,
                        State::Pending(remaining) => remaining,
                        State::Finished => return Ok(None),
                    };
                    if remaining == 0 {
                        if !self.reader.is_eof() {
                            return Err(ProgramParseError(ProgramParseErrorKind::Other(
                                "the export section contains more data than expected",
                            )));
                        }
                        return Ok(None);
                    }
                    let address = self.reader.read_varint()?;
                    let prototype = self.reader.read_extern_fn_prototype()?;
                    let export = ProgramExport { address, prototype };
                    self.state = State::Pending(remaining - 1);
                    Ok(Some(export))
                }
            }
            impl<'a> Iterator for ExportIterator<'a> {
                type Item = Result<ProgramExport<'a>, ProgramParseError>;
                fn next(&mut self) -> Option<Self::Item> {
                    self.read_next().transpose()
                }
            }
            ExportIterator {
                state: if self.exports != (0_usize..0_usize) {
                    State::Uninitialized
                } else {
                    State::Finished
                },
                reader: self.get_section_reader(self.exports.clone()),
            }
        }
        /// Returns an iterator over program instructions.
        pub fn instructions(
            &'_ self,
        ) -> impl Iterator<Item = Result<RawInstruction, ProgramParseError>> + Clone + '_ {
            struct CodeIterator<'a> {
                code_section_position: usize,
                position: usize,
                code: &'a [u8],
            }
            #[automatically_derived]
            impl<'a> ::core::clone::Clone for CodeIterator<'a> {
                #[inline]
                fn clone(&self) -> CodeIterator<'a> {
                    CodeIterator {
                        code_section_position: ::core::clone::Clone::clone(
                            &self.code_section_position,
                        ),
                        position: ::core::clone::Clone::clone(&self.position),
                        code: ::core::clone::Clone::clone(&self.code),
                    }
                }
            }
            impl<'a> Iterator for CodeIterator<'a> {
                type Item = Result<RawInstruction, ProgramParseError>;
                fn next(&mut self) -> Option<Self::Item> {
                    let slice = &self.code[self.position..];
                    if slice.is_empty() {
                        return None;
                    }
                    if let Some((bytes_consumed, instruction)) = RawInstruction::deserialize(slice)
                    {
                        self.position += bytes_consumed;
                        return Some(Ok(instruction));
                    }
                    let offset = self.code_section_position + self.position;
                    self.position = self.code.len();
                    Some(Err(ProgramParseError(
                        ProgramParseErrorKind::UnexpectedInstruction { offset },
                    )))
                }
            }
            CodeIterator {
                code_section_position: self.code.start,
                position: 0,
                code: self.code(),
            }
        }
        fn get_debug_string(&self, offset: u32) -> Result<&str, ProgramParseError> {
            let mut reader = self.get_section_reader(self.debug_strings.clone());
            reader.skip(offset)?;
            reader.read_string_with_length()
        }
        /// Returns the debug info for the function corresponding to the given instruction.
        pub fn get_function_debug_info(
            &self,
            nth_instruction: u32,
        ) -> Result<Option<FunctionInfo>, ProgramParseError> {
            if self.debug_function_ranges.is_empty() || self.debug_function_info.is_empty() {
                return Ok(None);
            }
            if self.blob[self.debug_function_info.start] != VERSION_DEBUG_FUNCTION_INFO_V1 {
                return Err(ProgramParseError(ProgramParseErrorKind::Other(
                    "the debug function info section has an unsupported version",
                )));
            }
            const ENTRY_SIZE: usize = 12;
            let slice = &self.blob[self.debug_function_ranges.clone()];
            if slice.len() % ENTRY_SIZE != 0 {
                return Err(ProgramParseError(ProgramParseErrorKind::Other(
                    "the debug function ranges section has an invalid size",
                )));
            }
            let offset = binary_search(slice, ENTRY_SIZE, |xs| {
                let begin = u32::from_le_bytes([xs[0], xs[1], xs[2], xs[3]]);
                if nth_instruction < begin {
                    return core::cmp::Ordering::Greater;
                }
                let end = u32::from_le_bytes([xs[4], xs[5], xs[6], xs[7]]);
                if nth_instruction >= end {
                    return core::cmp::Ordering::Less;
                }
                core::cmp::Ordering::Equal
            });
            let Ok(offset) = offset else { return Ok(None) };
            let xs = &slice[offset..offset + ENTRY_SIZE];
            let index_begin = u32::from_le_bytes([xs[0], xs[1], xs[2], xs[3]]);
            let index_end = u32::from_le_bytes([xs[4], xs[5], xs[6], xs[7]]);
            let info_offset = u32::from_le_bytes([xs[8], xs[9], xs[10], xs[11]]);
            if nth_instruction < index_begin || nth_instruction >= index_end {
                return Err(ProgramParseError(ProgramParseErrorKind::Other(
                    "binary search for function debug info failed",
                )));
            }
            let mut reader = self.get_section_reader(self.debug_function_info.clone());
            reader.skip(info_offset)?;
            let common_info = FunctionInfoCommon::read(self, &mut reader)?;
            let inline_frame_count = reader.read_varint()?;
            Ok(Some(FunctionInfo {
                blob: self,
                entry_index: offset / ENTRY_SIZE,
                index_begin,
                index_end,
                common_info,
                inline_frame_count,
                inline_reader: reader,
            }))
        }
    }
    /// The source location.
    pub enum Location<'a> {
        Path {
            path: &'a str,
        },
        PathAndLine {
            path: &'a str,
            line: u32,
        },
        Full {
            path: &'a str,
            line: u32,
            column: u32,
        },
    }
    #[automatically_derived]
    impl<'a> ::core::marker::Copy for Location<'a> {}
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for Location<'a> {
        #[inline]
        fn clone(&self) -> Location<'a> {
            let _: ::core::clone::AssertParamIsClone<&'a str>;
            let _: ::core::clone::AssertParamIsClone<&'a str>;
            let _: ::core::clone::AssertParamIsClone<u32>;
            let _: ::core::clone::AssertParamIsClone<&'a str>;
            *self
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for Location<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for Location<'a> {
        #[inline]
        fn eq(&self, other: &Location<'a>) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (Location::Path { path: __self_0 }, Location::Path { path: __arg1_0 }) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        Location::PathAndLine {
                            path: __self_0,
                            line: __self_1,
                        },
                        Location::PathAndLine {
                            path: __arg1_0,
                            line: __arg1_1,
                        },
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                    (
                        Location::Full {
                            path: __self_0,
                            line: __self_1,
                            column: __self_2,
                        },
                        Location::Full {
                            path: __arg1_0,
                            line: __arg1_1,
                            column: __arg1_2,
                        },
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1 && *__self_2 == *__arg1_2,
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralEq for Location<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::Eq for Location<'a> {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<&'a str>;
            let _: ::core::cmp::AssertParamIsEq<&'a str>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
            let _: ::core::cmp::AssertParamIsEq<&'a str>;
        }
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Location<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Location::Path { path: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(f, "Path", "path", &__self_0)
                }
                Location::PathAndLine {
                    path: __self_0,
                    line: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "PathAndLine",
                    "path",
                    __self_0,
                    "line",
                    &__self_1,
                ),
                Location::Full {
                    path: __self_0,
                    line: __self_1,
                    column: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f, "Full", "path", __self_0, "line", __self_1, "column", &__self_2,
                ),
            }
        }
    }
    impl<'a> Location<'a> {
        /// The path to the original source file.
        pub fn path(&self) -> &'a str {
            match *self {
                Location::Path { path, .. } => path,
                Location::PathAndLine { path, .. } => path,
                Location::Full { path, .. } => path,
            }
        }
        /// The line in the original source file.
        pub fn line(&self) -> Option<u32> {
            match *self {
                Location::Path { .. } => None,
                Location::PathAndLine { line, .. } => Some(line),
                Location::Full { line, .. } => Some(line),
            }
        }
        /// The column in the original source file.
        pub fn column(&self) -> Option<u32> {
            match *self {
                Location::Path { .. } => None,
                Location::PathAndLine { .. } => None,
                Location::Full { column, .. } => Some(column),
            }
        }
    }
    impl<'a> core::fmt::Display for Location<'a> {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                Location::Path { path } => fmt.write_str(path),
                Location::PathAndLine { path, line } => {
                    fmt.write_fmt(format_args!("{0}:{1}", path, line))
                }
                Location::Full { path, line, column } => {
                    fmt.write_fmt(format_args!("{0}:{1}:{2}", path, line, column))
                }
            }
        }
    }
    struct FunctionInfoCommon<'a> {
        name_prefix: &'a str,
        name_suffix: &'a str,
        path: &'a str,
        line: u32,
        column: u32,
    }
    /// Function debug info.
    pub struct FunctionInfo<'a> {
        blob: &'a ProgramBlob<'a>,
        entry_index: usize,
        index_begin: u32,
        index_end: u32,
        common_info: FunctionInfoCommon<'a>,
        inline_frame_count: u32,
        inline_reader: Reader<'a>,
    }
    /// Inlined function debug info.
    pub struct InlineFunctionInfo<'a> {
        index_base: u32,
        rel_index_begin: u32,
        rel_index_end: u32,
        depth: u32,
        common_info: FunctionInfoCommon<'a>,
    }
    struct DisplayName<'a> {
        prefix: &'a str,
        suffix: &'a str,
    }
    impl<'a> core::fmt::Display for DisplayName<'a> {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.write_str(self.prefix)?;
            if !self.prefix.is_empty() {
                fmt.write_str("::")?;
            }
            fmt.write_str(self.suffix)
        }
    }
    impl<'a> FunctionInfoCommon<'a> {
        fn read(
            blob: &'a ProgramBlob<'a>,
            reader: &mut Reader<'a>,
        ) -> Result<Self, ProgramParseError> {
            let name_prefix_offset = reader.read_varint()?;
            let name_suffix_offset = reader.read_varint()?;
            let path_offset = reader.read_varint()?;
            let line = reader.read_varint()?;
            let column = reader.read_varint()?;
            let name_prefix = blob.get_debug_string(name_prefix_offset)?;
            let name_suffix = blob.get_debug_string(name_suffix_offset)?;
            let path = blob.get_debug_string(path_offset)?;
            Ok(Self {
                name_prefix,
                name_suffix,
                path,
                line,
                column,
            })
        }
        fn location(&self) -> Option<Location<'a>> {
            if !self.path.is_empty() {
                if self.line != 0 {
                    if self.column != 0 {
                        Some(Location::Full {
                            path: self.path,
                            line: self.line,
                            column: self.column,
                        })
                    } else {
                        Some(Location::PathAndLine {
                            path: self.path,
                            line: self.line,
                        })
                    }
                } else {
                    Some(Location::Path { path: self.path })
                }
            } else {
                None
            }
        }
    }
    impl<'a> FunctionInfo<'a> {
        /// Returns the entry index of this function info object.
        pub fn entry_index(&self) -> usize {
            self.entry_index
        }
        /// The range of instruction indexes this function covers.
        pub fn instruction_range(&self) -> Range<u32> {
            self.index_begin..self.index_end
        }
        /// The name of the function.
        pub fn full_name(&self) -> impl core::fmt::Display + 'a {
            DisplayName {
                prefix: self.common_info.name_prefix,
                suffix: self.common_info.name_suffix,
            }
        }
        /// Return the source location of where the function is defined.
        pub fn location(&self) -> Option<Location<'a>> {
            self.common_info.location()
        }
        /// Returns an iterator over frames which were inlined into this function.
        pub fn inlined(
            &self,
        ) -> impl Iterator<Item = Result<InlineFunctionInfo, ProgramParseError>> {
            struct InlineIter<'a> {
                index_base: u32,
                blob: &'a ProgramBlob<'a>,
                remaining: usize,
                reader: Reader<'a>,
            }
            impl<'a> InlineIter<'a> {
                fn read_next(
                    &mut self,
                ) -> Result<Option<InlineFunctionInfo<'a>>, ProgramParseError> {
                    if self.remaining == 0 {
                        return Ok(None);
                    }
                    let next_remaining = core::mem::replace(&mut self.remaining, 0) - 1;
                    let rel_index_begin = self.reader.read_varint()?;
                    let rel_index_end = self.reader.read_varint()?;
                    let depth = self.reader.read_varint()?;
                    let common_info = FunctionInfoCommon::read(self.blob, &mut self.reader)?;
                    let info = InlineFunctionInfo {
                        index_base: self.index_base,
                        rel_index_begin,
                        rel_index_end,
                        depth,
                        common_info,
                    };
                    self.remaining = next_remaining;
                    Ok(Some(info))
                }
            }
            impl<'a> Iterator for InlineIter<'a> {
                type Item = Result<InlineFunctionInfo<'a>, ProgramParseError>;
                fn next(&mut self) -> Option<Self::Item> {
                    self.read_next().transpose()
                }
            }
            InlineIter {
                index_base: self.index_begin,
                blob: self.blob,
                remaining: self.inline_frame_count as usize,
                reader: self.inline_reader.clone(),
            }
        }
    }
    impl<'a> InlineFunctionInfo<'a> {
        /// The range of instruction indexes this inline function covers.
        pub fn instruction_range(&self) -> Range<u32> {
            self.index_base + self.rel_index_begin..self.index_base + self.rel_index_end
        }
        /// The name of the function.
        pub fn full_name(&self) -> impl core::fmt::Display + 'a {
            DisplayName {
                prefix: self.common_info.name_prefix,
                suffix: self.common_info.name_suffix,
            }
        }
        /// Returns the source location of the inline frame.
        pub fn location(&self) -> Option<Location<'a>> {
            self.common_info.location()
        }
        /// Returns the depth of the inline frame.
        pub fn depth(&self) -> u32 {
            self.depth
        }
    }
    /// A binary search implementation which can work on chunks of items, and guarantees that it
    /// will always return the first item if there are multiple identical consecutive items.
    fn binary_search(
        slice: &[u8],
        chunk_size: usize,
        compare: impl Fn(&[u8]) -> core::cmp::Ordering,
    ) -> Result<usize, usize> {
        let mut size = slice.len() / chunk_size;
        if size == 0 {
            return Err(0);
        }
        let mut base = 0_usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let item = &slice[mid * chunk_size..(mid + 1) * chunk_size];
            match compare(item) {
                core::cmp::Ordering::Greater => {
                    size -= half;
                }
                core::cmp::Ordering::Less => {
                    size -= half;
                    base = mid;
                }
                core::cmp::Ordering::Equal => {
                    let previous_item = &slice[(mid - 1) * chunk_size..mid * chunk_size];
                    if compare(previous_item) != core::cmp::Ordering::Equal {
                        return Ok(mid * chunk_size);
                    }
                    size -= half;
                }
            }
        }
        let item = &slice[base * chunk_size..(base + 1) * chunk_size];
        let ord = compare(item);
        if ord == core::cmp::Ordering::Equal {
            Ok(base * chunk_size)
        } else {
            Err((base + (ord == core::cmp::Ordering::Less) as usize) * chunk_size)
        }
    }
    /// The magic bytes with which every program blob must start with.
    pub const BLOB_MAGIC: [u8; 4] = [b'P', b'V', b'M', b'\0'];
    pub const SECTION_MEMORY_CONFIG: u8 = 1;
    pub const SECTION_RO_DATA: u8 = 2;
    pub const SECTION_RW_DATA: u8 = 3;
    pub const SECTION_IMPORTS: u8 = 4;
    pub const SECTION_EXPORTS: u8 = 5;
    pub const SECTION_CODE: u8 = 6;
    pub const SECTION_OPT_DEBUG_STRINGS: u8 = 128;
    pub const SECTION_OPT_DEBUG_FUNCTION_INFO: u8 = 129;
    pub const SECTION_OPT_DEBUG_FUNCTION_RANGES: u8 = 130;
    pub const SECTION_END_OF_FILE: u8 = 0;
    pub const BLOB_VERSION_V1: u8 = 1;
    pub const VERSION_DEBUG_FUNCTION_INFO_V1: u8 = 1;
}
