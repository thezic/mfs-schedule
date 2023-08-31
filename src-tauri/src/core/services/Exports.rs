use chrono::NaiveDate;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;

use crate::config::Config;
use crate::core::{
    entities::ministry_event::MinistryEvent, errors::DataStoreError,
    traits::MinistryEventRepository,
};

use super::helpers;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Context {
    text: String,
}

#[derive(Debug, Serialize)]
struct Context2 {
    month: String,
    text: String,
    events: Vec<MinistryEvent>,
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

        let context = Context2 {
            events,
            text: extra_context.text,
            month: String::from("September"),
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
        handlebars.register_helper("format", Box::new(helpers::format));

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
