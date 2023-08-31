use chrono::NaiveDate;
use handlebars::*;

handlebars_helper!(format: |dt: NaiveDate, fmt:str| dt.format(fmt).to_string());
