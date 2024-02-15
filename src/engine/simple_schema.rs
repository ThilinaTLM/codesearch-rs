use serde::{Deserialize, Serialize};
use tantivy::doc;
use tantivy::schema::{Field, STORED, STRING, TEXT, TextOptions};

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaWrapperModel {
    pub repo_name: String,
    pub repo_path: String,
    pub repo_type: String,

    pub file_name: String,
    pub file_path: String,
    pub file_ext: String,
    pub file_size: u64,
    pub file_content: String,

    pub last_updated: chrono::DateTime<chrono::Utc>,
}

pub enum SchemaWrapperFields {
    RepoName,
    RepoPath,
    RepoType,

    FileName,
    FilePath,
    FileExt,
    FileSize,
    FileContent,

    LastUpdated,
}

impl SchemaWrapperFields {
    pub fn get_name(&self) -> &str {
        match self {
            SchemaWrapperFields::RepoName => "repo_name",
            SchemaWrapperFields::RepoPath => "repo_path",
            SchemaWrapperFields::RepoType => "repo_type",

            SchemaWrapperFields::FileName => "file_name",
            SchemaWrapperFields::FilePath => "file_path",
            SchemaWrapperFields::FileExt => "file_ext",
            SchemaWrapperFields::FileSize => "file_size",
            SchemaWrapperFields::FileContent => "file_content",

            SchemaWrapperFields::LastUpdated => "last_updated",
        }
    }

    pub fn get_type(&self) -> TextOptions {
        match self {
            SchemaWrapperFields::RepoName => STRING | STORED,
            SchemaWrapperFields::RepoPath => TEXT | STORED,
            SchemaWrapperFields::RepoType => STRING | STORED,

            SchemaWrapperFields::FileName => TEXT | STORED,
            SchemaWrapperFields::FilePath => TEXT | STORED,
            SchemaWrapperFields::FileExt => STRING | STORED,
            SchemaWrapperFields::FileSize => STORED.into(),
            SchemaWrapperFields::FileContent => TEXT | STORED,

            SchemaWrapperFields::LastUpdated => STORED.into(),
        }
    }
}

pub(crate) struct SimpleSchemaWrapper {
    schema: tantivy::schema::Schema,
}

impl SimpleSchemaWrapper {
    pub fn create() -> tantivy::Result<Self> {
        let schema = {
            let mut schema_builder = tantivy::schema::Schema::builder();
            let fields = vec![
                SchemaWrapperFields::RepoName,
                SchemaWrapperFields::RepoPath,
                SchemaWrapperFields::RepoType,

                SchemaWrapperFields::FileName,
                SchemaWrapperFields::FilePath,
                SchemaWrapperFields::FileExt,
                SchemaWrapperFields::FileSize,
                SchemaWrapperFields::FileContent,

                SchemaWrapperFields::LastUpdated,
            ];
            for field in fields {
                schema_builder.add_text_field(field.get_name(), field.get_type());
            }
            schema_builder.build()
        };

        Ok(SimpleSchemaWrapper {
            schema,
        })
    }

    pub fn get_schema(&self) -> &tantivy::schema::Schema {
        &self.schema
    }

    pub fn get_field(&self, field_name: SchemaWrapperFields) -> Field {
        self.schema.get_field(field_name.get_name()).unwrap()
    }

    pub fn create_document(&self, data: SchemaWrapperModel) -> tantivy::Document {
        let repo_name_field = self.get_field(SchemaWrapperFields::RepoName);
        let repo_path_field = self.get_field(SchemaWrapperFields::RepoPath);
        let repo_type_field = self.get_field(SchemaWrapperFields::RepoType);

        let name_field = self.get_field(SchemaWrapperFields::FileName);
        let path_field = self.get_field(SchemaWrapperFields::FilePath);
        let ext_field = self.get_field(SchemaWrapperFields::FileExt);
        let size_field = self.get_field(SchemaWrapperFields::FileSize);
        let content_field = self.get_field(SchemaWrapperFields::FileContent);

        let updated_field = self.get_field(SchemaWrapperFields::LastUpdated);

        return doc!(
            repo_name_field => data.repo_name,
            repo_path_field => data.repo_path,
            repo_type_field => data.repo_type,
            name_field => data.file_name,
            path_field => data.file_path,
            ext_field => data.file_ext,
            size_field => data.file_size.to_string(),
            content_field => data.file_content,
            updated_field => data.last_updated.timestamp_millis(),
        );
    }

    pub fn create_code_file_dto(&self, doc: &tantivy::Document) -> tantivy::Result<SchemaWrapperModel> {
        let repo_name = self.extract_text_field(doc, SchemaWrapperFields::RepoName)?;
        let repo_path = self.extract_text_field(doc, SchemaWrapperFields::RepoPath)?;
        let repo_type = self.extract_text_field(doc, SchemaWrapperFields::RepoType)?;
        let file_name = self.extract_text_field(doc, SchemaWrapperFields::FileName)?;
        let file_path = self.extract_text_field(doc, SchemaWrapperFields::FilePath)?;
        let file_ext = self.extract_text_field(doc, SchemaWrapperFields::FileExt)?;
        let file_size = self.extract_text_field(doc, SchemaWrapperFields::FileSize)?.parse().unwrap();
        let file_content = self.extract_text_field(doc, SchemaWrapperFields::FileContent)?;

        let last_updated = self.extract_date_field(doc, SchemaWrapperFields::LastUpdated)?;
        let last_updated = chrono::DateTime::from_timestamp_millis(last_updated.into_timestamp_millis())
            .unwrap_or_else(|| chrono::Utc::now());

        Ok(SchemaWrapperModel {
            repo_name,
            repo_path,
            repo_type,
            file_name,
            file_path,
            file_ext,
            file_size,
            file_content,
            last_updated,
        })
    }

    fn extract_date_field(&self, doc: &tantivy::Document, field_name: SchemaWrapperFields) -> tantivy::Result<tantivy::DateTime> {
        Ok(doc.get_first(self.get_field(field_name)).unwrap().as_date().unwrap().clone())
    }

    fn extract_text_field(&self, doc: &tantivy::Document, field_name: SchemaWrapperFields) -> tantivy::Result<String> {
        Ok(doc.get_first(self.get_field(field_name)).unwrap().as_text().unwrap().to_string())
    }
}
