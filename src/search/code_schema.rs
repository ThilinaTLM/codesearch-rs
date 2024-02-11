use serde::{Deserialize, Serialize};
use tantivy::doc;
use tantivy::schema::{Field, STORED, STRING, TEXT, TextOptions};

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeFileDto {
    pub repo_name: String,
    pub repo_path: String,
    pub repo_type: String,

    pub file_name: String,
    pub file_path: String,
    pub file_ext: String,
    pub file_size: u64,
    pub file_last_updated: chrono::DateTime<chrono::Utc>,
    pub file_language: String,
    pub file_content: String,
}

pub enum CodeSchemaFields {
    RepoName,
    RepoPath,
    RepoType,
    FileName,
    FilePath,
    FileExt,
    FileSize,
    FileLastUpdated,
    FileLanguage,
    FileContent,
}

impl CodeSchemaFields {
    pub fn get_name(&self) -> &str {
        match self {
            CodeSchemaFields::RepoName => "repo_name",
            CodeSchemaFields::RepoPath => "repo_path",
            CodeSchemaFields::RepoType => "repo_type",
            CodeSchemaFields::FileName => "file_name",
            CodeSchemaFields::FilePath => "file_path",
            CodeSchemaFields::FileExt => "file_ext",
            CodeSchemaFields::FileSize => "file_size",
            CodeSchemaFields::FileLastUpdated => "file_last_updated",
            CodeSchemaFields::FileLanguage => "file_language",
            CodeSchemaFields::FileContent => "file_content",
        }
    }

    pub fn get_type(&self) -> TextOptions {
        match self {
            CodeSchemaFields::RepoName => STRING | STORED,
            CodeSchemaFields::RepoPath => TEXT | STORED,
            CodeSchemaFields::RepoType => STRING | STORED,
            CodeSchemaFields::FileName => TEXT | STORED,
            CodeSchemaFields::FilePath => TEXT | STORED,
            CodeSchemaFields::FileExt => STRING | STORED,
            CodeSchemaFields::FileSize => STORED.into(),
            CodeSchemaFields::FileLastUpdated => STORED.into(),
            CodeSchemaFields::FileLanguage => STRING | STORED,
            CodeSchemaFields::FileContent => TEXT | STORED,
        }
    }
}

pub(crate) struct CodeFileSchema {
    schema: tantivy::schema::Schema,
}

impl CodeFileSchema {
    pub fn create() -> tantivy::Result<Self> {
        let schema = {
            let mut schema_builder = tantivy::schema::Schema::builder();
            let fields = vec![
                CodeSchemaFields::RepoName,
                CodeSchemaFields::RepoPath,
                CodeSchemaFields::RepoType,
                CodeSchemaFields::FileName,
                CodeSchemaFields::FilePath,
                CodeSchemaFields::FileExt,
                CodeSchemaFields::FileSize,
                CodeSchemaFields::FileLastUpdated,
                CodeSchemaFields::FileLanguage,
                CodeSchemaFields::FileContent,
            ];
            for field in fields {
                schema_builder.add_text_field(field.get_name(), field.get_type());
            }
            schema_builder.build()
        };

        Ok(CodeFileSchema {
            schema,
        })
    }

    pub fn get_schema(&self) -> &tantivy::schema::Schema {
        &self.schema
    }

    pub fn get_field(&self, field_name: CodeSchemaFields) -> Field {
        self.schema.get_field(field_name.get_name()).unwrap()
    }

    pub fn create_document(&self, data: CodeFileDto) -> tantivy::Document {
        let repo_name_field = self.get_field(CodeSchemaFields::RepoName);
        let repo_path_field = self.get_field(CodeSchemaFields::RepoPath);
        let repo_type_field = self.get_field(CodeSchemaFields::RepoType);
        let name_field = self.get_field(CodeSchemaFields::FileName);
        let path_field = self.get_field(CodeSchemaFields::FilePath);
        let ext_field = self.get_field(CodeSchemaFields::FileExt);
        let size_field = self.get_field(CodeSchemaFields::FileSize);
        let last_updated_field = self.get_field(CodeSchemaFields::FileLastUpdated);
        let language_field = self.get_field(CodeSchemaFields::FileLanguage);
        let content_field = self.get_field(CodeSchemaFields::FileContent);
        return doc!(
            repo_name_field => data.repo_name,
            repo_path_field => data.repo_path,
            repo_type_field => data.repo_type,
            name_field => data.file_name,
            path_field => data.file_path,
            ext_field => data.file_ext,
            size_field => data.file_size.to_string(),
            last_updated_field => crate::utils::convert_datetime_chrono_to_tantivy(&data.file_last_updated),
            language_field => data.file_language,
            content_field => data.file_content,
        );
    }

    pub fn create_code_file_dto(&self, doc: &tantivy::Document) -> tantivy::Result<CodeFileDto> {
        let repo_name = self.extract_text_field(doc, CodeSchemaFields::RepoName)?;
        let repo_path = self.extract_text_field(doc, CodeSchemaFields::RepoPath)?;
        let repo_type = self.extract_text_field(doc, CodeSchemaFields::RepoType)?;
        let file_name = self.extract_text_field(doc, CodeSchemaFields::FileName)?;
        let file_path = self.extract_text_field(doc, CodeSchemaFields::FilePath)?;
        let file_ext = self.extract_text_field(doc, CodeSchemaFields::FileExt)?;
        let file_size = self.extract_text_field(doc, CodeSchemaFields::FileSize)?.parse().unwrap();
        let file_language = self.extract_text_field(doc, CodeSchemaFields::FileLanguage)?;
        let file_content = self.extract_text_field(doc, CodeSchemaFields::FileContent)?;

        let file_last_updated = self.extract_date_field(doc, CodeSchemaFields::FileLastUpdated)?;
        let file_last_updated = chrono::DateTime::from_timestamp_millis(file_last_updated.into_timestamp_millis())
            .unwrap_or_else(|| chrono::Utc::now());

        Ok(CodeFileDto {
            repo_name,
            repo_path,
            repo_type,
            file_name,
            file_path,
            file_ext,
            file_size,
            file_last_updated,
            file_language,
            file_content,
        })
    }

    fn extract_date_field(&self, doc: &tantivy::Document, field_name: CodeSchemaFields) -> tantivy::Result<tantivy::DateTime> {
        Ok(doc.get_first(self.get_field(field_name)).unwrap().as_date().unwrap().clone())
    }

    fn extract_text_field(&self, doc: &tantivy::Document, field_name: CodeSchemaFields) -> tantivy::Result<String> {
        Ok(doc.get_first(self.get_field(field_name)).unwrap().as_text().unwrap().to_string())
    }
}
