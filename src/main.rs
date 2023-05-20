use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Угадайте число!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Секретное число равно {}", secret_number);
    println!("Пожалуйста, введите свою догадку.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Не получилось прочитать строку");
    println!("Вы загадали: {}", guess);

    let guess: u32 = guess.trim().parse()
           .expect("Пожалуйста, наберите число!");

           match guess.cmp(&secret_number) {
        Ordering::Less => println!("Слишком малое число!"),
        Ordering::Greater => println!("Слишком большое число!"),
        Ordering::Equal => println!("Вы выиграли!"),
    }
}
