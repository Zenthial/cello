fn main() {
    let mut n;
    let mut i;
    let mut fact;
    let mut ist;
    let mut factst;

    n = 16;
    i = 0;
    fact = 1;
    loop {
        if i > n {
            break;
        }
        ist = i;
        factst = fact;
        println!(
            "{}{}{}",
            format!("{}", ist),
            format!("{}", "! = "),
            format!("{}", factst)
        );
        i += 1;
        fact *= i;
    }
}
