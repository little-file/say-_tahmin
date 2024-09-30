use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mtahmin = rand::thread_rng().gen_range(1..=100);

    println!("merhabalar sayı tahmin oyununa hoş geldiniz.");
    loop {
        println!("lütfen bir tahmin giriniz: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess);
    let guess: u32 = match guess.trim().parse() { 
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("tahmin ettiğiniz sayı {guess}");

    match guess.cmp(&mtahmin) {
        Ordering::Less => println!("çok az"),
        Ordering::Greater => println!("çok büyük"),
        Ordering::Equal => {
            println!("sen kazandın");
            break;
        }
    }
    }
}