/* 
 * Copyright (C) 2024 khumnath cg <mail@khumnath.com.np>
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
#include "bikram.h"


int main(int argc, char * argv[]) {
  if (argc != 6 || std::string(argv[1]) != "--conv" ||
    (std::string(argv[2]) != "--tobs" && std::string(argv[2]) != "--toad")) {
    std::cerr << "Usage: " << argv[0]  << "\n\e[31m Convert to Nepali Date: --conv --tobs year month day\n Convert to Gregorian Date: --conv --toad year month day\n\e[0m";
    std::time_t now = std::time(nullptr);
    std::tm timeinfo = * std::localtime( & now);

    int year = timeinfo.tm_year + 1900;
    int month = timeinfo.tm_mon + 1; // Adjust month from 0-based to 1-based
    int day = timeinfo.tm_mday;

    bikram bsdate;
    bsdate.fromGregorian(year, month, day);

    std::string bs_weekday_name = bsdate.getWeekdayName(year, month, day);

    std::cout << "\e[36m   Today's Date:\e[0m" << std::endl;
    std::cout << " \e[33m Gregorian: \e[0m"
    "\e[35m" << year << " " << month << " " << day << " " << bsdate.getWeekdayName(year, month, day) << " " << "\e[0m" << std::endl;
    std::cout << " \e[33m Bikram Sambat: \e[0m"
    "\e[35m" << bsdate.getYear() << " " << bsdate.getMonth() << " " << bsdate.getDay() << " " << bs_weekday_name << " "<< " \e[33m days in bikram month: \e[0m" << bsdate.daysInMonth(bsdate.getYear(), bsdate.getMonth()) << "\e[0m" << std::endl;
    return 1;
  }

  std::string convType = argv[2];
  int year = std::stoi(argv[3]);
  int month = std::stoi(argv[4]);
  int day = std::stoi(argv[5]);

  bikram bsdate;

  if (convType == "--tobs") {
    bsdate.fromGregorian(year, month, day);
    int convertedYear;
    int convertedMonth;
    int convertedDay;
    int gregorian_weekday = bsdate.getDayOfWeek();

  } else if (convType == "--toad") {
    bsdate.fromNepali(year, month, day);
  }

  int convertedYear = bsdate.getYear();
  int convertedMonth = bsdate.getMonth();
  int convertedDay = bsdate.getDay();
  std::string bs_weekday_name = bsdate.getWeekdayName(year, month, day);
  std::string gregorian_weekday_name = "";
  if (convType == "--tobs") {
    // Get weekday name for Gregorian date after conversion
    gregorian_weekday_name = bsdate.getWeekdayName(convertedYear, convertedMonth, convertedDay);

    std::cout << " \e[33m Bikram Sambat Date: \e[0m"
    "\e[35m" << convertedYear << " " << convertedMonth << " " << convertedDay << " " << bs_weekday_name << " " << "\e[0m" << " \e[33m days in bikram month: \e[0m" << bsdate.daysInMonth(bsdate.getYear(), bsdate.getMonth()) << "\e[0m" << std::endl;
  } else if (convType == "--toad") {
    bsdate.fromNepali(year, month, day);

    // Convert to Gregorian date for weekday calculation
    bsdate.toGregorian(year, month, day, convertedYear, convertedMonth, convertedDay);

    int gregorian_weekday = bsdate.getDayOfWeek();
    std::string gregorian_weekday_name = bsdate.getWeekdayName(convertedYear, convertedMonth, convertedDay);

    std::cout << "\e[33m Gregorian Date: \e[0m"
    "\e[35m" << convertedYear << " " << convertedMonth << " " << convertedDay << " " << gregorian_weekday_name << " " << "\e[0m" << std::endl;
  }

  return 0;
}
