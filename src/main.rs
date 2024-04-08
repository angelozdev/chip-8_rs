#[derive(Debug)]
struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn new() -> Self {
        CPU {
            registers: [0; 16],
            position_in_memory: 0,
            memory: [0; 0x1000],
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte_1 = self.memory[p] as u16;
        let op_byte_2 = self.memory[p + 1] as u16;

        op_byte_1 << 8 | op_byte_2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            println!("Current position in memory: 0x{:04x}", opcode);
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let n = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;
            // let kk = (opcode & 0x00FF) as u8;

            match (c, x, y, n) {
                (0, 0, 0, 0) => break,
                (_, _, 0xE, 0xE) => {
                    println!("Returning from subroutine");
                    self.return_from_subroutine()
                }
                (0x2, _, _, _) => {
                    println!("Calling subroutine at 0x{:04x}", nnn);
                    self.call_subroutine(nnn)
                }
                (0x8, _, _, 0x4) => {
                    println!("Adding V{:x} and V{:x}", x, y);
                    self.add(x, y)
                }
                _ => todo!("opcode not implemented: 0x{:04x}", opcode),
            }
        }
    }

    fn add(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (result, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = result;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn call_subroutine(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn return_from_subroutine(&mut self) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let call_addr = stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }
}

fn main() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; // it calls subroutine at 0x100
    mem[0x001] = 0x00;

    mem[0x002] = 0x21; // it calls subroutine at 0x100
    mem[0x003] = 0x00;

    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    mem[0x100] = 0x80;
    mem[0x101] = 0x14;

    mem[0x102] = 0x80;
    mem[0x103] = 0x14;

    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
