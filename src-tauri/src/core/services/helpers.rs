use handlebars::handlebars_helper;

use ::markdown as md;
use chrono::Locale;
use chrono::NaiveDate;
use chrono::NaiveTime;

handlebars_helper!(format_date: |dt: NaiveDate, fmt:str| dt.format_localized(fmt, Locale::lv_LV).to_string());
handlebars_helper!(format_time: |t: NaiveTime, fmt:str| t.format(fmt).to_string());
handlebars_helper!(markdown: |text: str| md::to_html(text));
handlebars_helper!(capitalize: |text: str| {
let mut chars = text.chars();
match chars.next() {
    Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    None => String::new(),
}});
