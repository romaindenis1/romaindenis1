/*
// This code is not perfect, it is a project to learn Rust and there are more efficient ways to do this
// You can probaly do this code with half the lines in python
// The end result should look somewhat like a neofetch (eventually)
*/
use chrono::{Local, Datelike};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
type CategoryFunction = Box<dyn Fn() -> String>; //this should not work wtf

fn main() {
    image_to_ascii("C:/Users/ps04egl/Documents/GitHub/romaindenis1/xioajie.jpeg");
    // Global variables (should not need to change)
    let categories = vec!["Uptime", "Language"];  
    let big_categories = vec!["Host", "Contact", "Stats"];
    let line_length = 60;

    // Variables that should change
    let user = "romain";
    let host = "denis";
    
    let mut function_map: HashMap<&str, CategoryFunction> = HashMap::new();
    function_map.insert("Uptime", Box::new(get_uptime)); 
    function_map.insert("Host", Box::new({
        let user = user.to_string();
        let host = host.to_string();
        move || get_host(&user, &host) 
    }));

    let all_categories = categories.iter().chain(big_categories.iter());

    for category in all_categories {
        let value = function_map
            .get(*category)
            .map_or("Unknown".to_string(), |f| f());

        let formatted_category = format_category(line_length, category, &big_categories, &value);

        println!("{}", formatted_category);
    }
}

fn format_category(line_length: usize, category: &str, big_categories: &[&str], value: &str) -> String {
    let is_big = big_categories.contains(&category);
    let mut output = String::from(category);

    // Add the separator based on the category type
    if is_big {
        output.push(' ');
    } else {
        output.push(':');
    }

    let value_length = value.len();

    // Calculate the remaining length for separators
    let used_length = output.len() + value_length;  
    let remaining_length = line_length - used_length;  

    if is_big {
        output.push_str(value);
        output.push(' ');  
        output.push_str(&"-".repeat(remaining_length));
    } else {
        output.push_str(&".".repeat(remaining_length));
        output.push_str(value);
    }

    output
}

fn get_uptime() -> String{
    let now = Local::now();
    let user_birthdate: Vec<(&str, &str)> = vec![
        ("year", "2006"),
        ("month", "12"),
        ("day", "11"),
    ];

    let user_year: i32 = user_birthdate.iter().find(|&&(k, _)| k == "year").unwrap().1.parse().unwrap();
    let user_month: u32 = user_birthdate.iter().find(|&&(k, _)| k == "month").unwrap().1.parse().unwrap();
    let user_day: u32 = user_birthdate.iter().find(|&&(k, _)| k == "day").unwrap().1.parse().unwrap();


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
    
    uptime
}

//this is such bad code but i want a function for each line for clarity
fn get_host(user: &str, host: &str) -> String {
    format!("{}@{}", user, host)
}

fn image_to_ascii(image_path: &str) {
    use image::{GenericImageView, imageops::FilterType};

    // Open the image
    let img = image::open(image_path).unwrap().to_luma8();

    // Resize the image (really bad)
    // First number is width, second number is height
    // TODO: make this better
    let resized_img = image::imageops::resize(&img, 80, 40, FilterType::Nearest);

    let (width, height) = resized_img.dimensions();
    let ascii_chars = ['@', '#', '8', '&', 'o', ':', '*', '.', ' ', 'x', 'X', '%', '=', '+'];


    // Create or overwrite the output file
    let mut file = File::create("output.txt").expect("Creation doesnt work u idiot lol");

    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            let pixel = resized_img.get_pixel(x, y)[0] as f32;
            // I had this at 255.0 and everything i saw was using 255.0 but i increased it and it was better lol
            // Surely this won't lead to problems in the future
            let idx = (pixel / 512.0 * (ascii_chars.len() as f32 - 1.0)) as usize;
            line.push(ascii_chars[idx]);
        }
        line.push('\n');
        file.write_all(line.as_bytes()).expect("Write doesnt work u idiot lol");
    }

    println!("ASCII art has been saved");
}