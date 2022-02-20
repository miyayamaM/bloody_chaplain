use std::{thread, time};

pub struct Referee {
  first_number: usize,
  second_number: usize,
  pub candidates: Vec<usize>,
  pub winner: String,
}

impl Referee {
  pub fn new() -> Self {
    let mut candidates: Vec<usize> = Vec::new();
    for i in 2..=20 { candidates.push(i); }
    Referee { first_number: 0, second_number: 0, candidates, winner: String::new() }
  }

  pub fn set_first_number(&mut self) {
    let your_number: usize;
    loop {
      println!("1~10までの数字を選んでね");
      your_number = match Referee::input_int() {
          Ok(number) if number < 11 && number > 0 => number,
          _ => {
              println!("[不正な値です]1~10までの数字を選んでください");
              continue;
          }
      };
      break;
    }
    println!("あなたは{}を選択", your_number);
    self.first_number = your_number;
  }

  pub fn set_second_number(&mut self, number: usize) {
    self.second_number = number;
  }

  fn input_int() -> Result<usize, std::num::ParseIntError> {
    let mut input_number = String::new();
    std::io::stdin().read_line(&mut input_number).expect("入力エラー");
    match input_number.trim().parse::<usize>() {
        Ok(number) => Ok(number),
        Err(e) => Err(e),
    }
  }

  pub fn reveice_call(&self) -> usize {
    println!("数字をcallしてください");
    println!("以下から数字を選んでね");
    self.print_candidates();

    let called_number: usize;
    loop {
      called_number = match Referee::input_int() {
        Ok(number) if self.candidates.iter().any(|candidate| *candidate == number) => number,
        _ => {
            println!("候補の中から選んでください");
            continue;
        }
      };
      break;
    }
    println!("{}を選びました", called_number);
    called_number
  }

  pub fn judge(&mut self, caller: &str, called_number: usize) -> bool {
    println!("{}は...", called_number);
    thread::sleep(time::Duration::from_millis(2000));
    println!("...");

    let is_correct = called_number == self.first_number + self.second_number;

    if is_correct {
      println!("あたり！！");
      self.winner = caller.to_string();
    } else {
      println!("ハズレーーーーー");
      let index = self.candidates.iter().position(|x| *x == called_number).unwrap();
      self.candidates[index] = 0;
    }
    thread::sleep(time::Duration::from_millis(2000));
    is_correct
  }

  pub fn validate(&mut self, called_number: usize) -> bool {
    if called_number <= self.first_number || called_number > self.first_number + 10 {
      println!("あり得ない数字はcallできません");
      self.winner = "bot".to_string();
      return false;
    }
    return true;
  }

  pub fn print_candidates(&self) {
    for &candidate in &self.candidates {
        if candidate == 0 {
            print!("_ ,");
        } else {
            print!("{} ,", candidate);
        }
    }
    println!("");
  }

  pub fn decide_winner(&self) {
    println!("{}の勝ちです", self.winner);
  }
}
