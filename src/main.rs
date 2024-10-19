use std::env;
use std::process;
use chrono::{Datelike, Local};
mod bikram;
use bikram::Bikram;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 || args[1] != "--conv" || (args[2] != "--tobs" && args[2] != "--toad") {
        eprintln!(
            "Usage: {}\n\x1b[31m Convert to Nepali Date: --conv --tobs year month day\n Convert to Gregorian Date: --conv --toad year month day\x1b[0m",
            args[0]
        );

        // Get current date
        let now_date = Local::now();
        let year = now_date.year();
        let month = now_date.month();
        let day = now_date.day();

        let mut bsdate = Bikram::new();
        bsdate.from_gregorian(year, month as i32, day as i32);

        let bs_weekday_name = bsdate.get_weekday_name(year, month as i32, day as i32);

        println!("\x1b[36m   Today's Date:\x1b[0m");
        println!(
            " \x1b[33m Gregorian: \x1b[0m \x1b[35m{} {} {} {} \x1b[0m",
            year, month, day, bs_weekday_name
        );
        println!(
            " \x1b[33m Bikram Sambat: \x1b[0m \x1b[35m{} {} {} {} \x1b[33m days in bikram month: \x1b[0m{} \x1b[0m",
            bsdate.get_year(),
            bsdate.get_month(),
            bsdate.get_day(),
            bs_weekday_name,
            bsdate.days_in_month(bsdate.get_year(), bsdate.get_month())
        );
        process::exit(1);
    }

    let conv_type = &args[2];
    let year = args[3].parse::<i32>().expect("Year must be an integer.");
    let month = args[4].parse::<i32>().expect("Month must be an integer.");
    let day = args[5].parse::<i32>().expect("Day must be an integer.");

    let mut bsdate = Bikram::new();

    if conv_type == "--tobs" {
        bsdate.from_gregorian(year, month, day);
    } else if conv_type == "--toad" {
        bsdate.from_nepali(year, month, day);
    }

    let converted_year = bsdate.get_year();
    let converted_month = bsdate.get_month();
    let converted_day = bsdate.get_day();
    let bs_weekday_name = bsdate.get_weekday_name(converted_year, converted_month, converted_day);

    if conv_type == "--tobs" {
        let _gregorian_weekday_name = bsdate.get_weekday_name(converted_year, converted_month, converted_day);

        println!(
            " \x1b[33m Bikram Sambat Date: \x1b[0m \x1b[35m{} {} {} {} \x1b[0m \x1b[33m days in bikram month: \x1b[0m{} \x1b[0m",
            converted_year,
            converted_month,
            converted_day,
            bs_weekday_name,
            bsdate.days_in_month(bsdate.get_year(), bsdate.get_month())
        );
    } else if conv_type == "--toad" {
        let gregorian_weekday_name = bsdate.get_weekday_name(converted_year, converted_month, converted_day);

        println!(
            "\x1b[33m Gregorian Date: \x1b[0m \x1b[35m{} {} {} {} \x1b[0m",
            converted_year, converted_month, converted_day, gregorian_weekday_name
        );
    }
}
