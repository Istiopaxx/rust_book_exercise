use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("숫자 추리하기!");

    let secret_number = rand::thread_rng().gen_range(1, 501);

    loop {
        println!("추리한 숫자를 입력해주세요. ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("읽는 데 실패함");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("당신이 추리한 것은 : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("너무..작아요."),
            Ordering::Greater => println!("너무..커요."),
            Ordering::Equal => {
                println!("당신이 이겼다!");
                break;
            }
        }
    }
}
