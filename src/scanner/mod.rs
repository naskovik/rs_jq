// also general string utilities

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

    pub fn split_by(s: &str, separator: char) -> Option<Vec<String>> {
        match s.find(separator) {
            None => None,
            Some(_) => {
                let result = s.split(separator)
                    .filter(|el| !el.is_empty())
                    .map(|el| el.trim().to_string())
                    .collect();
                Some(result)
            }
        }
    }    

}

#[cfg(test)]
mod tests {

    use super::Scanner;

    #[test]
    fn test_split_by() {
        let example1 = "object: foo.baz.cdu";
        assert_eq!(
            Scanner::split_by(example1, ':'),
            Some(vec!["object".to_string(), "foo.baz.cdu".to_string()])
        );
    }

    
    #[test]
    fn test_parse_pair() {
        assert_eq!(
            Scanner::parse_pair::<String>("cdu,quino", ','),
            Some(("cdu".to_string(), "quino".to_string()))
        );

        assert_eq!(
            Scanner::parse_pair::<String>("cdu:quino", ':'),
            Some(("cdu".to_string(), "quino".to_string()))
        );


        assert_eq!(
            Scanner::parse_pair::<String>("cdu:quino", ','),
            None
        );


        assert_eq!(
            Scanner::parse_pair::<String>("cdu:quino,nazar", ','),
            Some(("cdu:quino".to_string(), "nazar".to_string()))
        );

        assert_eq!(
            Scanner::parse_pair::<String>("cdu:quino.nazar", ':'),
            Some(("cdu".to_string(), "quino.nazar".to_string()))
        );

        
        assert_eq!(
            Scanner::parse_pair::<usize>("1:2", ':'),
            Some((1, 2))
        );

        
        assert_eq!(
            Scanner::parse_pair::<usize>("1:2", ','),
            None
        );

         assert_eq!(
            Scanner::parse_pair::<usize>("1:2", ','),
            None
        );

        assert_eq!(
            Scanner::parse_pair::<usize>("12", ','),
            None
        );

    }

    #[test]
    fn test_take_until() {
        let mut scanner = Scanner::new("aasdfasd%{&=9823}");
        assert_eq!(
            scanner.take_until('%'),
            Some("aasdfasd".to_string())
        );

        assert_eq!(scanner.take_until('}'), Some("%{&=9823".to_string()));

        assert_eq!(scanner.peek(), Some('}').as_ref());

    }

}
