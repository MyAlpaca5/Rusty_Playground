// modified from the example in Rust in Action
// the goal is to emulate a simplified version of instruction set called CHIP-8
// only include subset of the supported instruction set, https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set

struct CHIP8 {
    memory: [u8; 4096],
    program_counter: usize,
    stack: [u16; 16],
    stack_pointer: usize,
    registers: [u8; 16],
}

impl CHIP8 {
    fn new() -> Self {
        CHIP8 {
            memory: [0; 4096],
            program_counter: 0,
            stack: [0; 16],
            stack_pointer: 0,
            registers: [0; 16],
        }
    }

    fn read_opcode(&mut self) -> u16 {
        let pc = self.program_counter;
        let op_fst_byte = self.memory[pc] as u16;
        let op_snd_byte = self.memory[pc + 1] as u16;

        self.program_counter += 2;

        op_fst_byte << 8 | op_snd_byte
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;
            let nn = (opcode & 0x00FF) as u8;
            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => {
                    self.ret();
                }
                (0x1, _, _, _) => {
                    self.jmp(nnn);
                }
                (0x2, _, _, _) => {
                    self.call(nnn);
                }
                (0x3, _, _, _) => {
                    self.skip_e(x, nn);
                }
                (0x4, _, _, _) => {
                    self.skip_ne(x, nn);
                }
                (0x5, _, _, 0) => {
                    self.skip_ve(x, y);
                }
                (0x6, _, _, _) => {
                    self.store_in_vx(x, nn);
                }
                (0x7, _, _, _) => {
                    self.add_to_vx(x, nn);
                }
                (0x8, _, _, _) => match d {
                    0 => self.store_vy_in_vx(x, y),
                    1 => self.or_xy(x, y),
                    2 => self.and_xy(x, y),
                    3 => self.xor_xy(x, y),
                    4 => {
                        self.add_vy_to_vx(x, y);
                    }
                    5 => {
                        self.sub_vy_from_vx(x, y);
                    }
                    _ => {
                        todo!("unsupported opcode: {:04x}", opcode);
                    }
                },
                _ => todo!("unsupported opcode {:04x}", opcode),
            }
        }
    }

    /// (00ee) returns from the current subroutine
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("stack underflow");
        }

        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    /// (1nnn) jumps to address `addr`
    fn jmp(&mut self, addr: u16) {
        self.program_counter = addr as usize;
    }

    /// (2nnn) execute subroutine starting at `addr`
    fn call(&mut self, addr: u16) {
        if self.stack_pointer == self.stack.len() {
            panic!("stack overflow");
        }

        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    /// (3xnn) skips the following instruction if value of
    /// register `vx` is equal to `nn`
    fn skip_e(&mut self, vx: u8, nn: u8) {
        if self.registers[vx as usize] == nn {
            self.program_counter += 2;
        }
    }

    /// (4xnn) skips the following instruction if value of
    /// register `vx` is not equal to `nn`
    fn skip_ne(&mut self, vx: u8, nn: u8) {
        if self.registers[vx as usize] != nn {
            self.program_counter += 2;
        }
    }

    /// (5xy0) skips the following instruction if the value of
    /// register `vx` is equal to the value of register `vy`
    fn skip_ve(&mut self, vx: u8, vy: u8) {
        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.program_counter += 2;
        }
    }

    /// (6xnn) stores `nn` in register `vx`
    fn store_in_vx(&mut self, vx: u8, nn: u8) {
        self.registers[vx as usize] = nn;
    }

    /// (7xnn) adds the value `nn` into register `vx`
    fn add_to_vx(&mut self, vx: u8, nn: u8) {
        self.registers[vx as usize] += nn;
    }

    /// (8xy0) stores the value of register `vy` in register `vx`
    fn store_vy_in_vx(&mut self, vx: u8, vy: u8) {
        self.registers[vx as usize] = self.registers[vy as usize];
    }

    /// (8xy1) set the value of `vx` to
    /// the value of `vx` OR the value of `vy`
    fn or_xy(&mut self, vx: u8, vy: u8) {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        self.registers[vx as usize] = x | y;
    }

    /// (8xy2) set the value of `vx` to
    /// the value of `vx` AND the value of `vy`
    fn and_xy(&mut self, vx: u8, vy: u8) {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        self.registers[vx as usize] = x & y;
    }

    /// (8xy3) set the value of `vx` to
    /// the value of `vx` XOR the value of `vy`
    fn xor_xy(&mut self, vx: u8, vy: u8) {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        self.registers[vx as usize] = x ^ y;
    }

    /// (8xy4) adds value in register `vy` into register `vx`.
    /// Set `vf` to 01 if a carry occurs, otherwise, set `vf` to 00
    fn add_vy_to_vx(&mut self, vx: u8, vy: u8) {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        let (res, carried) = x.overflowing_add(y);

        if carried {
            self.registers[0xF] = 0x01;
        } else {
            self.registers[0xF] = 0x00;
        }
        self.registers[vx as usize] = res;
    }

    /// (8xy5) subtract value in register `vy` from register `vx`.
    /// Set `vf` to 00 if a borrow occurs, otherwise, set `vf` to 01
    fn sub_vy_from_vx(&mut self, vx: u8, vy: u8) {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        let (res, borrowed) = x.overflowing_sub(y);

        if borrowed {
            self.registers[0xF] = 0x00;
        } else {
            self.registers[0xF] = 0x01;
        }
        self.registers[vx as usize] = res;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call_addxy_twice() {
        let mut chip = CHIP8::new();

        // call the add_xy twice
        chip.memory[0x0..0x6].copy_from_slice(&[0x22, 0x00, 0x22, 0x00, 0x00, 0x00]);

        // add_xy function
        chip.memory[0x200..0x204].copy_from_slice(&[0x80, 0x14, 0x00, 0xEE]);

        chip.registers[0] = 2;
        chip.registers[1] = 3;

        chip.run();

        assert_eq!(chip.registers[0], 8);
    }

    #[test]
    fn fibonacci_iterative() {
        // generate first 10 fibonacci number
        // let mut tmp = 0;
        // let mut prev_1 = 1;
        // let mut prev_2 = 0;

        // for _ in 2..10 {
        //     tmp = prev_1;
        //     prev_1 += prev_2;
        //     prev_2 = tmp;
        // }

        let mut chip = CHIP8::new();

        chip.registers[0] = 0; // tmp
        chip.registers[1] = 1; // prev_1
        chip.registers[2] = 0; // prev_2
        chip.registers[3] = 2; // i


        chip.memory[0x00..0x02].copy_from_slice(&[0x80, 0x10]);
        chip.memory[0x02..0x04].copy_from_slice(&[0x81, 0x24]);
        chip.memory[0x04..0x06].copy_from_slice(&[0x82, 0x00]);
        chip.memory[0x06..0x08].copy_from_slice(&[0x73, 0x01]);
        chip.memory[0x08..0x0A].copy_from_slice(&[0x33, 0x0A]);
        chip.memory[0x0A..0x0C].copy_from_slice(&[0x10, 0x00]);
        chip.memory[0x0C..0x0E].copy_from_slice(&[0x00, 0x00]);

        chip.run();

        assert_eq!(chip.registers[1], 34);
    }

}
