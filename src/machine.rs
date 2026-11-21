/// Abstraction of the device hosting the Z80 CPU
///
/// The device hosting the CPU has to provide implementations
/// of the memory and port access. A simple implementation is
/// provided with `PlainMachine`
pub trait Machine {
    /// Returns the memory contents in [address]
    fn peek(&mut self, address: u16) -> u8;

    /// Sets the memory content to [value] in [address]
    fn poke(&mut self, address: u16, value: u8);

    /// Returns the memory contents in [address] as word
    fn peek16(&mut self, address: u16) -> u16 {
        u16::from_le_bytes([self.peek(address), self.peek(address.wrapping_add(1))])
    }

    /// Sets the memory content to the word [value] in [address]
    fn poke16(&mut self, address: u16, value: u16) {
        let [low, hi] = value.to_le_bytes();
        self.poke(address, low);
        self.poke(address.wrapping_add(1), hi);
    }

    /// Port in, from the device to the CPU. Returns the port value
    /// in the hosting device.
    fn port_in(&mut self, address: u16) -> u8;
    /// Port out, from the CPU to the device. Sets a port value on
    /// the hosting device.
    fn port_out(&mut self, address: u16, value: u8);

    fn int_device_id(&mut self, pc: u16) -> u8;
}

/// A simple Machine implementation
///
/// A minimum implementation of Machine. It uses two arrays of 65536 bytes to back the peeks and
/// pokes to memory and the ins and outs of ports.
pub struct PlainMachine {
    mem: [u8; 65536],
    io: [u8; 65536],
}

impl PlainMachine {
    /// Returns a new `PlainMachine` instance
    pub fn new() -> PlainMachine {
        PlainMachine {
            mem: [0; 65536],
            io: [0; 65536],
        }
    }
}

impl Default for PlainMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl Machine for PlainMachine {
    fn peek(&mut self, address: u16) -> u8 {
        self.mem[address as usize]
    }
    fn poke(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    fn port_in(&mut self, address: u16) -> u8 {
        self.io[address as usize]
    }
    fn port_out(&mut self, address: u16, value: u8) {
        self.io[address as usize] = value;
    }

    fn int_device_id(&mut self, _pc: u16) -> u8 {
        0xFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_byte() {
        let mut m = PlainMachine::new();
        const A: u16 = 0x2345;
        const V: u8 = 0xa0;

        m.poke(A, V);
        assert_eq!(V, m.peek(A));
    }
}
