/*
// This code is not perfect, it is a project to learn Rust and there are more efficient ways to do this
// You can probaly do this code with half the lines in python
// The end result should look somewhat like a neofetch (eventually)
*/
use chrono::{Local, Datelike};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use svg::node::element::{Text, Rectangle};
use svg::node::element::SVG;
use svg::Document;

type CategoryFunction = Box<dyn Fn() -> String>; //this should not work wtf

#[derive(Deserialize)]
struct Config {
    image_path: String,
    output_file: String,
    resize_width: u32,
    resize_height: u32,
    ascii_chars: String,
    categories: Vec<String>,
    big_categories: Vec<String>,
    line_length: usize,
    user: String,
    host: String,
    user_birthdate: Birthdate,
}

#[derive(Deserialize)]
struct Birthdate {
    year: i32,
    month: u32,
    day: u32,
}

fn main() {
    // Load config hopefully
    let config: Config = load_config("config.yaml");

    image_to_ascii(
        &config.image_path,
        &config.output_file,
        config.resize_width,
        config.resize_height,
        &config.ascii_chars,
    );

    let mut function_map: HashMap<&str, CategoryFunction> = HashMap::new();
    function_map.insert("Uptime", Box::new(move || get_uptime(&config.user_birthdate))); //This line took an hour of my life :)
    function_map.insert("Host", Box::new({
        let user = config.user.clone();
        let host = config.host.clone();
        move || get_host(&user, &host)
    }));

    let all_categories = config
        .categories
        .iter()
        .chain(config.big_categories.iter());

    for category in all_categories {
        let value = function_map
            .get(category.as_str())
            .map_or("Unknown".to_string(), |f| f());

        let formatted_category = format_category(
            config.line_length,
            category,
            &config.big_categories,
            &value,
        );

        println!("{}", formatted_category);
    }
}

fn load_config(path: &str) -> Config {
    let file = File::open(path).expect("Failed to open config.yaml");
    serde_yaml::from_reader(file).expect("Failed to parse config.yaml")
}

fn format_category(
    line_length: usize,
    category: &str,
    big_categories: &[String],
    value: &str,
) -> String {
    let is_big = big_categories.contains(&category.to_string());
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

fn get_uptime(birthdate: &Birthdate) -> String {
    let now = Local::now();

    let year_diff = now.year() - birthdate.year;
    let month_diff = now.month() as i32 - birthdate.month as i32;
    let day_diff = now.day() as i32 - birthdate.day as i32;

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

    format!("{} years, {} months, {} days", year_diff, month_diff, day_diff)
}
//this is such bad code but i want a function for each line for clarity for now
fn get_host(user: &str, host: &str) -> String {
    format!("{}@{}", user, host)
}

fn image_to_ascii(
    image_path: &str,
    output_file: &str,
    resize_width: u32,
    resize_height: u32,
    ascii_chars: &str,
) {
    use image::{imageops::FilterType};

    // Open the image
    let img = image::open(image_path).unwrap().to_luma8();

    // Resize the image (really bad)
    // First number is width, second number is height
    // TODO: make this better
    let resized_img = image::imageops::resize(&img, resize_width, resize_height, FilterType::Nearest);

    let (width, height) = resized_img.dimensions();
    let ascii_chars: Vec<char> = ascii_chars.chars().collect();

    // Create or overwrite the output file
    let mut file = File::create(output_file).expect("Creation doesnt work u idiot lol");

    let mut ascii_art = String::new();

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
        ascii_art.push_str(&line);
        file.write_all(line.as_bytes()).expect("Unable to write to file");
    }
    save_svg(&ascii_art);
}

fn save_svg(image: &str) {
    //Escape characters in the input string
    let escaped_image = image
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;");

    let background = Rectangle::new()
        .set("width", 1920)
        .set("height", 980)
        .set("fill", "white");

    //Get each line of image
    let lines: Vec<&str> = escaped_image.split('\n').collect();

    //Group of <tspan> (\n equivalent for .svg files) elements for each line
    let mut text = Text::new()
        .set("x", 20)
        .set("y", 20)
        .set("font-family", "Verdana")
        .set("font-size", 12)
        .set("fill", "black");

    for (i, line) in lines.iter().enumerate() {
        let tspan = svg::node::element::TSpan::new()
            .add(svg::node::Text::new(line.to_string()))
            .set("x", 20) //Reset x
            .set("dy", if i == 0 { 0 } else { 15 }); //Vertical spacing
        text = text.add(tspan);
    }

    let document = Document::new()
        .set("viewBox", (0, 0, 1920, 980))
        .add(background)
        .add(text);

    svg::save("output.svg", &document).unwrap();
    println!("Saved variable to output.svg!");
}