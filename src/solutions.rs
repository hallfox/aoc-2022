macro_rules! solutions {
    ($day:ident) => {
        mod $day;
        pub use $day::$day;
    };

    ($day:ident, $($days:ident),+) => {
        solutions! { $day }
        solutions! { $($days),+ }
    };
}

solutions!(day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14);