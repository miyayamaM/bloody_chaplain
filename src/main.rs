#[allow(unused)]
use std::num::ParseIntError;
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    //あなたが数字を選択する
    let your_number: usize;
    loop {
        println!("1-10までの数字を選んでね");
        your_number = match input_int() {
            Ok(number) if number < 11 => number,
            _ => {
                println!("1~10までの数字を選んでください");
                continue;
            }
        };
        break;
    }
    println!("あなたは{}を選択", your_number);

    //botの初期化
    let mut bot = Bot::new();

    //数を当てる
    let answer = your_number + bot.number;
    let mut candidates: Vec<usize> = Vec::new();
    for i in 2..=20 { candidates.push(i); }

    loop {
        println!("数字をcallしてください");
        println!("以下から数字を選んでね");
        print_candidates(&candidates);

        let selected_number = match input_int() {
            Ok(number) if number <= your_number => {
                println!("選んだ数字より大きい数はcallできません");
                println!("あなたの負けです");
                break;
            },
            Ok(number) if candidates.iter().any(|candidate| *candidate == number) => number,
            _ => {
                println!("候補の中から選んでください");
                continue;
            }
        };

        if is_correct(selected_number, answer) {
            println!("あたり！！");
            break;
        } else {
            println!("ハズレーーーーー");
            let index = candidates.iter().position(|x| *x == selected_number).unwrap();
            candidates.remove(index);
            bot.update_candidates(selected_number);
        }

        //botが数字をcall
        println!("{:?}", bot.call_candidates);
        let bot_selected = bot.call_number();
        println!("botは{:?}を選びました", bot_selected);

        if is_correct(*bot_selected, answer) {
            println!("あたり！！");
            println!("あなたの負けです！！");
            break;
        } else {
            println!("ハズレーーーーー");
            let index = candidates.iter().position(|x| *x == *bot_selected).unwrap();
            candidates.remove(index);
            bot.update_candidates(selected_number);
        }
    }
}

fn input_int() -> Result<usize, ParseIntError> {
    let mut input_number = String::new();
    std::io::stdin().read_line(&mut input_number).expect("入力エラー");
    match input_number.trim().parse::<usize>() {
        Ok(number) => Ok(number),
        Err(e) => Err(e),
    }
}

fn print_candidates(candidates: &[usize]) {
    for candidate in candidates {
        if *candidate == 0 {
            print!("_ ,");
        } else {
            print!("{} ,", *candidate);
        }
    }
    println!("");
}

fn is_correct(selected_number: usize, answer: usize) -> bool {
    println!("選んだ{:?}は...", selected_number);
    println!(".....");

    selected_number == answer
}

struct Bot {
    number: usize,
    call_candidates: Vec<usize>
}

impl Bot {
    fn new() -> Self {
        let number: usize = rand::thread_rng().gen_range(1, 11);
        let mut call_candidates: Vec<usize> = Vec::new();
        for i in 0..=10 { call_candidates.push(number + i); }

        Bot { number, call_candidates }
    }

    fn update_candidates(&mut self, selected_number: usize) {
        if selected_number < 11 {
            self.call_candidates.retain(|&x| x >= selected_number);
        }
        self.call_candidates.retain(|&x| x != selected_number);
    }

    fn call_number(&self) -> &usize {
        self.call_candidates.choose(&mut rand::thread_rng()).unwrap()
    }
}
