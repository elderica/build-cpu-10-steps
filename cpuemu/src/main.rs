#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq)]
enum OpCode {
    MOV = 0,
    ADD = 1,
    SUB = 2,
    AND = 3,
    OR = 4,
    SL = 5,
    SR = 6,
    SRA = 7,
    LDL = 8,
    LDH = 9,
    CMP = 10,
    JE = 11,
    JMP = 12,
    LD = 13,
    ST = 14,
    HLT = 15,
}

#[allow(dead_code)]
const REG0: RegisterIndex = 0;
#[allow(dead_code)]
const REG1: RegisterIndex = 1;
#[allow(dead_code)]
const REG2: RegisterIndex = 2;
#[allow(dead_code)]
const REG3: RegisterIndex = 3;
#[allow(dead_code)]
const REG4: RegisterIndex = 4;
#[allow(dead_code)]
const REG5: RegisterIndex = 5;
#[allow(dead_code)]
const REG6: RegisterIndex = 6;
#[allow(dead_code)]
const REG7: RegisterIndex = 7;

type Address = u16;
type Word = u16;
type Instruction = u16;
type RegisterIndex = usize;

#[derive(Debug)]
struct Chip {
    pc: u16,
    flag_eq: bool,
    reg: [Word; 8],
    rom: [Instruction; 256],
    ram: [Word; 256],
}

impl Chip {
    fn new() -> Self {
        let asm = assembler();
        let mut rom = [0; 256];
        rom[..asm.len()].copy_from_slice(&asm);
        Self {
            pc: 0,
            flag_eq: false,
            reg: [0; 8],
            rom,
            ram: [0; 256],
        }
    }

    fn fetch(&self) -> Instruction {
        self.rom[self.pc as usize]
    }

    fn is_halt(&self) -> bool {
        let ir = self.fetch();
        op_code(ir) == OpCode::HLT
    }

    fn tick(&mut self) {
        let ir = self.fetch();
        println!(
            "pc:{:5} ir:{:5x} reg0:{:5} reg1:{:5} reg2{:5} reg3:{:5}",
            self.pc, ir, self.reg[0], self.reg[1], self.reg[2], self.reg[3]
        );
        self.pc += 1;

        let ra = op_reg_a(ir);
        let rb = op_reg_b(ir);
        let addr = op_addr(ir);
        let data = op_data(ir);
        use OpCode::*;
        match op_code(ir) {
            MOV => self.reg[ra] = self.reg[rb],
            ADD => self.reg[ra] += self.reg[rb],
            SUB => self.reg[ra] -= self.reg[rb],
            AND => self.reg[ra] &= self.reg[rb],
            OR => self.reg[ra] |= self.reg[rb],
            SL => self.reg[ra] <<= 1,
            SR => self.reg[ra] >>= 1,
            SRA => self.reg[ra] = (self.reg[ra] & 0x8000) | (self.reg[ra] >> 1),
            LDL => self.reg[ra] = (self.reg[ra] & 0xff00) | (data & 0x00ff),
            LDH => self.reg[ra] = (data << 8) | (self.reg[ra] & 0x00ff),
            CMP => self.flag_eq = self.reg[ra] == self.reg[rb],
            JE => {
                if self.flag_eq {
                    self.pc = op_addr(ir)
                }
            }
            JMP => self.pc = op_addr(ir),
            LD => self.reg[ra] = self.ram[addr as usize],
            ST => self.ram[addr as usize] = self.reg[ra],
            HLT => (),
        }
    }
}

fn assembler() -> Vec<Instruction> {
    vec![
        ldh(REG0, 0),
        ldl(REG0, 0),
        ldh(REG1, 0),
        ldl(REG1, 1),
        ldh(REG2, 0),
        ldl(REG2, 0),
        ldh(REG3, 0),
        ldl(REG3, 10),
        add(REG2, REG1),
        add(REG0, REG2),
        st(REG0, 64),
        cmp(REG2, REG3),
        je(14),
        jmp(8),
        hlt(),
    ]
}

fn main() {
    let mut chip = Chip::new();

    while !chip.is_halt() {
        chip.tick();
    }

    println!("ram[64] = {:}", chip.ram[64])
}

