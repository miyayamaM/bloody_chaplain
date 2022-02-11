use rand::Rng;
use rand::seq::SliceRandom;

pub struct Bot {
  pub number: usize,
  pub call_candidates: Vec<usize>
}

impl Bot {
  pub fn new() -> Self {
      let number: usize = rand::thread_rng().gen_range(1, 11);
      let mut call_candidates: Vec<usize> = Vec::new();
      for i in 1..=10 { call_candidates.push(number + i); }

      Bot { number, call_candidates }
  }

  pub fn update_candidates(&mut self, selected_number: usize) {
      if selected_number < 11 {
          self.call_candidates.retain(|&x| x >= selected_number);
      }
      self.call_candidates.retain(|&x| x != selected_number);
  }

  pub fn call_number(&mut self) -> usize {
      let called_number = self.call_candidates.choose(&mut rand::thread_rng()).unwrap();
      println!("botは{:?}を選びました", called_number);
      *called_number
  }
}
