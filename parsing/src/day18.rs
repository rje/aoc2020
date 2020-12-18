use crate::util::get_lines;
extern crate peg;

peg::parser!( grammar arithmetic() for str {
    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub(crate) rule calculate() -> i64 = precedence!{
        x:(@) " + " y:@ { x + y }
        x:(@) " * " y:@ { x * y }
        --
        "(" v:calculate() ")" { v }
        n:number() {n}
    }
});

peg::parser!( grammar arithmetic_pt2() for str {
    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub(crate) rule calculate() -> i64 = precedence!{
        x:(@) " * " y:@ { x * y }
        --
        x:(@) " + " y:@ { x + y }
        --
        "(" v:calculate() ")" { v }
        n:number() {n}
    }
});

pub fn parse(path: &str) -> Vec<i64> {
    get_lines(path)
        .map(|line| arithmetic::calculate(&line).unwrap())
        .collect()
}

pub fn parse_pt2(path: &str) -> Vec<i64> {
    get_lines(path)
        .map(|line| arithmetic_pt2::calculate(&line).unwrap())
        .collect()
}
