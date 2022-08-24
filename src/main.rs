use core::num;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hallo und herzlich Wilkommen bei Zahlenraten! Es wird nun eine Zufallszahl zwischen 1 und 125 erzeugt. Du musst versuchen, die Zahl zu erraten. Viel Spaß!");
    let mut randomnumber = rand::thread_rng().gen_range(1..=10);
    let mut attempts = 0;
    loop {
        println!("Gib deine Schätzung ein:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Fehler: Zeile konnte nicht gelesen werden.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bitte gib eine Zahl als Schätzung ein!");
                continue;
            }
        };
        println!("Deine Schätzung war: {guess}");
        match guess.cmp(&mut randomnumber) {
            Ordering::Less => {
                attempts = attempts +1;
                println!("Deine Schätzung war zu klein! Das war dein {attempts}. Versuch!");
            }
            Ordering::Equal => {
                attempts = attempts +1;
                println!("Du hast richtig geschätzt und für die richtige Lösung {attempts} Versuche gebraucht!");
                break;
            }
            Ordering::Greater => {
                attempts = attempts +1;
                println!("Deine Schätzung war zu groß! Das war dein {attempts}. Versuch!");
            }
        }
    }
}