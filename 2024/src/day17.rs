use aoc_common::{AocDay, DayError};
use itertools::Itertools;
use z3::{
    ast::{Ast, BV},
    Config, Context, Optimize, SatResult,
};

type LiteralOperand = u8;

#[allow(clippy::upper_case_acronyms)]
enum OpCode {
    ADV(ComboOperand),   // 0: division, store A
    BXL(LiteralOperand), // 1: bitwise xor B and literal
    BST(ComboOperand),   // 2: combo mod 8
    JNZ(LiteralOperand), // 3: jump not zero
    BXC,                 // 4: bitwise xor B and C
    OUT(ComboOperand),   // 5: print
    BDV(ComboOperand),   // 6: division, store B
    CDV(ComboOperand),   // 7: division, store C
}

impl TryFrom<(u8, u8)> for OpCode {
    type Error = DayError;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        let (opcode, operand) = value;
        match opcode {
            0 => Ok(Self::ADV(operand.try_into()?)),
            1 => Ok(Self::BXL(operand)),
            2 => Ok(Self::BST(operand.try_into()?)),
            3 => Ok(Self::JNZ(operand)),
            4 => Ok(Self::BXC),
            5 => Ok(Self::OUT(operand.try_into()?)),
            6 => Ok(Self::BDV(operand.try_into()?)),
            7 => Ok(Self::CDV(operand.try_into()?)),
            _ => Err(DayError::GenericParseErr(
                "can't parse opcode: unknown opcode",
            )),
        }
    }
}

enum ComboOperand {
    Literal(LiteralOperand),
    Register(usize),
}

impl TryFrom<u8> for ComboOperand {
    type Error = DayError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=3 => Ok(Self::Literal(value)),
            4..=6 => Ok(Self::Register((value - 4).into())),
            _ => Err(DayError::GenericParseErr(
                "can't parse combo operand: unknown operand",
            )),
        }
    }
}

struct ExecutionContext<'a> {
    registers: [u32; 3],
    ip: usize,
    instructions: &'a [u8],
    output: Vec<u8>,
}

impl<'a> ExecutionContext<'a> {
    fn new(registers: [u32; 3], instructions: &'a [u8]) -> Self {
        Self {
            registers,
            ip: 0,
            instructions,
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.ip + 1 < self.instructions.len() {
            let opcode = self.instructions[self.ip];
            let operand = self.instructions[self.ip + 1];
            let instruction: OpCode = (opcode, operand).try_into().unwrap();
            self.ip += 2;
            self.run_instruction(&instruction);
        }
    }

    fn run_instruction(&mut self, instruction: &OpCode) {
        match instruction {
            OpCode::ADV(operand) => {
                let result = self.registers[0] >> self.resolve_combo(operand);
                self.registers[0] = result;
            }
            OpCode::BDV(operand) => {
                let result = self.registers[0] >> self.resolve_combo(operand);
                self.registers[1] = result;
            }
            OpCode::CDV(operand) => {
                let result = self.registers[0] >> self.resolve_combo(operand);
                self.registers[2] = result;
            }
            OpCode::BXL(operand) => {
                let result = self.registers[1] ^ u32::from(*operand);
                self.registers[1] = result;
            }
            OpCode::BST(operand) => {
                let result = self.resolve_combo(operand) & 0b111;
                self.registers[1] = result;
            }
            OpCode::JNZ(operand) => {
                if self.registers[0] != 0 {
                    self.ip = usize::from(*operand);
                }
            }
            OpCode::BXC => {
                let result = self.registers[1] ^ self.registers[2];
                self.registers[1] = result;
            }
            OpCode::OUT(operand) => {
                let result = self.resolve_combo(operand) & 0b111;
                self.output.push(result as u8);
            }
        }
    }

    fn resolve_combo(&self, combo: &ComboOperand) -> u32 {
        match combo {
            ComboOperand::Literal(v) => (*v).into(),
            ComboOperand::Register(i) => self.registers[*i],
        }
    }

    fn get_output(&self) -> String {
        self.output.iter().join(",")
    }
}

// This assumes there is only one jump instruction at the end :/
struct Z3ExecutionContext<'a, 'z> {
    ctx: &'z Context,
    solver: Optimize<'z>,

    start_a: BV<'z>,
    registers: [BV<'z>; 3],
    ip: usize,
    instructions: &'a [u8],
    output_size: usize,
}

impl<'a, 'z> Z3ExecutionContext<'a, 'z> {
    const INT_LEN: u32 = 64;

