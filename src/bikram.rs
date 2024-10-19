use std::f64::consts::PI;
use chrono::NaiveDate;
pub struct Bikram {
    year: i32,
    month: i32,
    day: i32,
    yuga_rotation_star: f64,
    yuga_rotation_sun: f64,
    yuga_civil_days: f64,
    planet_apogee_sun: f64,
    planet_circum_sun: f64,
    rad: f64,
}

impl Bikram {
    pub fn new() -> Self {
        Self {
            year: 0,
            month: -1,
            day: 0,
            yuga_rotation_star: 1582237828.0,
            yuga_rotation_sun: 4320000.0,
            yuga_civil_days: 0.0,
            planet_apogee_sun: 77.0 + 17.0 / 60.0,
            planet_circum_sun: 13.0 + 50.0 / 60.0,
            rad: 180.0 / PI,
        }
    }

    pub fn get_saura_masa_day(&self, ahar: i64) -> (i32, i32) {
        if self.today_saura_masa_first_p(ahar) {
            let tslong_tomorrow = self.get_tslong(ahar + 1);
            let month = ((tslong_tomorrow / 30.0) as i32) % 12;
            (month, 1)
        } else {
            let (prev_month, mut day) = self.get_saura_masa_day(ahar - 1);
            day += 1;
            (prev_month, day)
        }
    }

    pub fn today_saura_masa_first_p(&self, ahar: i64) -> bool {
        let tslong_today = self.get_tslong(ahar) % 30.0;
        let tslong_tomorrow = self.get_tslong(ahar + 1) % 30.0;
        tslong_today > 25.0 && tslong_tomorrow < 5.0
    }

    pub fn get_tslong(&self, ahar: i64) -> f64 {
        let t1 = (self.yuga_rotation_sun * ahar as f64 / self.yuga_civil_days) % 1.0;
        let mslong = 360.0 * t1;
        let x1 = mslong - self.planet_apogee_sun;
        let y1 = self.planet_circum_sun / 360.0;
        let y2 = (x1 / self.rad).sin();
        let y3 = y1 * y2;
        let x2 = (y3.asin()) * self.rad;
        mslong - x2
    }

    pub fn get_julian_date(&self, year: i32, month: i32, day: i32) -> f64 {
        let (mut year, mut month) = (year, month);
        if month <= 2 {
            year -= 1;
            month += 12;
        }
        let a = (year as f64 / 100.0).floor();
        let b = 2.0 - a + (a / 4.0).floor();
        (365.25 * (year as f64 + 4716.0)).floor() + (30.6001 * (month as f64 + 1.0)).floor() + day as f64 + b - 1524.5
    }

    pub fn from_julian_date(&self, julian_date: f64) -> (i32, i32, i32) {
        let z = (julian_date + 0.5).floor() as i32;
        let a = if z < 2299161 {
            z
        } else {
            let alpha = ((z as f64 - 1867216.25) / 36524.25).floor() as i32;
            z + 1 + alpha - (alpha / 4)
        };
        let b = a + 1524;
        let c = ((b as f64 - 122.1) / 365.25).floor() as i32;
        let d = (365.25 * c as f64).floor() as i32;
        let e = ((b - d) as f64 / 30.6001).floor() as i32;

        let day = b - d - (30.6001 * e as f64) as i32;
        let month = if e < 14 { e - 1 } else { e - 13 };
        let year = if month > 2 { c - 4716 } else { c - 4715 };

        (year, month, day)
    }

    pub fn from_gregorian(&mut self, y: i32, m: i32, d: i32) {
        self.yuga_civil_days = self.yuga_rotation_star - self.yuga_rotation_sun;
        let julian = self.get_julian_date(y, m, d);
        let ahar = julian as i64 - 588465;
        let (saura_masa_num, saura_masa_day) = self.get_saura_masa_day(ahar);
        let year_kali = (ahar as f64 * self.yuga_rotation_sun / self.yuga_civil_days).floor() as i64;
        let year_saka = year_kali - 3179;
        let nepalimonth = saura_masa_num % 12;
        self.year = (year_saka + 135 + ((saura_masa_num - nepalimonth) / 12) as i64) as i32;
        self.month = (saura_masa_num + 12) % 12;
        self.day = saura_masa_day;
    }

    pub fn to_gregorian(&mut self, bs_year: i32, bs_month: i32, bs_day: i32) -> (i32, i32, i32) {
        let mut bs_month = bs_month;
        self.yuga_civil_days = self.yuga_rotation_star - self.yuga_rotation_sun;
        let year_saka = bs_year - 135;
        let year_kali = year_saka + 3179;
        let mut ahar = ((year_kali as f64 * self.yuga_civil_days) / self.yuga_rotation_sun).floor() as i64;
        let mut saura_masa_num;
        let mut saura_masa_day;
        bs_month = (bs_month + 11) % 12;
        loop {
            let result = self.get_saura_masa_day(ahar);
            saura_masa_num = result.0;
            saura_masa_day = result.1;
            if saura_masa_num == bs_month && saura_masa_day == bs_day {
                break;
            }
            ahar += if saura_masa_num < bs_month || (saura_masa_num == bs_month && saura_masa_day < bs_day) {
                1
            } else {
                -1
            };
        }
        let julian_date = ahar as f64 + 588465.5;
        self.from_julian_date(julian_date)
    }

    pub fn from_nepali(&mut self, bs_year: i32, bs_month: i32, bs_day: i32) {
        let (g_year, g_month, g_day) = self.to_gregorian(bs_year, bs_month, bs_day);
        self.year = g_year;
        self.month = g_month - 1;
        self.day = g_day;
    }

    pub fn get_year(&self) -> i32 {
        self.year
    }

    pub fn get_month(&self) -> i32 {
        self.month + 1
    }

    pub fn get_day(&self) -> i32 {
        self.day
    }

    pub fn get_weekday_name(&self, year: i32, month: i32, day: i32) -> String {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month as u32, day as u32) {
            date.format("%A").to_string()
        } else {
            "Invalid date".to_string() // Handle invalid date case
        }
    }

    pub fn days_in_month(&mut self, bs_year: i32, bs_month: i32) -> i32 {
        let next_month = (bs_month % 12) + 1;
        let next_year = if bs_month == 12 { bs_year + 1 } else { bs_year };
        let (g_year_start, g_month_start, g_day_start) = self.to_gregorian(bs_year, bs_month, 1);
        let julian_start = self.get_julian_date(g_year_start, g_month_start, g_day_start);
        let (g_year_end, g_month_end, g_day_end) = self.to_gregorian(next_year, next_month, 1);
        let julian_end = self.get_julian_date(g_year_end, g_month_end, g_day_end);
        (julian_end - julian_start) as i32
    }
}
