use chrono::{Datelike, Local};
use bikram::bikram::Bikram;
fn main() {
    // Get today's date in NaiveDate format
    // Get current date
        let now_date = Local::now();
        let year = now_date.year();
        let month = now_date.month();
        let day = now_date.day();

        let mut bsdate = Bikram::new();
        bsdate.from_gregorian(year, month as i32, day as i32);

        let bs_weekday_name = bsdate.get_weekday_name(year, month as i32, day as i32);
println!(
            " \x1b[33m Bikram Sambat: \x1b[0m \x1b[35m{} {} {} {} \x1b[33m days in bikram month: \x1b[0m{} \x1b[0m",
            bsdate.get_year(),
            bsdate.get_month(),
            bsdate.get_day(),
            bs_weekday_name,
            bsdate.days_in_month(bsdate.get_year(), bsdate.get_month())
        );
}