use std::num::ParseIntError;
use std::str::FromStr;

pub struct BitStack {
    bits: u64,
    stack: Vec<u64>,
}

impl FromStr for BitStack {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Self {
            bits: 0,
            stack: vec![],
        };
        for i in 0..s.len() - 1 {
            res.push(u64::from_str_radix(&s[i..=i], 16)?, 4);
        }
        Ok(res)
    }
}

#[allow(dead_code)]
impl BitStack {
    fn push(&mut self, bits: u64, num: u64) {
        let pos = self.bits as usize / 64;
        if pos >= self.stack.len() {
            self.stack.push(0)
        }
        self.stack[pos] |= (bits & (u64::MAX >> 64 - num)) << (60 - (self.bits % 64));
        self.bits += num;
    }

    fn print(&self) {
        for i in self.stack.iter() {
            print!("{:#b}", i);
        }
        println!();
    }

    pub fn pop(&mut self, bits: u8) -> u64 {
        if self.stack.len() == 0 {
            return 0;
        }

        let output = self.stack[0] >> (64 - bits);

        for i in 0..self.stack.len() {
            self.stack[i] = (self.stack[i] << bits) & u64::MAX;
            if i < self.stack.len() - 1 {
                self.stack[i] |= self.stack[i + 1] >> (64 - bits);
            }
        }

        if self.stack[self.stack.len() - 1] == 0 {
            self.stack.remove(self.stack.len() - 1);
        }

        output
    }
}
