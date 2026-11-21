use super::environment::Environment;
use super::machine::Machine;
use super::registers::Flag;

#[derive(Copy, Clone)]
pub enum Operator {
    Add,
    Adc,
    Sub,
    Sbc,
    And,
    Xor,
    Or,
    Cp,
}

impl Operator {
    pub fn apply<M: Machine>(self, env: &mut Environment<M>, a: u8, b: u8) -> u8 {
        match self {
            Self::Add => operator_add(env, a, b),
            Self::Adc => operator_adc(env, a, b),
            Self::Sub => operator_sub(env, a, b),
            Self::Sbc => operator_sbc(env, a, b),
            Self::And => operator_and(env, a, b),
            Self::Xor => operator_xor(env, a, b),
            Self::Or => operator_or(env, a, b),
            Self::Cp => operator_cp(env, a, b),
        }
    }
}

pub fn operator_add<M: Machine>(env: &mut Environment<M>, a: u8, b: u8) -> u8 {
    env.state.reg.clear_flag(Flag::C);
    operator_adc(env, a, b)
}

pub fn operator_adc<M: Machine>(env: &mut Environment<M>, a: u8, b: u8) -> u8 {
    let aa = a as u16;
    let bb = b as u16;
    let mut vv = aa + bb;
    if env.state.reg.get_flag(Flag::C) {
        vv += 1;
    }
    env.state
        .reg
        .update_arithmetic_flags(aa, bb, vv, false, true);
    vv as u8
}

pub fn operator_add16<M: Machine>(env: &mut Environment<M>, aa: u16, bb: u16) -> u16 {
    let aaaa = aa as u32;
    let bbbb = bb as u32;
    let vvvv = aaaa + bbbb;

    env.state.reg.update_add16_flags(aaaa, bbbb, vvvv);
    vvvv as u16
}

pub fn operator_adc16<M: Machine>(env: &mut Environment<M>, aa: u16, bb: u16) -> u16 {
    let aaaa = aa as u32;
    let bbbb = bb as u32;
    let mut vvvv = aaaa.wrapping_add(bbbb);
    if env.state.reg.get_flag(Flag::C) {
        vvvv = vvvv.wrapping_add(1);
    }
    let vv = vvvv as u16;

    // TUZD-8.6
    env.state
        .reg
        .update_arithmetic_flags_16(aaaa, bbbb, vvvv, false);
    env.state.reg.put_flag(Flag::Z, vv == 0);
    vv
}

pub fn operator_sbc16<M: Machine>(env: &mut Environment<M>, aa: u16, bb: u16) -> u16 {
    let aaaa = aa as u32;
    let bbbb = bb as u32;
    let mut vvvv = aaaa.wrapping_sub(bbbb);
    if env.state.reg.get_flag(Flag::C) {
        vvvv = vvvv.wrapping_sub(1);
    }
    let vv = vvvv as u16;

    // TUZD-8.6
    env.state
        .reg
        .update_arithmetic_flags_16(aaaa, bbbb, vvvv, true);
    env.state.reg.put_flag(Flag::Z, vv == 0);
    vv
}

pub fn operator_inc<M: Machine>(env: &mut Environment<M>, a: u8) -> u8 {
    let aa = a as u16;
    let vv = aa + 1;
    env.state
        .reg
        .update_arithmetic_flags(aa, 0, vv, false, false);
    vv as u8
}

pub fn operator_sub<M: Machine>(env: &mut Environment<M>, a: u8, b: u8) -> u8 {
    env.state.reg.clear_flag(Flag::C);
    operator_sbc(env, a, b)
}

pub fn operator_sbc<M: Machine>(env: &mut Environment<M>, a: u8, b: u8) -> u8 {
    let aa = a as u16;
    let bb = b as u16;
    let mut vv = aa.wrapping_sub(bb);
    if env.state.reg.get_flag(Flag::C) {
        vv = vv.wrapping_sub(1);
    }
    env.state
        .reg
        .update_arithmetic_flags(aa, bb, vv, true, true);
    vv as u8
}

pub fn operator_dec<M: Machine>(env: &mut Environment<M>, a: u8) -> u8 {
    let aa = a as u16;
    let vv = aa.wrapping_sub(1);
    env.state
        .reg
        .update_arithmetic_flags(aa, 0, vv, true, false);
    vv as u8
}

pub fn operator_and<M: Machine>(env: &mut Environment<M>, a: u8, b: u8) -> u8 {
    let v = a & b;
    env.state.reg.update_logic_flags(v, true);
    v
}

pub fn operator_xor<M: Machine>(env: &mut Environment<M>, a: u8, b: u8) -> u8 {
    let v = a ^ b;
    env.state.reg.update_logic_flags(v, false);
    v
}

pub fn operator_or<M: Machine>(env: &mut Environment<M>, a: u8, b: u8) -> u8 {
    let v = a | b;
    env.state.reg.update_logic_flags(v, false);
    v
}

pub fn operator_cp<M: Machine>(env: &mut Environment<M>, a: u8, b: u8) -> u8 {
    env.state.reg.clear_flag(Flag::C);
    operator_sub(env, a, b);

    // Note: flags 3 and 5 are taken from b. TUZD-8.4
    env.state.reg.update_undocumented_flags(b);
    a // Do not update the accumulator
}
