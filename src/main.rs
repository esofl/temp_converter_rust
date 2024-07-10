use std::io;
const C: f32 = 32.0;
fn c_to_f(celsius_temp: f32) -> f32 {
    (celsius_temp * (9.0 / 5.0)) + C
}
fn f_to_c(fahrenheit_temp: f32) -> f32 {
    (fahrenheit_temp - C) * (5.0 / 9.0)
}

fn convert(temp: f32, choice: u8) -> Option<f32> {
    match choice {
        1 => Some(c_to_f(temp)),
        2 => Some(f_to_c(temp)),
        _ => None,
    }
}


fn main() {
    loop {
        println!("Выберете операцию:\n 1) C => F\n 2) F => C");
        let mut user_choice = String::new();

        io::stdin().read_line(&mut user_choice).unwrap();
        let n_choice = user_choice
            .trim()
            .parse::<u8>()
            .expect("Пожалуйста введите номер операции");
        println!("Ведите значение:");

        let mut temp = String::new();
        io::stdin().read_line(&mut temp).unwrap();
        let temp = temp
            .trim()
            .parse::<f32>()
            .expect("Пожалуйста введите номер операции");

        match convert(temp, n_choice) {
            Some(result) => println!("Результат: {result}\n"),
            None => println!("Даже хз че сказать\n")
        };
    }
}