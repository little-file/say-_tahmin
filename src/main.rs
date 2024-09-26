use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mtahmin = rand::thread_rng().gen_range(1..=100);

    println!("merhabalar sayı tahmin oyununa hoş geldiniz.");
    loop {
        println!("lütfen bir tahmin giriniz: ");

    let mut x = String::new();

    io::stdin()
        .read_line(&mut x);
    let x: u32 = match x.trim().parse() { 
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("tahmin ettiğiniz sayı {x}");

    match x.cmp(&mtahmin) {
        Ordering::Less => println!("çok az"),
        Ordering::Greater => println!("çok büyük"),
        Ordering::Equal => {
            println!("sen kazandın");
            break;
        }
    }
    }
}