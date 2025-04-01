/*
// This code is not perfect, it is a project to learn Rust and there are more efficient ways to do this
// You can probaly do this code with half the lines in python
// The end result should look somewhat like a neofetch
*/
use chrono::{Local, Datelike};
use std::collections::HashMap;
type CategoryFunction = fn() -> String;

fn get_uptime() -> String{
    let now = Local::now();
    let user: Vec<(&str, &str)> = vec![
        ("year", "2006"),
        ("month", "12"),
        ("day", "11"),
    ];

    let user_year: i32 = user.iter().find(|&&(k, _)| k == "year").unwrap().1.parse().unwrap();
    let user_month: u32 = user.iter().find(|&&(k, _)| k == "month").unwrap().1.parse().unwrap();
    let user_day: u32 = user.iter().find(|&&(k, _)| k == "day").unwrap().1.parse().unwrap();


    let year_diff = now.year() - user_year;
    let month_diff = now.month() as i32 - user_month as i32;
    let day_diff = now.day() as i32 - user_day as i32;

    // Ajust month and year difference if negative
    let (year_diff, month_diff) = if month_diff < 0 || (month_diff == 0 && day_diff < 0) {
        (year_diff - 1, month_diff + 12)
    } else {
        (year_diff, month_diff)
    };
    // Adjust day difference if negative
    let day_diff = if day_diff < 0 {
        let prev_month_days = (now - chrono::Duration::days(now.day() as i64)).day() as i32;
        day_diff + prev_month_days
    } else {
        day_diff
    };
    let uptime = format!("{} years, {} months, {} days", year_diff, month_diff, day_diff);
    
    println!("{}", uptime);
}

fn get_host() {
    let user = "Romain";
    let host = "git";
    println!("{}@{}", user, host);
}

fn format_category(line_length: usize, category: &str, big_categories: &[&str]) -> String {

    let is_big = big_categories.contains(&category);
    let mut output = String::from(category);
    
    if is_big {
        output.push(' ');
        while output.len() < line_length {
            output.push('-');
        }
    } else {
        output.push(':');
        while output.len() < line_length {
            output.push('.');
        }
    }
    
    output
}

fn main() {
    let _ = get_host();
    let line_length = 60;
    let categories = vec!["Uptime", "Lang"];
    let big_categories = vec!["Host", "Contact", "Stats"];

    let mut function_map: HashMap<&str, CategoryFunction> = HashMap::new();
    function_map.insert("Uptime", get_uptime);
    //function_map.insert("Lang", get_lang);
    
    let all_categories = categories.iter().chain(big_categories.iter());
    
    for category in all_categories {
        let formatted_category = format_category(line_length, category, &big_categories);
        let value = function_map.get(category).map_or("Unknown".to_string(), |f| f());
        println!("{} {}", formatted_category, value);
    }
    //let _ = get_contact();
    let _ = get_uptime();

}