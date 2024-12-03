use std::{
    fs::File,
    io::{BufReader, Read},
};

const BUFFER_SIZE: usize = 1024;

struct Reader {
    buffer: [u8; BUFFER_SIZE],
    fd: BufReader<File>,
    pointer: usize,
    loaded: usize,
    process: bool,
}

impl Reader {
    pub fn from_file(file: File) -> Reader {
        Self {
            buffer: [0; BUFFER_SIZE],
            fd: BufReader::new(file),
            pointer: 0,
            loaded: 0,
            process: true,
        }
    }

    pub fn get_next(&mut self) -> Option<(i32, i32)> {
        while let Some(p) = self.get_and_bump_pointer() {
            if self.buffer[p] == 'm' as u8 && self.process {
                let val = self.mul();
                if val.is_some() {
                    return val;
                }
            }

            if self.buffer[p] == 'd' as u8 {
                self.do_or_dont();
            }
        }

        None
    }

    fn do_or_dont(&mut self) -> Option<()> {
        if !self.expect_bool('o' as u8)? {
            return None;
        }
        self.get_and_bump_pointer();

        if self.expect_bool('(' as u8)? {
            self.get_and_bump_pointer();
            if self.expect_bool(')' as u8)? {
                self.process = true;
            }
        }

        if self.expect_bool('n' as u8)? {
            self.get_and_bump_pointer();
            if self.expect_bool('\'' as u8)? {
                self.get_and_bump_pointer();
                if self.expect_bool('t' as u8)? {
                    self.get_and_bump_pointer();
                    if self.expect_bool('(' as u8)? {
                        self.get_and_bump_pointer();
                        if self.expect_bool(')' as u8)? {
                            self.process = false;
                        }
                    }
                }
            }
            return None;
        }

        None
    }

    fn mul(&mut self) -> Option<(i32, i32)> {
        //let mut deq = vec!['(' as u8, 'l' as u8, 'u' as u8];
        let v = vec!['u' as u8, 'l' as u8, '(' as u8];

        let mut i = 0;

        while i < v.len() {
            let p = self.get_and_bump_pointer()?;

            let expected = v[i];
            if self.buffer[p] != expected {
                return None;
            }
            i += 1;
        }

        if i != v.len() {
            return None;
        }

        let first = self.get_number()?;
        self.expect(',' as u8);
        self.get_and_bump_pointer()?;
        let second = self.get_number()?;
        self.expect(')' as u8)?;

        Some((first, second))
    }

    fn get_number(&mut self) -> Option<i32> {
        let mut number: String = "".to_owned();

        while let Some(p) = self.get_pointer() {
            let v = self.buffer[p];
            if v >= '0' as u8 && v <= '9' as u8 {
                number = format!("{}{}", number, v as char);
            } else {
                break;
            }
            self.get_and_bump_pointer()?;
        }

        if number.is_empty() {
            return None;
        }

        Some(number.as_str().parse().unwrap())
    }

    fn expect(&mut self, expected: u8) -> Option<()> {
        let got = self.buffer[self.get_pointer()?];
        if got != expected {
            return None;
        }
        Some(())
    }

    fn expect_bool(&mut self, expected: u8) -> Option<bool> {
        let got = self.buffer[self.get_pointer()?];
        if got != expected {
            return Some(false);
        }
        return Some(true);
    }

    fn get_pointer(&mut self) -> Option<usize> {
        if self.pointer >= self.loaded {
            match self.fd.read(&mut self.buffer) {
                Ok(buf) => {
                    if buf == 0 {
                        return None;
                    }
                    self.loaded = buf;
                    self.pointer = 0;
                }
                Err(_) => return None,
            }
        }
        Some(self.pointer)
    }

    fn get_and_bump_pointer(&mut self) -> Option<usize> {
        let val = self.get_pointer();

        self.pointer += 1;
        val
    }
}

pub fn execute(file: File) -> i64 {
    let mut reader = Reader::from_file(file);
    let mut res: i64 = 0;

    while let Some(pair) = reader.get_next() {
        res += pair.0 as i64 * pair.1 as i64;
    }

    res
}
