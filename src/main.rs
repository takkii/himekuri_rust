extern crate chrono;
extern crate chrono_tz;

use chrono::{ParseResult, TimeZone, Weekday};
use chrono::offset::FixedOffset;
use chrono::prelude::{Datelike, DateTime, Local, Timelike, Utc};
use chrono_tz::Japan;

#[no_mangle]
pub extern fn himekuri() {
    let utc = Utc::now().naive_utc();
    let dt = Japan.from_utc_datetime(&utc);
    let ja_utc = dt.format("時刻を表示 : %Y年%m月%d日 : %H時%M分%S秒 :").to_string();
    // 令和10年のとき0を消す
    let reiwa = "令和0".to_string() + &(dt.year() - 2018).to_string() + &dt.format("年%m月%d日").to_string();
    // 令和10年のとき0を消す
    let reiwa_alpha = "R0".to_string() + &(dt.year() - 2018).to_string() + &dt.format(".%m.%d").to_string();
    let one = dt.format("%j").to_string();
    let num_one: i64 = one.parse().unwrap();
    let num_one_year: i64 = (366 - num_one);

    let week = dt.format("%w").to_string();

    if week == "0".to_string() {
        println!("{} 日曜日", ja_utc);
    } else if week == "1".to_string() {
        println!("{} 月曜日", ja_utc);
    } else if week == "2".to_string() {
        println!("{} 火曜日", ja_utc);
    } else if week == "3".to_string() {
        println!("{} 水曜日", ja_utc);
    } else if week == "4".to_string() {
        println!("{} 木曜日", ja_utc);
    } else if week == "5".to_string() {
        println!("{} 金曜日", ja_utc);
    } else if week == "6".to_string() {
        println!("{} 土曜日", ja_utc);
    }

    let version = "1.0.1";

    println!("来年の1月1日まであと : {} 日です", num_one_year);
    println!("{} : {}", reiwa, reiwa_alpha);
    println!("日めくり数え番号 : {}", version);
}

fn main() {
    himekuri()
}
