extern crate sexp;

pub struct Init {
    pub side: char,
    pub number: i64
}

impl Init {
    pub fn build(string: String) -> Init {
        let mut side = 'l';
        let mut number = 1;
        let tree = sexp::parse(&string).unwrap();
        if let sexp::Sexp::List(elements) = tree {
            if let sexp::Sexp::Atom(sexp::Atom::S(ref side_str)) = elements[1] {
                side = side_str.chars().nth(0).unwrap();
            }
            if let sexp::Sexp::Atom(sexp::Atom::I(value)) = elements[2] {
                number = value;
            }
        }
        Init { side, number }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_init_build() {
        let init = Init::build(String::from("(init l 1 before_kick_off)"));
        println!("{} {}", init.side, init.number);
        assert_eq!(1, 1);
    }

}