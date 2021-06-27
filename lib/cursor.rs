// This struct could also be named Stream.
// It is inspired both by the Cursor from rustc_lexer and by Stream from xmlparser.
#[derive(Clone)]
pub struct Cursor<'input> {
    input: &'input str,
    // The char position
    pos: usize,
    // The byte position
    byte_pos: usize,
}

impl<'input> Cursor<'input> {
    pub fn new(input: &str) -> Cursor {
        Cursor {
            input: input,
            pos: 0,
            byte_pos: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn byte_offset(&self) -> usize {
        self.byte_pos
    }

    pub fn input(&self) -> &'input str {
        self.input
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.input[self.byte_pos..].chars().next();

        if let Some(c) = c {
            self.pos += 1;
            self.byte_pos += c.len_utf8();
        }

        c
    }

    pub fn nth(&self, n: usize) -> Option<char> {
        self.input[self.byte_pos..].chars().nth(n)
    }

    pub fn first(&self) -> Option<char> {
        self.nth(0)
    }

    pub fn second(&self) -> Option<char> {
        self.nth(1)
    }

    pub fn skip(&mut self, n: usize) {
        let chars = &mut self.input[self.byte_pos..].chars();
        for _ in 0..n {
            match chars.next() {
                None => break,
                Some(c) => {
                    self.pos += 1;
                    self.byte_pos += c.len_utf8();
                },
            }
        }
    }

    pub fn skip_while<P>(&mut self, predicate: P) where
        P: Fn(char) -> bool {
        loop {
            let c = match self.first() {
                Some(c) => c,
                None => return,
            };

            if predicate(c) {
                self.next();
            } else {
                return;
            }
        }
    }
    
    pub fn take_while<P>(&mut self, predicate: P) -> &'input str where
        P: Fn(char) -> bool {

        let start = self.byte_offset();

        loop {
            let c = match self.first() {
                Some(c) => c,
                None    => return &self.input[start..self.byte_offset()], 
            };

            if predicate(c) {
                self.next();
            } else {
                return &self.input[start..self.byte_offset()];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cursor_start_at_beginning() {
        let mut c = Cursor::new("hello");
        assert_eq!(c.position(), 0);
        assert_eq!(c.next(), Some('h'));
    }

    #[test]
    fn next_consumes_one_ascii_char() {
        let mut c = Cursor::new("hello");
        c.next();
        assert_eq!(c.position(), 1);
        assert_eq!(c.next(), Some('e'));
    }

    #[test]
    fn next_consumes_one_utf8_char() {
        let mut c = Cursor::new("åäö");
        c.next();
        assert_eq!(c.position(), 1);
        assert_eq!(c.next(), Some('ä'));
    }

    #[test]
    fn next_increments_byte_pos_correctly_for_utf8() {
        let mut c = Cursor::new("åäö");
        c.next();
        assert_eq!(c.pos, 1);
        assert_eq!(c.byte_pos, 2);
    }

    #[test]
    fn nth_wont_consume() {
        let c = Cursor::new("abcdef");
        assert_eq!(c.nth(0), Some('a'));
        assert_eq!(c.nth(0), Some('a'));
    }

    #[test]
    fn skip_while_skips_whole_input_when_all_matching() {
        let mut c = Cursor::new("aaa");

        c.skip_while(|c| c == 'a');

        assert_eq!(c.next(), None);
    }

    #[test]
    fn skip_while_skips_all_matching() {
        let mut c = Cursor::new("aaabbb");

        c.skip_while(|c| c == 'a');

        assert_eq!(c.next(), Some('b'));
    }

    #[test]
    fn take_while_for_empty_string() {
        let mut c = Cursor::new("");

        let s = c.take_while(|c| c == 'a');

        assert_eq!(s, "");
    }

    #[test]
    fn take_while_takes_whole_input_when_all_matching() {
        let mut c = Cursor::new("aaa");
        
        let s = c.take_while(|c| c == 'a');

        assert_eq!(s, "aaa");
        assert_eq!(c.next(), None);
    }

    #[test]
    fn take_while_takes_all_matching() {
        let mut c = Cursor::new("aaabbb");

        let s = c.take_while(|c| c == 'a');

        assert_eq!(s, "aaa");
        assert_eq!(c.next(), Some('b'));
    }
}
