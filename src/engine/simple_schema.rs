use serde::{Deserialize, Serialize};
use tantivy::doc;
use tantivy::schema::{Field, STORED, STRING, TEXT, TextOptions};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleSchemaModel {
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

pub enum SimpleSchemaFields {
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

impl SimpleSchemaFields {
    pub fn get_name(&self) -> &str {
        match self {
            SimpleSchemaFields::RepoName => "repo_name",
            SimpleSchemaFields::RepoPath => "repo_path",
            SimpleSchemaFields::RepoType => "repo_type",

            SimpleSchemaFields::FileName => "file_name",
            SimpleSchemaFields::FilePath => "file_path",
            SimpleSchemaFields::FileExt => "file_ext",
            SimpleSchemaFields::FileSize => "file_size",
            SimpleSchemaFields::FileContent => "file_content",

            SimpleSchemaFields::LastUpdated => "last_updated",
        }
    }

    pub fn get_type(&self) -> TextOptions {
        match self {
            SimpleSchemaFields::RepoName => STRING | STORED,
            SimpleSchemaFields::RepoPath => TEXT | STORED,
            SimpleSchemaFields::RepoType => STRING | STORED,

            SimpleSchemaFields::FileName => TEXT | STORED,
            SimpleSchemaFields::FilePath => TEXT | STORED,
            SimpleSchemaFields::FileExt => STRING | STORED,
            SimpleSchemaFields::FileSize => STORED.into(),
            SimpleSchemaFields::FileContent => TEXT | STORED,

            SimpleSchemaFields::LastUpdated => STORED.into(),
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
                SimpleSchemaFields::RepoName,
                SimpleSchemaFields::RepoPath,
                SimpleSchemaFields::RepoType,

                SimpleSchemaFields::FileName,
                SimpleSchemaFields::FilePath,
                SimpleSchemaFields::FileExt,
                SimpleSchemaFields::FileSize,
                SimpleSchemaFields::FileContent,

                SimpleSchemaFields::LastUpdated,
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

    pub fn get_field(&self, field_name: SimpleSchemaFields) -> Field {
        self.schema.get_field(field_name.get_name()).unwrap()
    }

    pub fn create_document(&self, data: SimpleSchemaModel) -> tantivy::Document {
        let repo_name_field = self.get_field(SimpleSchemaFields::RepoName);
        let repo_path_field = self.get_field(SimpleSchemaFields::RepoPath);
        let repo_type_field = self.get_field(SimpleSchemaFields::RepoType);

        let name_field = self.get_field(SimpleSchemaFields::FileName);
        let path_field = self.get_field(SimpleSchemaFields::FilePath);
        let ext_field = self.get_field(SimpleSchemaFields::FileExt);
        let size_field = self.get_field(SimpleSchemaFields::FileSize);
        let content_field = self.get_field(SimpleSchemaFields::FileContent);

        let updated_field = self.get_field(SimpleSchemaFields::LastUpdated);
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

    pub fn create_code_file_dto(&self, doc: &tantivy::Document) -> tantivy::Result<SimpleSchemaModel> {
        let repo_name = self.extract_text_field(doc, SimpleSchemaFields::RepoName)?;
        let repo_path = self.extract_text_field(doc, SimpleSchemaFields::RepoPath)?;
        let repo_type = self.extract_text_field(doc, SimpleSchemaFields::RepoType)?;
        let file_name = self.extract_text_field(doc, SimpleSchemaFields::FileName)?;
        let file_path = self.extract_text_field(doc, SimpleSchemaFields::FilePath)?;
        let file_ext = self.extract_text_field(doc, SimpleSchemaFields::FileExt)?;
        let file_size = self.extract_text_field(doc, SimpleSchemaFields::FileSize)?.parse().unwrap();
        let file_content = self.extract_text_field(doc, SimpleSchemaFields::FileContent)?;

        let last_updated = self.extract_date_field(doc, SimpleSchemaFields::LastUpdated)?;
        let last_updated = chrono::DateTime::from_timestamp_millis(last_updated.into_timestamp_millis())
            .unwrap_or_else(|| chrono::Utc::now());

        Ok(SimpleSchemaModel {
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

    fn extract_date_field(&self, doc: &tantivy::Document, field_name: SimpleSchemaFields) -> tantivy::Result<tantivy::DateTime> {
        Ok(doc.get_first(self.get_field(field_name)).unwrap().as_date().unwrap().clone())
    }

    fn extract_text_field(&self, doc: &tantivy::Document, field_name: SimpleSchemaFields) -> tantivy::Result<String> {
        Ok(doc.get_first(self.get_field(field_name)).unwrap().as_text().unwrap().to_string())
    }
}
