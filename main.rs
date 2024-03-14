use std::io;
//io = input output biblotechka

fn main() {
    loop{
    let mut name = String::new();
    let mut age = String::new();
    
    println!("Введите имя девушки: ");
    io::stdin().read_line(&mut name).expect("err");
    println!("Введите возраст девушки: ");
    io::stdin().read_line(&mut age).expect("err");
    let age: u32 = age.trim().parse().expect("chislo pls");

    let girlfriend = if age >= 18 { "Да" } else { "ст. 134-135 УК РФ" };
   
    println!("");
    println!("My gf <3, {}", name.trim());
    println!("сексить сексить?, {}", girlfriend);
    println!("");

println!("пака пака? (net/paka)");
let mut answer = String::new();
io::stdin().read_line(&mut answer).expect("err x3 ror");
let answer = answer.trim().to_lowercase();

if answer != "net"
{
    break;
} 

    }
}

