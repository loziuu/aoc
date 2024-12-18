use std::fs::read_to_string;

struct Computer {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    ip: usize,
}

impl Computer {
    fn new(reg_a: i32, reg_b: i32, reg_c: i32) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            ip: 0,
        }
    }

    fn adv(&mut self, operand: u8) {
        self.reg_a = (self.reg_a / 2i32.pow(self.combo_operator(operand) as u32)) as i32;
        self.bump_ip();
    }

    fn bxl(&mut self, operand: u8) {
        self.reg_b = self.reg_b ^ operand as i32;
        self.bump_ip();
    }

    fn bst(&mut self, operand: u8) {
        self.reg_b = self.combo_operator(operand) % 8;
        self.bump_ip();
    }

    fn jnz(&mut self, operand: u8) {
        if self.reg_a != 0 {
            self.ip = operand as usize;
        } else {
            self.bump_ip();
        }
    }

    fn bxc(&mut self, _: u8) {
        self.reg_b = self.reg_b ^ self.reg_c;
        self.bump_ip();
    }

    fn out(&mut self, operand: u8) -> i32 {
        let res = self.combo_operator(operand) % 8;
        self.bump_ip();
        res
    }

    fn bdv(&mut self, operand: u8) {
        self.reg_b = (self.reg_a / 2i32.pow(self.combo_operator(operand) as u32)) as i32;
        self.bump_ip();
    }

    fn cdv(&mut self, operand: u8) {
        self.reg_c = (self.reg_a / 2i32.pow(self.combo_operator(operand) as u32)) as i32;
        self.bump_ip();
    }

    fn bump_ip(&mut self) {
        self.ip += 2;
    }

    fn combo_operator(&self, operand: u8) -> i32 {
        match operand {
            0..=3 => operand as i32,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid operand!"),
        }
    }

    fn run(&mut self, program: Vec<i32>) -> String {
        let mut out = Vec::new();

        while self.ip < program.len() {
            match program[self.ip] {
                0 => {
                    self.adv(program[self.ip + 1] as u8);
                }
                1 => {
                    self.bxl(program[self.ip + 1] as u8);
                }
                2 => {
                    self.bst(program[self.ip + 1] as u8);
                }
                3 => {
                    self.jnz(program[self.ip + 1] as u8);
                }
                4 => {
                    self.bxc(program[self.ip + 1] as u8);
                }
                5 => {
                    out.push(self.out(program[self.ip + 1] as u8));
                }
                6 => {
                    self.bdv(program[self.ip + 1] as u8);
                }
                7 => {
                    self.cdv(program[self.ip + 1] as u8);
                }
                _ => panic!("Invalid opcode"),
            };
        }

        out.iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

pub fn execute(file_path: &str) -> String {
    let input = read_to_string(file_path).unwrap();

    let mut lines = input.split('\n');

    let a = lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()[2]
        .parse::<i32>()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()[2]
        .parse::<i32>()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()[2]
        .parse::<i32>()
        .unwrap();
    lines.next().unwrap();
    let program = lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()[1]
        .split(',')
        .map(|it| it.parse::<i32>().unwrap())
        .collect();

    let mut computer = Computer::new(a, b, c);
    computer.run(program)
}

#[cfg(test)]
mod tests {
    use super::Computer;

    #[test]
    fn first() {
        let mut cpu = Computer::new(729, 0, 0);
        let result = cpu.run(vec![0, 1, 5, 4, 3, 0]);

        assert_eq!("4,6,3,5,6,3,5,2,1,0", result.as_str());
    }
}
