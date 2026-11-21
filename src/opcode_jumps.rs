use super::environment::Environment;
use super::machine::Machine;
use super::opcode::Opcode;
use super::registers::{Flag, Reg8};

// Relative jumps
pub fn build_djnz<M: Machine>() -> Opcode<M> {
    Opcode::new("DJNZ d".to_string(), |env: &mut Environment<M>| {
        let offset = env.advance_pc();
        let b = env.state.reg.get8(Reg8::B).wrapping_add(0xff /* -1 */);
        env.state.reg.set8(Reg8::B, b);
        if b != 0 {
            // Condition not met
            env.set_branch_taken();
            relative_jump(env, offset);
        }
    })
}

pub fn build_jr_unconditional<M: Machine>() -> Opcode<M> {
    Opcode::new("JR d".to_string(), |env: &mut Environment<M>| {
        let offset = env.advance_pc();
        relative_jump(env, offset);
    })
}

pub fn build_jr_eq<M: Machine>((flag, value, name): (Flag, bool, &str)) -> Opcode<M> {
    Opcode::new(format!("JR {name}, d"), move |env: &mut Environment<M>| {
        let offset = env.advance_pc();
        if env.state.reg.get_flag(flag) == value {
            env.set_branch_taken();
            relative_jump(env, offset);
        }
    })
}

fn relative_jump<M: Machine>(env: &mut Environment<M>, offset: u8) {
    let mut pc = env.state.reg.pc();
    pc = pc.wrapping_add(offset as i8 as i16 as u16);
    env.state.reg.set_pc(pc);
}

// Absolute jumps
pub fn build_jp_unconditional<M: Machine>() -> Opcode<M> {
    Opcode::new("JP nn".to_string(), |env: &mut Environment<M>| {
        let address = env.advance_immediate16();
        env.state.reg.set_pc(address);
    })
}

pub fn build_jp_eq<M: Machine>((flag, value, name): (Flag, bool, &str)) -> Opcode<M> {
    Opcode::new(format!("JP {name}, nn"), move |env: &mut Environment<M>| {
        let address = env.advance_immediate16();
        if env.state.reg.get_flag(flag) == value {
            env.set_branch_taken();
            env.state.reg.set_pc(address);
        }
    })
}

pub fn build_jp_hl<M: Machine>() -> Opcode<M> {
    Opcode::new(
        "JP HL".to_string(), // Note: it is usaully written as JP (HL)
        |env: &mut Environment<M>| {
            // Note: no displacement added to the index
            let address = env.index_value();
            env.state.reg.set_pc(address);
        },
    )
}

// Calls to subroutine
pub fn build_call<M: Machine>() -> Opcode<M> {
    Opcode::new("CALL nn".to_string(), |env: &mut Environment<M>| {
        let address = env.advance_immediate16();
        env.subroutine_call(address);
    })
}

pub fn build_call_eq<M: Machine>((flag, value, name): (Flag, bool, &str)) -> Opcode<M> {
    Opcode::new(
        format!("CALL {name}, nn"),
        move |env: &mut Environment<M>| {
            let address = env.advance_immediate16();
            if env.state.reg.get_flag(flag) == value {
                env.set_branch_taken();
                env.subroutine_call(address);
            }
        },
    )
}

pub fn build_rst<M: Machine>(d: u8) -> Opcode<M> {
    Opcode::new(format!("RST {d:02x}h"), move |env: &mut Environment<M>| {
        let address = d as u16;
        env.subroutine_call(address);
    })
}

// Returns

pub fn build_ret<M: Machine>() -> Opcode<M> {
    Opcode::new("RET".to_string(), |env: &mut Environment<M>| {
        env.subroutine_return();
    })
}

pub fn build_reti<M: Machine>() -> Opcode<M> {
    Opcode::new("RETI".to_string(), |env: &mut Environment<M>| {
        env.subroutine_return();
    })
}

pub fn build_retn<M: Machine>() -> Opcode<M> {
    Opcode::new("RETN".to_string(), |env: &mut Environment<M>| {
        env.subroutine_return();
        env.state.reg.end_nmi();
    })
}

pub fn build_ret_eq<M: Machine>((flag, value, name): (Flag, bool, &str)) -> Opcode<M> {
    Opcode::new(format!("RET {name}"), move |env: &mut Environment<M>| {
        if env.state.reg.get_flag(flag) == value {
            env.set_branch_taken();
            env.subroutine_return();
        }
    })
}
