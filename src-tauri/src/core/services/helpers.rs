use handlebars::handlebars_helper;
use handlebars::HelperDef;
use handlebars::*;

use ::markdown as md;
use chrono::Locale;
use chrono::NaiveDate;
use chrono::NaiveTime;

struct MyLocale(Locale);
impl From<&str> for MyLocale {
    fn from(value: &str) -> Self {
        match value {
            "lv_LV" => MyLocale(Locale::lv_LV),
            "sv_SE" => MyLocale(Locale::sv_SE),
            "sv_FI" => MyLocale(Locale::sv_FI),
            "sv_FI_euro" => MyLocale(Locale::sv_FI_euro),
            _ => panic!("Unknown locale or something {}", value),
        }
    }
}

impl From<MyLocale> for Locale {
    fn from(value: MyLocale) -> Self {
        value.0
    }
}

pub struct FormatDateHelper {
    locale: Locale,
}

impl FormatDateHelper {
    pub fn new(locale: &str) -> FormatDateHelper {
        FormatDateHelper {
            locale: MyLocale::from(locale).into(),
        }
    }
}

impl HelperDef for FormatDateHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'reg, 'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<handlebars::ScopedJson<'reg, 'rc>, RenderError> {
        let date_string = h
            .param(0)
            .and_then(|x| x.value().as_str())
            .ok_or(RenderError::new("Parameter 'format' is missing"))?;

        let fmt = h
            .param(1)
            .and_then(|x| x.value().as_str())
            .ok_or(RenderError::new("Parameter 'fmt' is missing"))?;

        let dt = date_string.parse::<NaiveDate>().map_err(|err| {
            RenderError::new(format!("Cannot convert '{}' to date: {}", date_string, err))
        })?;

        Ok(to_json(dt.format_localized(fmt, self.locale).to_string()).into())
    }
}

handlebars_helper!(format_time: |t: NaiveTime, fmt:str| t.format(fmt).to_string());
handlebars_helper!(markdown: |text: str| md::to_html(text));
handlebars_helper!(capitalize: |text: str| {
let mut chars = text.chars();
match chars.next() {
    Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    None => String::new(),
}});
