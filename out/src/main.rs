#![allow(unused)]

use conum::Num;
fn main() {
    let mut fact: Num<18> = Num::zero();
    let mut n: Num<18> = Num::zero();
    let mut i: Num<18> = Num::zero();
    let mut ist = String::from("000000000000000000");
    let mut factst = String::from("000000000000000000");

    n = 16.into();
    i = 0.into();
    fact = 1.into();
    loop {
        if i > n {
            break;
        }
        ist = i.to_string();
        factst = fact.to_zeroed_string();
        println!(
            "{}{}{}",
            format!("{}", ist),
            format!("{}", "! = "),
            format!("{}", factst)
        );
        i += 1;
        fact *= &i;
    }
}
