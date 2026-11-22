use serde::{Deserialize, Serialize};

use super::decoder::Decoder;
use super::environment::Environment;
use super::machine::Machine;
use super::registers::{Reg8, Registers};
use super::state::State;

const IRQ_ADDRESS: u16 = 0x0038;
const NMI_ADDRESS: u16 = 0x0066;

/// The Z80 cpu emulator.
///
/// Executes Z80 instructions changing the cpu State and Machine
pub struct Cpu<M: Machine> {
    state: State,
    decoder: Box<Decoder<M>>,
}

// Ensure that the Cpu is Send and Sync and can be used with async code
const _: () = {
    fn assert_send<T: Send + Sync>() {}
    let _ = assert_send::<Cpu<super::machine::PlainMachine>>;
};

impl<M: Machine> Cpu<M> {
    /// Returns a Z80 Cpu instance
    pub fn new() -> Cpu<M> {
        Cpu {
            state: State::new(),
            decoder: Box::new(Decoder::new()),
        }
    }
}

impl<M: Machine> Default for Cpu<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: Machine> Cpu<M> {
    /// Executes a single instruction
    ///
    /// # Arguments
    ///
    /// * `sys` - A representation of the emulated machine that has the Machine trait
    ///
    pub fn execute_instruction(&mut self, sys: &mut M) {
        if self.is_halted() {
            // The CPU is in HALT state. Only interrupts can execute.
            return;
        }

        let mut env = Environment::new(&mut self.state, sys);
        if env.state.reset_pending {
            env.state.reset_pending = false;
            env.state.nmi_pending = false;
            env.state.halted = false;
            env.state.reg.reset();
            env.state.cycle = env.state.cycle.wrapping_add(3);
        } else if env.state.nmi_pending {
            env.state.nmi_pending = false;
            env.state.halted = false;
            env.state.reg.start_nmi();
            env.state.cycle = env.state.cycle.wrapping_add(11);
            env.subroutine_call(NMI_ADDRESS);
        } else if env.state.int_signaled {
            let (int_enabled, int_mode) = env.state.reg.get_interrupt_mode();
            if int_enabled && !env.state.int_just_enabled {
                let id = env.int_device_id();
                env.state.halted = false;
                env.state.reg.set_interrupts(false);
                match int_mode {
                    0 => {
                        // TODO: より正確にに挙動を合わせないといけない
                        let opcode = self.decoder.decode_with_firstcode(&mut env, id);
                        env.clear_branch_taken();
                        env.clear_int_just_enabled();
                        opcode.execute(&mut env);
                        env.advance_cycles(opcode);
                        env.clear_index();
                    }
                    1 => {
                        env.state.cycle = env.state.cycle.wrapping_add(13);
                        env.subroutine_call(IRQ_ADDRESS);
                    }
                    2 => {
                        // TODO: サイクル数を正確にする
                        let vec_addr = u16::from_le_bytes([id, env.state.reg.get8(Reg8::I)]);
                        let address = env.sys.peek16(vec_addr);
                        env.subroutine_call(address);
                    }
                    _ => panic!("Invalid interrupt mode"),
                }
            }
        }

        let opcode = self.decoder.decode(&mut env);

        env.clear_branch_taken();
        env.clear_int_just_enabled();
        opcode.execute(&mut env);
        env.advance_cycles(opcode);
        env.clear_index();
    }

    /// Returns the instruction in PC disassembled. PC is advanced.
    ///
    /// # Arguments
    ///
    /// * `sys` - A representation of the emulated machine that has the Machine trait
    ///
    #[cfg(feature = "disasm")]
    pub fn disasm_instruction(&mut self, sys: &mut M) -> String {
        let mut env = Environment::new(&mut self.state, sys);
        let opcode = self.decoder.decode(&mut env);
        opcode.disasm(&mut env)
    }

    /// Returns a Registers struct to read and write on the Z80 registers
    pub fn registers(&mut self) -> &mut Registers {
        &mut self.state.reg
    }

    /// Returns an immutable references to Registers struct to read the Z80 registers
    pub fn immutable_registers(&self) -> &Registers {
        &self.state.reg
    }

    /// Returns if the Cpu has executed a HALT
    pub fn is_halted(&self) -> bool {
        self.state.halted
            && !self.state.nmi_pending
            && !self.state.reset_pending
            && !self.state.int_signaled
    }

    /// Maskable interrupt request. It stays signaled until is is
    /// deactivated by calling `signal_interrupt(false)`.
    pub fn signal_interrupt(&mut self, active: bool) {
        self.state.int_signaled = active;
    }

    /// Non maskable interrupt request
    pub fn signal_nmi(&mut self) {
        self.state.nmi_pending = true;
    }

    /// Signal reset
    pub fn signal_reset(&mut self) {
        self.state.reset_pending = true;
    }

    /// Returns the current cycle count
    pub fn cycle_count(&self) -> u64 {
        self.state.cycle
    }
}

impl<M: Machine> Serialize for Cpu<M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.state.serialize(serializer)
    }
}

impl<'de, M: Machine> Deserialize<'de> for Cpu<M> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Cpu {
            state: State::deserialize(deserializer)?,
            decoder: Box::new(Decoder::new()),
        })
    }
}
