use std::{collections::HashMap, str::FromStr};

use aoc_common::{AocDay, DayError};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GateOperation {
    And,
    Or,
    Xor,
}

impl FromStr for GateOperation {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(DayError::GenericParseErr("unknown operation")),
        }
    }
}

#[derive(Debug)]
struct Gate {
    left: String,
    right: String,
    operation: GateOperation,
}

impl FromStr for Gate {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let left = parts
            .next()
            .ok_or(DayError::GenericParseErr("can't get left input"))?
            .to_owned();
        let operation = parts
            .next()
            .ok_or(DayError::GenericParseErr("can't get operation"))?
            .parse()?;
        let right = parts
            .next()
            .ok_or(DayError::GenericParseErr("can't get right input"))?
            .to_owned();

        Ok(Self {
            left,
            right,
            operation,
        })
    }
}

impl Gate {
    fn is_equivalent(&self, operation: GateOperation, left: &str, right: &str) -> bool {
        self.operation == operation
            && ((self.left == left && self.right == right)
                || (self.left == right && self.right == left))
    }
}

struct ExecuteContext<'a> {
    values: HashMap<String, bool>,
    gates: &'a HashMap<String, Gate>,
}

impl<'a> ExecuteContext<'a> {
    fn calculate(&mut self, input: &str) -> bool {
        if let Some(value) = self.values.get(input) {
            *value
        } else {
            let gate = self
                .gates
                .get(input)
                .expect("input has no default value and no gate that it can be derived from");

            let left = self.calculate(&gate.left);
            let right = self.calculate(&gate.right);
            let result = match gate.operation {
                GateOperation::And => left && right,
                GateOperation::Or => left || right,
                GateOperation::Xor => left != right,
            };

            self.values.insert(input.to_string(), result);
            result
        }
    }
}

fn find_incorrect(gates: &HashMap<String, Gate>, max_z: u8) -> Vec<String> {
    let mut incorrect = Vec::new();
    let max_z_name = format!("z{:02}", max_z);
    gates.iter().for_each(|(out, gate)| {
        // find gates incorrectly connected to output
        if out.starts_with('z')
            && ((gate.operation != GateOperation::Xor && *out != max_z_name)
                || (gate.operation != GateOperation::Or && *out == max_z_name))
        {
            incorrect.push(out.to_string());
        }

        // find xor gates connected to inputs that its
        // output doesn't lead to an xor gate (except for first bit)
        if gate.operation == GateOperation::Xor
            && gate.left.starts_with(['x', 'y'])
            && gate.right.starts_with(['x', 'y'])
            && !gate.is_equivalent(GateOperation::Xor, "x00", "y00")
            && find_gate_partial(gates, GateOperation::Xor, out).is_none()
        {
            incorrect.push(out.to_string());
        }

        // find xor gates that are not connected to inputs or outputs
        if gate.operation == GateOperation::Xor
            && !out.starts_with('z')
            && !gate.left.starts_with(['x', 'y'])
            && !gate.right.starts_with(['x', 'y'])
        {
            incorrect.push(out.to_string());
        }

        // find and gates whose output is not connected to an or gate
        if gate.operation == GateOperation::And
            && !gate.is_equivalent(GateOperation::And, "x00", "y00")
            && find_gate_partial(gates, GateOperation::Or, out).is_none()
        {
            incorrect.push(out.to_string());
        }
    });

    incorrect
}

fn find_gate_partial<'a>(
    gates: &'a HashMap<String, Gate>,
    operation: GateOperation,
    operand: &str,
) -> Option<&'a str> {
    gates
        .iter()
        .find(|(_, gate)| {
            gate.operation == operation && (gate.left == operand || gate.right == operand)
        })
        .map(|(out, _)| out.as_str())
}

pub struct AocDay24 {
    start_values: HashMap<String, bool>,
    gates: HashMap<String, Gate>,
}

