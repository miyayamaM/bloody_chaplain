mod referee;
mod bot;

use crate::referee::Referee;
use crate::bot::Bot;

fn main() {
    let mut referee = Referee::new();

    //あなたの数字をセット
    referee.set_first_number();

    //botの初期化
    let mut bot = Bot::new();

    //botの数字をセット
    referee.set_second_number(bot.number);

    //数を当てる
    loop {
        //あなたが数字をcall
        let called_number = referee.reveice_call();

        //callした数字を判定
        if !referee.validate(called_number) { break; };
        if referee.judge("あなた", called_number) { break; }

        //botのcall候補を更新
        bot.update_candidates(called_number);

        //botが数字をcall
        let bot_called = bot.call_number();
        bot.update_candidates(bot_called);

        //callした数字を判定
        if referee.judge("bot", bot_called) { break; }
    }
    referee.decide_winner();
}
