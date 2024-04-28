use std::str::FromStr;

pub struct Scanner {
    cursor: usize,
    characters: Vec<char>
}

impl Scanner {

    pub fn new(str: &str) -> Self {
        Self {
            cursor: 0,
            characters: str.chars().collect()
        }
    }

    #[allow(dead_code)]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    #[allow(dead_code)]
    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<char> {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;
                Some(*character)
            },
            None => None
            
        }
    }

    pub fn take(&mut self, target: char) -> bool {
        match self.characters.get(self.cursor) {
            Some(character) => {
                if *character == target {
                    self.cursor += 1;
                    true

                }
                else {
                    false
                }
            },
            None => false
        }
    } 

    #[allow(dead_code)]
    pub fn take_until(&mut self, target: char) -> Option<String> {
        let mut result = String::new();
        while !self.is_done() &&
            *self.peek().unwrap_or(&target) != target {

            match self.pop() {
                None => {},
                Some(char) => {
                    result.push(char);
                }
            }

        }

        match result.len() {
            0 => None,
            _ => Some(result)
        }
    }

    pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
        match s.find(separator) {
            None => None,
            Some(index) => {
                let maybe_l = T::from_str(&s[..index]);
                let maybe_r = T::from_str(&s[index + 1..]);

                match (maybe_l, maybe_r) {
                    (Ok(l), Ok(r)) => Some((l, r)),
                    _ => None,
                }
            }
        }
    }

}
