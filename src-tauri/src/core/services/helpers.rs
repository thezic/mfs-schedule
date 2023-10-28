use handlebars::handlebars_helper;
use handlebars::HelperDef;
use handlebars::*;
use serde_json::Value as Json;

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

pub struct FirstHelper;
impl HelperDef for FirstHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h
            .param(0)
            .ok_or(RenderError::new("No param found for helper 'first'"))?;

        let v = match *value.value() {
            Json::Array(ref list) => list.first(),
            _ => None,
        };

        if v.is_none() {
            return Ok(());
        }

        let mut block = BlockContext::new();
        block.set_base_value(v.unwrap().clone());

        rc.push_block(block);

        if let Some(template) = h.template() {
            template.render(r, ctx, rc, out)?;
        }

        rc.pop_block();

        Ok(())
    }
}

pub struct LastHelper;
impl HelperDef for LastHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h
            .param(0)
            .ok_or(RenderError::new("No param found for helper 'last'"))?;

        let v = match *value.value() {
            Json::Array(ref list) => list.last(),
            _ => None,
        };

        if v.is_none() {
            return Ok(());
        }

        let mut block = BlockContext::new();
        block.set_base_value(v.unwrap().clone());

        rc.push_block(block);

        if let Some(template) = h.template() {
            template.render(r, ctx, rc, out)?;
        }

        rc.pop_block();

        Ok(())
    }
}

handlebars_helper!(last: |x: Json| match x {
    Json::String(s) => to_json(s.chars().last()),
    Json::Array(a) => to_json(a.last()),
    _ => to_json(""),
});

handlebars_helper!(format_time: |t: NaiveTime, fmt:str| t.format(fmt).to_string());
handlebars_helper!(markdown: |text: str| md::to_html(text));
handlebars_helper!(capitalize: |text: str| {
let mut chars = text.chars();
match chars.next() {
    Some(first_character) => first_character.to_uppercase().to_string() + chars.as_str(),
    None => String::new(),
}});
