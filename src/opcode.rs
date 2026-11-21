use super::environment::Environment;
use super::machine::Machine;
use super::registers::Reg16;

type OpcodeFn<M> = dyn for<'a> Fn(&mut Environment<'a, M>) + Send + Sync;

pub struct Opcode<M: Machine> {
    pub name: String,
    pub cycles: u8,
    pub cycles_conditional: u8,
    pub action: Box<OpcodeFn<M>>,
}

impl<M: Machine> Opcode<M> {
    pub(crate) fn new<T>(name: String, action: T) -> Opcode<M>
    where
        T: for<'a> Fn(&mut Environment<'a, M>) + Send + Sync + 'static,
    {
        Opcode {
            name,
            cycles: 0,
            cycles_conditional: 0,
            action: Box::new(action),
        }
    }

    pub fn execute(&self, env: &mut Environment<M>) {
        (self.action)(env);
    }

    pub fn disasm(&self, env: &mut Environment<M>) -> String {
        let name = if self.name.contains("__index") {
            self.name.replace("__index", &env.index_description())
        } else {
            self.name.clone()
        };

        if self.name.contains("nn") {
            // Immediate argument 16 bits
            let nn = env.peek16_pc();
            let nn_str = format!("{nn:04x}h");
            name.replace("nn", &nn_str)
        } else if self.name.contains('n') {
            // Immediate argument 8 bits
            let n = env.peek_pc();
            let n_str = format!("{n:02x}h");
            name.replace('n', &n_str)
        } else if self.name.contains('d') {
            // Immediate argument 8 bits signed
            // In assembly it's shown with 2 added as if it were from the opcode pc.
            let d = env.peek_pc() as i8 as i16 + 2;
            let d_str = format!("{d:+x}");
            name.replace('d', &d_str)
        } else {
            name
        }
    }
}

pub fn build_not_an_opcode<M: Machine>() -> Opcode<M> {
    Opcode::new("NOT_AN_OPCODE".to_string(), |_: &mut Environment<M>| {
        panic!("Not an opcode")
    })
}

pub fn build_nop<M: Machine>() -> Opcode<M> {
    Opcode::new("NOP".to_string(), |_: &mut Environment<M>| {
        // Nothing done
    })
}

pub fn build_noni_nop<M: Machine>() -> Opcode<M> {
    Opcode::new("NONINOP".to_string(), |_: &mut Environment<M>| {
        // Nothing done
    })
}

pub fn build_halt<M: Machine>() -> Opcode<M> {
    Opcode::new("HALT".to_string(), |env: &mut Environment<M>| {
        env.state.halted = true;
    })
}

pub fn build_pop_rr<M: Machine>(rr: Reg16) -> Opcode<M> {
    Opcode::new(format!("POP {rr:?}"), move |env: &mut Environment<M>| {
        let value = env.pop();
        env.set_reg16(rr, value);
    })
}

pub fn build_push_rr<M: Machine>(rr: Reg16) -> Opcode<M> {
    Opcode::new(format!("PUSH {rr:?}"), move |env: &mut Environment<M>| {
        let value = env.reg16_ext(rr);
        env.push(value);
    })
}

pub fn build_disable_interrupts<M: Machine>() -> Opcode<M> {
    Opcode::new("DI".to_string(), |env: &mut Environment<M>| {
        env.state.reg.set_interrupts(false);
    })
}

pub fn build_enable_interrupts<M: Machine>() -> Opcode<M> {
    Opcode::new("EI".to_string(), |env: &mut Environment<M>| {
        env.state.reg.set_interrupts(true);
        env.state.int_just_enabled = true;
    })
}

pub fn build_im<M: Machine>(im: u8) -> Opcode<M> {
    Opcode::new(format!("IM {im}"), move |env: &mut Environment<M>| {
        env.state.reg.set_interrupt_mode(im);
    })
}
