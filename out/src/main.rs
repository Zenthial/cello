fn main() {
    let mut n;
    let mut i;
    let mut fact;

    n = 16;
    i = 0;
    fact = 1;
    fact += 1;
    n += i;
    println!(
        "{}{}{}",
        format!("{}", n),
        format!("{}", " != "),
        format!("{}", fact)
    );
}
