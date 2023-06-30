use rug::{Assign, Integer};

fn main() {
    let mut fact: Integer = Integer::new();
    fact.assign(Integer::parse("111111111111111").unwrap());
    let mut n: Integer = Integer::new();
    n.assign(Integer::parse("111").unwrap());
    let mut i: Integer = Integer::new();
    i.assign(Integer::parse("111").unwrap());
    let mut ist = String::from("000");
    let mut factst = String::from("000000000000000000");

    n = 16.into();
    i = 0.into();
    fact = 1.into();
    loop {
        if i > n {
            break;
        }
        ist = i.to_string();
        factst = fact.to_string();
        println!(
            "{}{}{}",
            format!("{}", ist),
            format!("{}", "! = "),
            format!("{}", factst)
        );
        i += &1;
        fact *= &i;
    }
}
