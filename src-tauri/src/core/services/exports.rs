use chrono::{Datelike, Duration, NaiveDate};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::mem;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;

use crate::config::Config;
use crate::core::{
    entities::ministry_event::MinistryEvent, errors::DataStoreError,
    traits::MinistryEventRepository,
};

use super::helpers;

fn find_month_in_range(from: &NaiveDate, to: &NaiveDate) -> NaiveDate {
    // count the days in each month
    let stats = from
        .iter_days()
        .take_while(|d| d <= to)
        .fold(HashMap::new(), |mut acc, d| {
            let counter = acc.entry((d.year(), d.month())).or_insert(0);
            *counter += 1;
            acc
        });

    // Find the month with most days in
    let ((year, month), _) =
        stats
            .iter()
            .fold(((0, 0), 0), |(max_month, max_count), (&month, &count)| {
                if count > max_count {
                    (month, count)
                } else {
                    (max_month, max_count)
                }
            });

    let best_match = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    best_match.clamp(*from, *to)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    macro_rules! get_month_tests {
        ($($name:ident: from:$from:expr, to:$to:expr, expected:$expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let from = NaiveDate::parse_from_str($from, "%Y-%m-%d").unwrap();
                let to = NaiveDate::parse_from_str($to, "%Y-%m-%d").unwrap();
                let expected = NaiveDate::parse_from_str($expected, "%Y-%m-%d").unwrap();
                assert_eq!(find_month_in_range(&from, &to), expected);
            }
        )*
        }
    }

    get_month_tests! {
        t1: from: "2023-08-28", to: "2023-10-01", expected: "2023-09-01",
        t2: from: "2023-09-28", to: "2023-11-01", expected: "2023-10-01",
        t3: from: "2023-09-04", to: "2023-10-04", expected: "2023-09-04",
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Context {
    text: String,
}

#[derive(Debug, Serialize)]
struct RenderContext {
    month: NaiveDate,
    text: String,
    events: Vec<MinistryEvent>,
    from: NaiveDate,
    to: NaiveDate,
}

pub struct ExportService<'a> {
    config: &'a Config,
    events_repository: Box<dyn MinistryEventRepository>,
}

impl ExportService<'_> {
    pub fn new(
        config: &Config,
        events_repository: Box<dyn MinistryEventRepository>,
    ) -> ExportService {
        ExportService {
            config,
            events_repository,
        }
    }

    pub async fn export_pdf(
        &self,
        // template: &Path,
        from: NaiveDate,
        to: NaiveDate,
        extra_context: Context,
    ) -> Result<PathBuf, DataStoreError> {
        println!(
            "Exporting pdf to folder {}",
            self.config.export.export_folder.display(),
        );
        let events = self.events_repository.get_range(from, to).await?;

        let context = RenderContext {
            events,
            text: extra_context.text,
            month: find_month_in_range(&from, &to),
            from,
            to,
        };

        let mut filepath = temp_dir();
        let filename = format!("{}.html", Uuid::new_v4());
        filepath.push(filename);

        let file = File::create(&filepath).unwrap();

        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars
            .register_template_string("template", include_str!("./template.html.hbs"))
            .unwrap();
        handlebars.register_helper("format_date", Box::new(helpers::format_date));
        handlebars.register_helper("format_time", Box::new(helpers::format_time));
        handlebars.register_helper("markdown", Box::new(helpers::markdown));
        handlebars.register_helper("capitalize", Box::new(helpers::capitalize));

        println!("Rendering template using context {:#?}", &context);

        if let Err(err) = handlebars.render_to_write("template", &context, file) {
            println!("Failed to render {:#?}", err);
        }

        let mut output = self.config.export.export_folder.clone();
        output.push("output.pdf");

        let mut command = Command::new(&self.config.export.command);
        let arg_context = HashMap::from([
            (
                "input_file",
                filepath.clone().into_os_string().into_string().unwrap(),
            ),
            (
                "output_file",
                output.into_os_string().into_string().unwrap(),
            ),
        ]);

        let args = self
            .config
            .export
            .args
            .iter()
            .map(|template_string| handlebars.render_template(template_string, &arg_context))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        println!("Using args {:#?}", &args);
        command.args(args);
        command.spawn().expect("Failed to export");

        println!("Exported file: {}", filepath.display());
        Ok(filepath)
    }
}