impl AocDay<usize, String> for AocDay24 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let start_values = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (name, value) = line
                    .split_once(": ")
                    .ok_or(DayError::GenericParseErr("can't split on start value"))?;

                Ok::<_, DayError>((name.to_string(), value.parse::<u8>()? != 0))
            })
            .process_results(|it| it.collect())?;

        let gates = lines
            .map(|line| {
                let (gate, output) = line
                    .split_once(" -> ")
                    .ok_or(DayError::GenericParseErr("can't split on gate output"))?;

                Ok::<_, DayError>((output.to_string(), gate.parse()?))
            })
            .process_results(|it| it.collect())?;

        Ok(AocDay24 {
            start_values,
            gates,
        })
    }
    fn part1(&self) -> usize {
        let mut context = ExecuteContext {
            values: self.start_values.clone(),
            gates: &self.gates,
        };
        (0u8..)
            .map(|i| format!("z{:02}", i))
            .take_while(|input| self.gates.contains_key(input))
            .map(|input| context.calculate(&input) as usize)
            .enumerate()
            .fold(0, |acc, (i, bit)| acc | ((bit & 1) << i))
    }
    fn part2(&self) -> String {
        let max_z = (0u8..)
            .take_while(|z| self.gates.contains_key(&format!("z{:02}", z)))
            .last()
            .unwrap_or(0);

        let mut result = find_incorrect(&self.gates, max_z);
        result.sort();
        result.dedup();

        result.join(",")
    }
}

#[cfg(test)]
mod day24tests {
    use super::*;

    const INPUT: &[&str] = &[
        "x00: 1",
        "x01: 1",
        "x02: 1",
        "y00: 0",
        "y01: 1",
        "y02: 0",
        "",
        "x00 AND y00 -> z00",
        "x01 XOR y01 -> z01",
        "x02 OR y02 -> z02",
    ];

    const INPUT_LARGER: &[&str] = &[
        "x00: 1",
        "x01: 0",
        "x02: 1",
        "x03: 1",
        "x04: 0",
        "y00: 1",
        "y01: 1",
        "y02: 1",
        "y03: 1",
        "y04: 1",
        "",
        "ntg XOR fgs -> mjb",
        "y02 OR x01 -> tnw",
        "kwq OR kpj -> z05",
        "x00 OR x03 -> fst",
        "tgd XOR rvg -> z01",
        "vdt OR tnw -> bfw",
        "bfw AND frj -> z10",
        "ffh OR nrd -> bqk",
        "y00 AND y03 -> djm",
        "y03 OR y00 -> psh",
        "bqk OR frj -> z08",
        "tnw OR fst -> frj",
        "gnj AND tgd -> z11",
        "bfw XOR mjb -> z00",
        "x03 OR x00 -> vdt",
        "gnj AND wpb -> z02",
        "x04 AND y00 -> kjc",
        "djm OR pbm -> qhw",
        "nrd AND vdt -> hwm",
        "kjc AND fst -> rvg",
        "y04 OR y02 -> fgs",
        "y01 AND x02 -> pbm",
        "ntg OR kjc -> kwq",
        "psh XOR fgs -> tgd",
        "qhw XOR tgd -> z09",
        "pbm OR djm -> kpj",
        "x03 XOR y03 -> ffh",
        "x00 XOR y04 -> ntg",
        "bfw OR bqk -> z06",
        "nrd XOR fgs -> wpb",
        "frj XOR qhw -> z04",
        "bqk OR frj -> z07",
        "y03 OR x01 -> nrd",
        "hwm AND bqk -> z03",
        "tgd XOR rvg -> z12",
        "tnw OR pbm -> gnj",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay24::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 4);
        Ok(())
    }

    #[test]
    fn part1_larger() -> Result<(), DayError> {
        let day = AocDay24::preprocessing_tests(INPUT_LARGER)?;
        assert_eq!(day.part1(), 2024);
        Ok(())
    }

    // no suitable test for part 2 (not applicable to example)
}
