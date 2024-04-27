//use crate::prelude::*;

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

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

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

}
