use super::environment::Environment;
use super::machine::Machine;
use super::opcode::Opcode;
use super::registers::{Reg16, Reg8};

/*
    From "The undocumented Z80 documented" TUZD-4.4:

Officially the Z80 has an 8 bit I/O port address space. When using the I/O ports, the 16 address
lines are used. And in fact, the high 8 bit do actually have some value, so you can use 65536
ports after all. IN r,(C), OUT (C),r, and the Block I/O instructions actually place the entire BC
register on the address bus. Similarly IN A,(n) and OUT (n),A put A × 256 + n on the address
bus.
The INI/INIR/IND/INDR instructions use BC after decrementing B, and the OUTI/OTIR/OUTD/OTDR
instructions before.
*/

pub fn build_out_c_r<M: Machine>(r: Reg8) -> Opcode<M> {
    Opcode::new(format!("OUT (C), {r}"), move |env: &mut Environment<M>| {
        let address = env.state.reg.get16(Reg16::BC);
        let value = env.state.reg.get8(r);
        env.port_out(address, value);
    })
}

pub fn build_out_c_0<M: Machine>() -> Opcode<M> {
    Opcode::new("OUT (C), 0".to_string(), |env: &mut Environment<M>| {
        let address = env.state.reg.get16(Reg16::BC);
        env.port_out(address, 0);
    })
}

pub fn build_out_n_a<M: Machine>() -> Opcode<M> {
    Opcode::new("OUT (n), A".to_string(), |env: &mut Environment<M>| {
        let a = env.state.reg.a();
        let address = u16::from_le_bytes([env.advance_pc(), a]);
        env.port_out(address, a);
    })
}

pub fn build_in_r_c<M: Machine>(r: Reg8) -> Opcode<M> {
    Opcode::new(format!("IN {r}, (C)"), move |env: &mut Environment<M>| {
        let address = env.state.reg.get16(Reg16::BC);
        let value = env.port_in(address);
        env.state.reg.set8(r, value);

        env.state.reg.update_bits_in_flags(value);
    })
}

pub fn build_in_0_c<M: Machine>() -> Opcode<M> {
    Opcode::new("IN (C)".to_string(), |env: &mut Environment<M>| {
        let address = env.state.reg.get16(Reg16::BC);
        let value = env.port_in(address);

        env.state.reg.update_bits_in_flags(value);
    })
}

pub fn build_in_a_n<M: Machine>() -> Opcode<M> {
    Opcode::new("IN A, (n)".to_string(), |env: &mut Environment<M>| {
        let a = env.state.reg.a();
        let address = u16::from_le_bytes([env.advance_pc(), a]);
        let value = env.port_in(address);
        env.state.reg.set_a(value);
    })
}

/*
, and the OUTI/OTIR/OUTD/OTDR
instructions before.
*/

pub fn build_in_block<M: Machine>((inc, repeat, postfix): (bool, bool, &'static str)) -> Opcode<M> {
    Opcode::new(format!("IN{postfix}"), move |env: &mut Environment<M>| {
        // The INI/INIR/IND/INDR instructions use BC after decrementing B
        let b = env.state.reg.inc_dec8(Reg8::B, false /* decrement */);
        let address = env.state.reg.get16(Reg16::BC);

        let value = env.port_in(address);
        // We won't have IX and IY cases to consider
        env.set_reg(Reg8::_HL, value);
        env.state.reg.inc_dec16(Reg16::HL, inc);

        // TUZD-4.3
        let j = (env.state.reg.get8(Reg8::C) as u16).wrapping_add(if inc { 1 } else { 0xFFFF });
        let k = value as u16 + (j & 0xff);
        env.state.reg.update_block_flags(value, k, b);

        if repeat && b != 0 {
            // Back to redo the instruction
            env.set_branch_taken();
            let pc = env.state.reg.pc().wrapping_sub(2);
            env.state.reg.set_pc(pc);
        }
    })
}

pub fn build_out_block<M: Machine>(
    (inc, repeat, postfix): (bool, bool, &'static str),
) -> Opcode<M> {
    let n0 = if repeat { "OT" } else { "OUT" };
    Opcode::new(format!("{n0}{postfix}"), move |env: &mut Environment<M>| {
        // the OUTI/OTIR/OUTD/OTDR instructions use BC before decrementing B
        let address = env.state.reg.get16(Reg16::BC);
        let b = env.state.reg.inc_dec8(Reg8::B, false /* decrement */);

        // We won't have IX and IY cases to consider
        let value = env.reg8_ext(Reg8::_HL);
        env.port_out(address, value);
        env.state.reg.inc_dec16(Reg16::HL, inc);

        // TUZD-4.3
        let k = value as u16 + env.state.reg.get8(Reg8::L) as u16;
        env.state.reg.update_block_flags(value, k, b);

        if repeat && b != 0 {
            // Back to redo the instruction
            env.set_branch_taken();
            let pc = env.state.reg.pc().wrapping_sub(2);
            env.state.reg.set_pc(pc);
        }
    })
}
