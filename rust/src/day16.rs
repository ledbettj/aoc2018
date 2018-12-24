
use regex::Regex;

const INPUT : &str = include_str!("../inputs/day16.txt");

#[derive(Debug)]
struct Instruction {
    opcode: usize,
    args:   Vec<usize>,
    before: Vec<usize>,
    after:  Vec<usize>
}


pub fn add(a: usize, b: usize) -> usize {
    a + b
}

pub fn mul(a: usize, b: usize) -> usize {
    a * b
}

pub fn ban(a: usize, b: usize) -> usize {
    a & b
}

pub fn bor(a: usize, b: usize) -> usize {
    a | b
}

pub fn set(a: usize, _: usize) -> usize {
    a
}

pub fn gt(a: usize, b: usize) -> usize {
    if a > b { 1 } else { 0 }
}

pub fn eq(a: usize, b: usize) -> usize {
    if a == b { 1 } else { 0 }
}


impl Instruction {
    pub fn load(input: &str) -> Vec<Instruction> {
        let lines : Vec<&str> = input.lines().collect();
        let r = Regex::new(r"\w+:\s+\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();

        lines
            .chunks(4)
            .map(|slice| Instruction::parse(&slice, &r))
            .collect()
    }

    pub fn parse(lines: &[&str], regex: &regex::Regex) -> Instruction {

        println!("PARSING: {:?}", lines);

        let mut parts = lines[1]
            .split(" ")
            .map(|i| i.parse::<usize>().unwrap());

        let before = regex.captures(lines[0]).unwrap()
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<usize>().unwrap())
            .collect();

        let after = regex.captures(lines[2]).unwrap()
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<usize>().unwrap())
            .collect();


        Instruction {
            opcode: parts.next().unwrap(),
            args:   parts.collect(),
            before: before,
            after:  after
        }
    }

    fn check_rr_bin_expr<F>(&self, func: F) -> bool
    where F: Fn(usize, usize) -> usize {
        let (a, b, c) = self.unpack_args();

        let mut after = self.after.clone();
        after[c] = func(self.before[a], self.before[b]);

        after == self.after
    }

    fn check_ri_bin_expr<F>(&self, func: F) -> bool
    where F: Fn(usize, usize) -> usize {
        let (a, b, c) = self.unpack_args();

        let mut after = self.after.clone();
        after[c] = func(self.before[a], b);

        after == self.after
    }

    fn check_ir_bin_expr<F>(&self, func: F) -> bool
    where F: Fn(usize, usize) -> usize {
        let (a, b, c) = self.unpack_args();

        let mut after = self.after.clone();
        after[c] = func(a, self.before[b]);

        after == self.after
    }

    pub fn possible_count(&self) -> usize {
        let mut count = 0;

        let funcs : Vec<&Fn(usize, usize) -> usize> = vec![
            &add, &mul, &ban, &bor, &set, &gt, &eq
        ];

        let extra : Vec<&Fn(usize, usize) -> usize> = vec![
            &gt, &eq
        ];

        let mut matches = funcs
            .iter()
            .filter(|f| self.check_rr_bin_expr(f))
            .count();

        matches += funcs
            .iter()
            .filter(|f| self.check_ri_bin_expr(f))
            .count();

        matches += extra.iter().filter(|f| self.check_ir_bin_expr(f)).count();

        matches
    }


    fn unpack_args(&self) -> (usize, usize, usize) {
        (self.args[0], self.args[1], self.args[2])
    }
}


mod tests {
    use super::*;

    #[test]
    fn day16_p1() {
        let instrs = Instruction::load(INPUT);
        let c = instrs
            .iter()
            .filter(|i| i.possible_count() >= 3)
            .count();

        assert_eq!(c, 567);
    }
}
