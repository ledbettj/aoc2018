use std::collections::HashMap;

const INPUT : &str = include_str!("../inputs/day19.txt");
const SAMPLE : &str = include_str!("../inputs/day19.sample.txt");

#[derive(Debug)]
pub enum OpType {
    RR,
    RI,
    IR
}

#[derive(Debug,PartialEq,Eq,Hash)]
pub enum OpCode {
    Add,
    Mul,
    And,
    Or,
    Set,
    Gt,
    Eq
}

pub type OpFn = Fn(usize, usize) -> usize;

pub struct Machine {
    registers: [usize; 6],
    ip: usize,
    ip_index: usize,
    table: HashMap<OpCode, Box<OpFn>>
}

#[derive(Debug)]
pub struct Instr {
    code: OpCode,
    optype: OpType,
    args: [usize; 3]
}

impl Instr {
    pub fn load(input: &str) -> Vec<Instr> {
        input
            .lines()
            .skip(1) // skip # comment
            .map(|line| Instr::parse(line))
            .collect()
    }

    pub fn parse(line: &str) -> Instr {
        let mut tokens = line.split(" ");
        let instr = tokens.next().unwrap();
        let args : Vec<usize> = tokens
            .map(|t| t.parse::<usize>().unwrap())
            .collect();

        let (code, optype) = Instr::parse_instr(instr);

        Instr {
            code: code,
            optype: optype,
            args: [args[0], args[1], args[2]]
        }
    }

    fn parse_instr(instr: &str) -> (OpCode, OpType) {
        let t = if instr.ends_with("ir") {
            OpType::IR
        } else if instr.ends_with("ri") {
            OpType::RI
        } else if instr.ends_with("i") {
            OpType::IR
        } else if instr.ends_with("r") {
            OpType::RR
        } else {
            panic!("Bad instruction suffix: {}", instr);
        };

        let code = match instr {
            "addi" | "addr" => OpCode::Add,
            "muli" | "mulr" => OpCode::Mul,
            "bani" | "banr" => OpCode::And,
            "bori" | "borr" => OpCode::Or,
            "seti" | "setr" => OpCode::Set,
            "gtrr" | "gtri" | "gtir" => OpCode::Gt,
            "eqrr" | "eqir" | "eqri" => OpCode::Eq,
            _ => panic!("Bad input instruction")
        };


        (code, t)
    }
}

impl Machine {
    pub fn new(ip_index: usize) -> Machine {
        Machine {
            registers: [0; 6],
            ip: 0,
            ip_index: ip_index,
            table: Machine::build_table()
        }
    }

    pub fn exec(&mut self, program: &Vec<Instr>) {
        self.ip = self.registers[self.ip_index];

        loop {

            if self.ip >= program.len() {
                break;
            }

            let instr = &program[self.ip];
            let func = &self.table[&instr.code];

            let (a, b) = match instr.optype {
                OpType::RR => (self.registers[instr.args[0]], self.registers[instr.args[1]]),
                OpType::IR => (instr.args[0], self.registers[instr.args[1]]),
                OpType::RI => (self.registers[instr.args[0]], instr.args[1])
            };

            self.registers[instr.args[2]] = func(a, b);
            self.ip = self.registers[self.ip_index] + 1;
            self.registers[self.ip_index] = self.ip;
        }
    }

    fn build_table() -> HashMap<OpCode, Box<OpFn>> {
        let mut m : HashMap<OpCode, Box<OpFn>> = HashMap::new();
        m.insert(OpCode::Add, Box::new(|a, b| a + b));
        m.insert(OpCode::Mul, Box::new(|a, b| a * b));
        m.insert(OpCode::And, Box::new(|a, b| a & b));
        m.insert(OpCode::Or, Box::new(|a, b| a | b));
        m.insert(OpCode::Set, Box::new(|a, _| a));
        m.insert(OpCode::Gt, Box::new(|a, b| if a > b { 1 } else { 0 } ));
        m.insert(OpCode::Eq, Box::new(|a, b| if a == b { 1 } else { 0 } ));

        m
    }
}


mod tests {
    use super::*;

    #[test]
    fn day19_p1() {
        let prgm = Instr::load(SAMPLE);
        let mut m = Machine::new(0);

        m.exec(&prgm);

        assert_eq!(m.registers[0], 0);
    }

}
