#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, 1) => match units {
            TimeUnit::Hours => format!("an hour ago"),
            _ => format!("a {} ago", units.singular()),
        },
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(units, 1) => match units {
            TimeUnit::Hours => format!("an hour from now"),
            _ => format!("a {} from now", units.singular()),
        },
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
    }
}

fn main() {
    let four_score_and_serven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    println!("{:?}", four_score_and_serven_years_ago);
    println!("{:?}", three_hours_from_now);
    println!("{}", rough_time_to_english(four_score_and_serven_years_ago));
    let one_hours_ago = RoughTime::InThePast(TimeUnit::Hours, 1);
    println!("{}", rough_time_to_english(one_hours_ago));
}
