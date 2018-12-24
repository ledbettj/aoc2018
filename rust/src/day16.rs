use std::collections::HashMap;
use regex::Regex;

const INPUT : &str = include_str!("../inputs/day16.txt");

#[derive(Debug)]
pub struct Instruction {
    opcode: usize,
    args:   Vec<usize>,
    before: Vec<usize>,
    after:  Vec<usize>
}

#[derive(PartialEq,Clone,Copy)]
pub enum OpType {
    RegReg,
    ImdReg,
    RegImd
}

pub type OpFn = Fn(usize, usize) -> usize;

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


pub fn identify_instructions(instr: &Vec<Instruction>) ->
    HashMap<usize, (OpType, &'static OpFn, &'static str)>
{
    let mut m = HashMap::new();
    let mut items : Vec<(OpType, &'static OpFn, &'static str)> = vec![
        (OpType::RegReg, &add, "add"),
        (OpType::RegReg, &mul, "mul"),
        (OpType::RegReg, &ban, "ban"),
        (OpType::RegReg, &bor, "bor"),
        (OpType::RegReg, &set, "set"),
        (OpType::RegReg, &gt, "gt"),
        (OpType::RegReg, &eq, "eq"),

        (OpType::RegImd, &add, "add"),
        (OpType::RegImd, &mul, "mul"),
        (OpType::RegImd, &ban, "ban"),
        (OpType::RegImd, &bor, "bor"),
        (OpType::RegImd, &set, "set"),
        (OpType::RegImd, &gt, "gt"),
        (OpType::RegImd, &eq, "eq"),

        (OpType::ImdReg, &gt, "gt"),
        (OpType::ImdReg, &eq, "eq")
    ];


    while !items.is_empty() {

        for i in instr.iter() {
            if m.contains_key(&i.opcode) {
                continue;
            }

            let possible : Vec<(OpType, &'static OpFn, &'static str)> = items
                .iter()
                .filter(|&(t, f, _)| i.exec(*t, f) == i.after)
                .map(|x| x.clone())
                .collect();

            if possible.len() == 0 {
                println!("wtf no possible results for {}", i.opcode);
                panic!("shit");
            }

            if possible.len() == 1 {
                m.insert(i.opcode, possible[0]);
                items.retain(|&(t, _, n)|{
                        t != possible[0].0 || n != possible[0].2
                    });
                println!("identified {}, {} remaining", i.opcode, items.len());
            }

        }

    }

    m
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

    pub fn exec<F>(&self, mode: OpType, func: F) -> Vec<usize>
    where F: Fn(usize, usize) -> usize {
        let (a, b, c) = self.unpack_args();

        let (r1, r2) = match mode {
            OpType::RegReg => (self.before[a], self.before[b]),
            OpType::RegImd => (self.before[a], b),
            OpType::ImdReg => (a, self.before[b])
        };

        let mut after = self.after.clone();
        after[c] = func(r1, r2);

        after
    }

    fn possible(&self) -> Vec<(OpType, &'static OpFn)> {
        let funcs : Vec<&'static OpFn> = vec![
            &add, &mul, &ban, &bor, &set, &gt, &eq
        ];

        let extra : Vec<&'static OpFn> = vec![&gt, &eq];

        let mut results = vec![];

        for f in funcs {
            if self.exec(OpType::RegReg, f) == self.after {
                results.push((OpType::RegReg, f));
            }
            if self.exec(OpType::RegImd, f) == self.after {
                results.push((OpType::RegImd, f));
            }
        }

        for f in extra {
            if self.exec(OpType::ImdReg, f) == self.after {
                results.push((OpType::ImdReg, f));
            }
        }

        results
    }

    pub fn possible_count(&self) -> usize {
        self.possible().len()
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

    #[test]
    fn day16_p2() {
        let instrs = Instruction::load(INPUT);
        let m = identify_instructions(&instrs);

    }
}
