use std::collections::HashMap;

pub enum JsonValue {
    Number (f64),
    Str (String),
    Bool (bool),
    Null,
    List (Vec<Box<JsonValue>>),
    Object (HashMap<String, Box<JsonValue>>),
}

pub fn parse(src: &str) -> (Result<JsonValue, &str>, &str) {
    let (head0, tail) = src.split_at(1);
    let head = head0.chars().nth(1).unwrap();
    match head {
        '{' => unimplemented!(),
        '[' => unimplemented!(),
        '"' => unimplemented!(),
        ' ' | '\t' | '\n' | '\r' => unimplemented!(),
        c if ('0' .. '9').contains(&c) => unimplemented!(),
        _ => (Err("invalid character"), tail),
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
