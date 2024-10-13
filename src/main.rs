use std::io;

fn main() {


    println!("Willkommen zum der Taschenrechner app!");

    println!("Gib deine erste Zahl ein:");
    let mut input1 = String::new();
        io::stdin()
            .read_line(&mut input1)
            .expect("Fehler beim Lesen der Eingabe");

    let input1: f64 = input1.trim().parse().expect("Bitte gib eine gültige Zahl ein");
    println!("Gib den operator ein:");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Fehler beim Lesen der Eingabe");


    println!("Gib deine zweite Zahl ein:");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Fehler beim Lesen der Eingabe");
    let input2: f64 = input2.trim().parse().expect("Bitte gib eine gültige Zahl ein");

    match operator.as_str().trim() {
        "+" => { println!("{}",input1+input2)}
        "-" => { println!("{}",input1-input2)}
        "*" => { println!("{}",input1*input2)}
        "/" => { println!("{}",input1/input2)}
        _ =>{}
    }


}