#[allow(dead_code)]
fn mov(ra: RegisterIndex, rb: RegisterIndex) -> Instruction {
    ((OpCode::MOV as u16) << 11) | ((ra as u16) << 8) | ((rb as u16) << 5)
}

#[allow(dead_code)]
fn add(ra: RegisterIndex, rb: RegisterIndex) -> Instruction {
    ((OpCode::ADD as u16) << 11) | ((ra as u16) << 8) | ((rb as u16) << 5)
}

#[allow(dead_code)]
fn sub(ra: RegisterIndex, rb: RegisterIndex) -> Instruction {
    ((OpCode::SUB as u16) << 11) | ((ra as u16) << 8) | ((rb as u16) << 5)
}

#[allow(dead_code)]
fn and(ra: RegisterIndex, rb: RegisterIndex) -> Instruction {
    ((OpCode::AND as u16) << 11) | ((ra as u16) << 8) | ((rb as u16) << 5)
}

#[allow(dead_code)]
fn or(ra: RegisterIndex, rb: RegisterIndex) -> Instruction {
    ((OpCode::OR as u16) << 11) | ((ra as u16) << 8) | ((rb as u16) << 5)
}

#[allow(dead_code)]
fn sl(ra: RegisterIndex) -> Instruction {
    ((OpCode::SL as u16) << 11) | ((ra as u16) << 8)
}

#[allow(dead_code)]
fn sr(ra: RegisterIndex) -> Instruction {
    ((OpCode::SR as u16) << 11) | ((ra as u16) << 8)
}

#[allow(dead_code)]
fn sra(ra: RegisterIndex) -> Instruction {
    ((OpCode::SRA as u16) << 11) | ((ra as u16) << 8)
}

#[allow(dead_code)]
fn ldl(ra: RegisterIndex, imm: u16) -> Instruction {
    ((OpCode::LDL as u16) << 11) | ((ra as u16) << 8) | (imm & 0x00ff)
}

#[allow(dead_code)]
fn ldh(ra: RegisterIndex, imm: u16) -> Instruction {
    ((OpCode::LDH as u16) << 11) | ((ra as u16) << 8) | (imm & 0x00ff)
}

#[allow(dead_code)]
fn cmp(ra: RegisterIndex, rb: RegisterIndex) -> Instruction {
    ((OpCode::CMP as u16) << 11) | ((ra as u16) << 8) | ((rb as u16) << 5)
}

#[allow(dead_code)]
fn je(addr: Address) -> Instruction {
    ((OpCode::JE as u16) << 11) | (addr & 0x00ff)
}

#[allow(dead_code)]
fn jmp(addr: Address) -> Instruction {
    ((OpCode::JMP as u16) << 11) | (addr & 0x00ff)
}

#[allow(dead_code)]
fn ld(ra: RegisterIndex, addr: u16) -> Instruction {
    ((OpCode::LD as u16) << 11) | ((ra as u16) << 8) | (addr & 0x00ff)
}

#[allow(dead_code)]
fn st(ra: RegisterIndex, addr: u16) -> Instruction {
    ((OpCode::ST as u16) << 11) | ((ra as u16) << 8) | (addr & 0x00ff)
}

#[allow(dead_code)]
fn hlt() -> Instruction {
    (OpCode::HLT as u16) << 11
}

fn op_code(ir: Instruction) -> OpCode {
    use OpCode::*;
    match ir >> 11 {
        0 => MOV,
        1 => ADD,
        2 => SUB,
        3 => AND,
        4 => OR,
        5 => SL,
        6 => SR,
        7 => SRA,
        8 => LDL,
        9 => LDH,
        10 => CMP,
        11 => JE,
        12 => JMP,
        13 => LD,
        14 => ST,
        15 => HLT,
        _ => panic!("invalid opcode"),
    }
}

fn op_reg_a(ir: Instruction) -> RegisterIndex {
    ((ir >> 8) & 0x0007).into()
}

fn op_reg_b(ir: Instruction) -> RegisterIndex {
    ((ir >> 5) & 0x0007).into()
}

fn op_data(ir: Instruction) -> Word {
    ir & 0x00ff
}

fn op_addr(ir: Instruction) -> Address {
    ir & 0x00ff
}
