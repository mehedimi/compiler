use regex::Regex;

#[derive(Debug)]
pub struct Token<'a> {
    pub t: &'a str,
    pub value: String,
}


pub fn parse (str: String) -> Vec<Token<'static>> {
    let mut tokens: Vec<Token> = Vec::new();

    let is_char = Regex::new(r"[a-zA-Z]").unwrap();
    let is_whitespace = Regex::new(r"\s").unwrap();

    let mut current = 0;

    while current < str.len() {
        let mut c = str.chars().nth(current).unwrap().to_string();

        if is_char.is_match(c.as_str()) {
            let mut keywords = String::new();

            while is_char.is_match(c.as_str()) {
                keywords.push_str(c.as_str());
                current = current + 1;

                c = str.chars().nth(current).unwrap().to_string();
            }

            tokens.push(Token { t: "name", value: keywords });
            continue;
        }

        if is_whitespace.is_match(c.as_str()) {
            current = current + 1;
            continue;
        }

        if c.as_str() == "'" {
            current = current + 1;
            tokens.push(Token { t: "qoute", value: c.to_string() });
            continue;
        }

        panic!("Unknown char: {}", &c);
    }

    return  tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_the_string() {
        let syntax = "print 'Hello'";

        let result = parse(syntax.to_string());

        assert_eq!(4, result.capacity());
    }
}