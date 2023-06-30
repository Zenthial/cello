use rug::Integer;

fn main() {
let mut fact: Integer = 111111111111111.into();
let mut n: Integer = 111.into();
let mut i: Integer = 111.into();
let mut ist= String::from("000");
let mut factst= String::from("000000000000000000");

n = 16;
i = 0;
fact = 1;
loop {
if i > n{
break;}ist = i;
factst = fact;
println!("{}{}{}", format!("{}", ist), format!("{}", "! = "), format!("{}", factst));
i += 1;
fact *= i;
}
}