use chrono::NaiveDate;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::env::temp_dir;
use std::fs::File;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::core::{
    entities::ministry_event::MinistryEvent, errors::DataStoreError,
    traits::MinistryEventRepository,
};

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

pub struct ExportService {
    events_repository: Box<dyn MinistryEventRepository>,
}

impl ExportService {
    pub fn new(events_repository: Box<dyn MinistryEventRepository>) -> ExportService {
        ExportService { events_repository }
    }

    pub async fn export_pdf(
        &self,
        // template: &Path,
        from: NaiveDate,
        to: NaiveDate,
        extra_context: Context,
    ) -> Result<PathBuf, DataStoreError> {
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
        handlebars
            .register_template_string("template", include_str!("./template.html.hbs"))
            .unwrap();

        handlebars.render_to_write("template", &context, file);
        Ok(filepath)
    }
}
