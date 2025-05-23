mod as_date;
mod as_datetime;
mod convert_time_zone;
mod datepart;
mod get_day;
mod get_hour;
mod get_minute;
mod get_month;
mod get_nanosecond;
mod get_ordinal;
mod get_second;
mod get_week;
mod get_weekday;
mod get_year;
mod replace_time_zone;
mod strftime;
mod truncate;

use crate::PolarsPlugin;
use nu_plugin::PluginCommand;

pub use as_date::AsDate;
pub use as_datetime::AsDateTime;
pub use convert_time_zone::ConvertTimeZone;
pub use datepart::ExprDatePart;
pub use get_day::GetDay;
pub use get_hour::GetHour;
pub use get_minute::GetMinute;
pub use get_month::GetMonth;
pub use get_nanosecond::GetNanosecond;
pub use get_ordinal::GetOrdinal;
pub use get_second::GetSecond;
pub use get_week::GetWeek;
pub use get_weekday::GetWeekDay;
pub use get_year::GetYear;
pub use replace_time_zone::ReplaceTimeZone;
pub use strftime::StrFTime;
pub use truncate::Truncate;

pub(crate) fn datetime_commands() -> Vec<Box<dyn PluginCommand<Plugin = PolarsPlugin>>> {
    vec![
        Box::new(AsDate),
        Box::new(AsDateTime),
        Box::new(ConvertTimeZone),
        Box::new(ExprDatePart),
        Box::new(GetDay),
        Box::new(GetHour),
        Box::new(GetMinute),
        Box::new(GetMonth),
        Box::new(GetNanosecond),
        Box::new(GetOrdinal),
        Box::new(GetSecond),
        Box::new(GetWeek),
        Box::new(GetWeekDay),
        Box::new(GetYear),
        Box::new(ReplaceTimeZone),
        Box::new(StrFTime),
        Box::new(Truncate),
    ]
}