    fn new(ctx: &'z Context, registers: [u32; 3], instructions: &'a [u8]) -> Self {
        let solver = Optimize::new(ctx);
        let start_a = BV::new_const(ctx, "a", Self::INT_LEN);

        let registers = [
            start_a.clone(),
            BV::from_u64(ctx, registers[1].into(), Self::INT_LEN),
            BV::from_u64(ctx, registers[2].into(), Self::INT_LEN),
        ];
        Self {
            ctx,
            solver,
            start_a,
            registers,
            ip: 0,
            instructions,
            output_size: 0,
        }
    }

    fn run(&mut self) {
        while self.ip + 1 < self.instructions.len() {
            let opcode = self.instructions[self.ip];
            let operand = self.instructions[self.ip + 1];
            let instruction: OpCode = (opcode, operand).try_into().unwrap();
            self.ip += 2;
            self.run_instruction(&instruction);
        }
    }

    fn run_instruction(&mut self, instruction: &OpCode) {
        match instruction {
            OpCode::ADV(operand) => {
                let result = self.registers[0].bvlshr(&self.resolve_combo(operand));
                self.registers[0] = result;
            }
            OpCode::BDV(operand) => {
                let result = self.registers[0].bvlshr(&self.resolve_combo(operand));
                self.registers[1] = result;
            }
            OpCode::CDV(operand) => {
                let result = self.registers[0].bvlshr(&self.resolve_combo(operand));
                self.registers[2] = result;
            }
            OpCode::BXL(operand) => {
                let result = &self.registers[1] ^ u64::from(*operand);
                self.registers[1] = result;
            }
            OpCode::BST(operand) => {
                let result = self.resolve_combo(operand) & 0b111u64;
                self.registers[1] = result;
            }
            OpCode::JNZ(operand) => {
                if self.output_size < self.instructions.len() {
                    self.ip = usize::from(*operand);
                } else {
                    self.solver.assert(&self.registers[0]._eq(&BV::from_u64(
                        self.ctx,
                        0,
                        Self::INT_LEN,
                    )));
                }
            }
            OpCode::BXC => {
                let result = &self.registers[1] ^ &self.registers[2];
                self.registers[1] = result;
            }
            OpCode::OUT(operand) => {
                let result = self.resolve_combo(operand) & 0b111u64;
                self.solver.assert(&result._eq(&BV::from_u64(
                    self.ctx,
                    self.instructions[self.output_size].into(),
                    Self::INT_LEN,
                )));
                self.output_size += 1;
            }
        }
    }

    fn resolve_combo(&self, combo: &ComboOperand) -> BV<'z> {
        match combo {
            ComboOperand::Literal(v) => BV::from_u64(self.ctx, (*v).into(), Self::INT_LEN),
            ComboOperand::Register(i) => self.registers[*i].clone(),
        }
    }

    fn get_a(&self) -> u64 {
        self.solver.minimize(&self.start_a);
        assert_eq!(self.solver.check(&[]), SatResult::Sat);
        let model = self.solver.get_model().unwrap();
        let result = model.eval(&self.start_a, true).unwrap();
        result.as_u64().unwrap()
    }
}

pub struct AocDay17 {
    registers: [u32; 3],
    instructions: Vec<u8>,
}

impl AocDay<String, u64> for AocDay17 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let registers = [lines.next(), lines.next(), lines.next()].try_map(|line| {
            let line = line.ok_or(DayError::GenericParseErr(
                "input does not contain 3 registers",
            ))?;

            Ok::<_, DayError>(line[("Register X: ".len())..].parse()?)
        })?;

        let instructions = lines
            .nth(1)
            .map(|line| {
                line[("Program: ").len()..]
                    .split(',')
                    .map(|v| v.parse())
                    .process_results(|it| it.collect_vec())
            })
            .ok_or(DayError::GenericParseErr("input does not have a program"))??;

        Ok(AocDay17 {
            registers,
            instructions,
        })
    }
    fn part1(&self) -> String {
        let mut execution_context = ExecutionContext::new(self.registers, &self.instructions);
        execution_context.run();

        execution_context.get_output()
    }
    fn part2(&self) -> u64 {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let mut execution_context =
            Z3ExecutionContext::new(&ctx, self.registers, &self.instructions);
        execution_context.run();

        execution_context.get_a()
    }
}

#[cfg(test)]
mod day17tests {
    use super::*;

    const INPUT: &[&str] = &[
        "Register A: 729",
        "Register B: 0",
        "Register C: 0",
        "",
        "Program: 0,1,5,4,3,0",
    ];

    const INPUT2: &[&str] = &[
        "Register A: 2024",
        "Register B: 0",
        "Register C: 0",
        "",
        "Program: 0,3,5,4,3,0",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay17::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay17::preprocessing_tests(INPUT2)?;
        assert_eq!(day.part2(), 117440);
        Ok(())
    }
}
