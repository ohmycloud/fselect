#[derive(Clone, PartialEq, Debug)]
pub enum Lexem {
    RawString(String),
    Comma,
    From,
    Where,
    Operator(String),
    String(String),
    Open,
    Close,
    And,
    Or,
    Order,
    By,
    DescendingOrder,
    Limit,
    Into,
}

#[derive(Debug)]
enum LexingMode {
    Undefined,
    RawString,
    Comma,
    Operator,
    String,
    Open,
    Close,
}

pub struct Lexer<'a> {
    input: &'a str,
    index: usize
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        return Lexer { input, index: 0 }
    }

    pub fn next_lexem(&mut self) -> Option<Lexem> {
        let mut s = String::new();
        let mut mode = LexingMode::Undefined;

        for c in self.input.chars().skip(self.index) {
            let stop = match mode {
                LexingMode::Undefined => {
                    self.index += 1;

                    match c {
                        ' ' => {},
                        '\'' =>  mode = LexingMode::String,
                        ',' =>  mode = LexingMode::Comma,
                        '(' => mode = LexingMode::Open,
                        ')' => mode = LexingMode::Close,
                        _ => {
                            if is_op_char(c) {
                                mode = LexingMode::Operator;
                            } else {
                                mode = LexingMode::RawString;
                            }
                            s.push(c);
                        }
                    }

                    false
                },
                LexingMode::String => {
                    self.index += 1;

                    if c == '\'' {
                        true
                    } else {
                        mode = LexingMode::String;
                        s.push(c);

                        false
                    }
                },
                LexingMode::Operator => {
                    self.index += 1;

                    if is_op_char(c) {
                        mode = LexingMode::Operator;
                        s.push(c);

                        false
                    } else {
                        true
                    }
                },
                LexingMode::RawString => {
                    if c == ' ' || c == ',' || c == ')' {
                        true
                    } else {
                        self.index += 1;
                        mode = LexingMode::RawString;
                        s.push(c);

                        false
                    }
                },
                LexingMode::Comma | LexingMode::Open | LexingMode::Close => {
                    true
                },
            };

            if stop {
                break;
            }
        }

        match mode {
            LexingMode::String => Some(Lexem::String(s)),
            LexingMode::Operator => Some(Lexem::Operator(s)),
            LexingMode::RawString => {
                match s.to_lowercase().as_str() {
                    "from" => Some(Lexem::From),
                    "where" => Some(Lexem::Where),
                    "or" => Some(Lexem::Or),
                    "and" => Some(Lexem::And),
                    "order" => Some(Lexem::Order),
                    "by" => Some(Lexem::By),
                    "asc" => self.next_lexem(),
                    "desc" => Some(Lexem::DescendingOrder),
                    "limit" => Some(Lexem::Limit),
                    "into" => Some(Lexem::Into),
                    "eq" | "ne" | "gt" | "lt" | "ge" | "le" | "gte" | "lte" | 
                    "regexp" | "rx" | "like" => Some(Lexem::Operator(s)),
                    _ => Some(Lexem::RawString(s)),
                }
            },
            LexingMode::Comma => Some(Lexem::Comma),
            LexingMode::Open => Some(Lexem::Open),
            LexingMode::Close => Some(Lexem::Close),
            _ => None
        }
    }
}

fn is_op_char(c: char) -> bool {
    match c {
        '=' | '!' | '<' | '>' | '~' => true,
        _ => false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexems() {
        let mut lexer = Lexer::new("select name, path ,size , fsize from /test depth 2, /test2 archives,/test3 depth 3 archives , /test4 ,'/test5' where name != 123 AND ( size gt 456 or fsize lte 758) or name = 'xxx' order by 1 ,3 desc , path asc limit 50");

        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("select"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("name"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Comma));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("path"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Comma));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("size"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Comma));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("fsize"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::From));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("/test"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("depth"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("2"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Comma));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("/test2"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("archives"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Comma));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("/test3"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("depth"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("3"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("archives"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Comma));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("/test4"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Comma));
        assert_eq!(lexer.next_lexem(), Some(Lexem::String(String::from("/test5"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Where));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("name"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Operator(String::from("!="))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("123"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::And));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Open));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("size"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Operator(String::from("gt"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("456"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Or));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("fsize"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Operator(String::from("lte"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("758"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Close));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Or));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("name"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Operator(String::from("="))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::String(String::from("xxx"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Order));
        assert_eq!(lexer.next_lexem(), Some(Lexem::By));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("1"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Comma));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("3"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::DescendingOrder));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Comma));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("path"))));
        assert_eq!(lexer.next_lexem(), Some(Lexem::Limit));
        assert_eq!(lexer.next_lexem(), Some(Lexem::RawString(String::from("50"))));
    }

}
