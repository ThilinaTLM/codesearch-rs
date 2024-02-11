use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, STORED, STRING, TEXT, TextOptions};

use crate::search::ResultItem;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleTextDto {
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

pub enum SimpleTextIndexFieldNames {
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

impl SimpleTextIndexFieldNames {
    pub fn get_name(&self) -> &str {
        match self {
            SimpleTextIndexFieldNames::RepoName => "repo_name",
            SimpleTextIndexFieldNames::RepoPath => "repo_path",
            SimpleTextIndexFieldNames::RepoType => "repo_type",
            SimpleTextIndexFieldNames::FileName => "file_name",
            SimpleTextIndexFieldNames::FilePath => "file_path",
            SimpleTextIndexFieldNames::FileExt => "file_ext",
            SimpleTextIndexFieldNames::FileSize => "file_size",
            SimpleTextIndexFieldNames::FileLastUpdated => "file_last_updated",
            SimpleTextIndexFieldNames::FileLanguage => "file_language",
            SimpleTextIndexFieldNames::FileContent => "file_content",
        }
    }

    pub fn get_type(&self) -> TextOptions {
        match self {
            SimpleTextIndexFieldNames::RepoName => STRING | STORED,
            SimpleTextIndexFieldNames::RepoPath => TEXT | STORED,
            SimpleTextIndexFieldNames::RepoType => STRING | STORED,
            SimpleTextIndexFieldNames::FileName => TEXT | STORED,
            SimpleTextIndexFieldNames::FilePath => TEXT | STORED,
            SimpleTextIndexFieldNames::FileExt => STRING | STORED,
            SimpleTextIndexFieldNames::FileSize => STORED.into(),
            SimpleTextIndexFieldNames::FileLastUpdated => STORED.into(),
            SimpleTextIndexFieldNames::FileLanguage => STRING | STORED,
            SimpleTextIndexFieldNames::FileContent => TEXT | STORED,
            _ => unimplemented!(),
        }
    }

    pub fn from_name(name: &str) -> Option<SimpleTextIndexFieldNames> {
        match name {
            "repo_name" => Some(SimpleTextIndexFieldNames::RepoName),
            "repo_path" => Some(SimpleTextIndexFieldNames::RepoPath),
            "repo_type" => Some(SimpleTextIndexFieldNames::RepoType),
            "file_name" => Some(SimpleTextIndexFieldNames::FileName),
            "file_path" => Some(SimpleTextIndexFieldNames::FilePath),
            "file_ext" => Some(SimpleTextIndexFieldNames::FileExt),
            "file_size" => Some(SimpleTextIndexFieldNames::FileSize),
            "file_last_updated" => Some(SimpleTextIndexFieldNames::FileLastUpdated),
            "file_language" => Some(SimpleTextIndexFieldNames::FileLanguage),
            "file_content" => Some(SimpleTextIndexFieldNames::FileContent),
            _ => None,
        }
    }
}

pub(crate) struct SimpleTextIndex {
    schema: tantivy::schema::Schema,
    index: Arc<tantivy::Index>,
    index_writer: Arc<Mutex<tantivy::IndexWriter>>,
    index_reader: tantivy::IndexReader,
}

impl SimpleTextIndex {
    pub fn new(index_path: MmapDirectory) -> tantivy::Result<Self> {
        let schema = {
            let mut schema_builder = tantivy::schema::Schema::builder();
            let fields = vec![
                SimpleTextIndexFieldNames::RepoName,
                SimpleTextIndexFieldNames::RepoPath,
                SimpleTextIndexFieldNames::RepoType,
                SimpleTextIndexFieldNames::FileName,
                SimpleTextIndexFieldNames::FilePath,
                SimpleTextIndexFieldNames::FileExt,
                SimpleTextIndexFieldNames::FileSize,
                SimpleTextIndexFieldNames::FileLastUpdated,
                SimpleTextIndexFieldNames::FileLanguage,
                SimpleTextIndexFieldNames::FileContent,
            ];
            for field in fields {
                schema_builder.add_text_field(field.get_name(), field.get_type());
            }
            schema_builder.build()
        };
        let index = tantivy::Index::open_or_create(index_path, schema.clone())?;

        let index_writer = index.writer(50_000_000)?;
        let index_reader = index.reader()?;

        Ok(SimpleTextIndex {
            schema,
            index: Arc::new(index),
            index_writer: Arc::new(Mutex::from(index_writer)),
            index_reader,
        })
    }

    pub fn get_schema(&self) -> &tantivy::schema::Schema {
        &self.schema
    }

    pub fn get_field(&self, field_name: SimpleTextIndexFieldNames) -> tantivy::schema::Field {
        self.schema.get_field(field_name.get_name()).unwrap()
    }

    pub fn add_record(&self, data: SimpleTextDto) {
        let repo_name_field = self.get_field(SimpleTextIndexFieldNames::RepoName);
        let repo_path_field = self.get_field(SimpleTextIndexFieldNames::RepoPath);
        let repo_type_field = self.get_field(SimpleTextIndexFieldNames::RepoType);
        let name_field = self.get_field(SimpleTextIndexFieldNames::FileName);
        let path_field = self.get_field(SimpleTextIndexFieldNames::FilePath);
        let ext_field = self.get_field(SimpleTextIndexFieldNames::FileExt);
        let size_field = self.get_field(SimpleTextIndexFieldNames::FileSize);
        let last_updated_field = self.get_field(SimpleTextIndexFieldNames::FileLastUpdated);
        let language_field = self.get_field(SimpleTextIndexFieldNames::FileLanguage);
        let content_field = self.get_field(SimpleTextIndexFieldNames::FileContent);

        let mut index_writer = self.index_writer.lock().unwrap();
        let doc = doc!(
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
        index_writer.add_document(doc).expect("Failed to add document to index");
    }

    pub fn simple_search(&self, query: &str) -> tantivy::Result<Vec<ResultItem>> {
        let searcher = self.index_reader.searcher();

        let fields_to_search: Vec<Field> = vec![
            self.get_field(SimpleTextIndexFieldNames::RepoName),
            self.get_field(SimpleTextIndexFieldNames::FileName),
            self.get_field(SimpleTextIndexFieldNames::FilePath),
            self.get_field(SimpleTextIndexFieldNames::FileContent),
            self.get_field(SimpleTextIndexFieldNames::FileLanguage),
        ];

        let query_parser = QueryParser::for_index(&self.index, fields_to_search);
        let query = query_parser.parse_query(query)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        let mut results = Vec::new();

        for (score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            let simple_text_dto = self.doc_to_dto(&retrieved_doc)?;

            results.push(ResultItem {
                data: simple_text_dto,
                score,
            });
        }

        Ok(results)
    }

    fn doc_to_dto(&self, doc: &tantivy::Document) -> tantivy::Result<SimpleTextDto> {
        Ok(SimpleTextDto {
            repo_name: self.extract_text_field(doc, SimpleTextIndexFieldNames::RepoName)?,
            repo_path: self.extract_text_field(doc, SimpleTextIndexFieldNames::RepoPath)?,
            repo_type: self.extract_text_field(doc, SimpleTextIndexFieldNames::RepoType)?,
            file_name: self.extract_text_field(doc, SimpleTextIndexFieldNames::FileName)?,
            file_path: self.extract_text_field(doc, SimpleTextIndexFieldNames::FilePath)?,
            file_ext: self.extract_text_field(doc, SimpleTextIndexFieldNames::FileExt)?,
            file_size: self.extract_text_field(doc, SimpleTextIndexFieldNames::FileSize)?.parse().unwrap(),
            file_last_updated: self.extract_text_field(doc, SimpleTextIndexFieldNames::FileLastUpdated)?.parse().unwrap(),
            file_language: self.extract_text_field(doc, SimpleTextIndexFieldNames::FileLanguage)?,
            file_content: self.extract_text_field(doc, SimpleTextIndexFieldNames::FileContent)?,
        })
    }

    fn extract_text_field(&self, doc: &tantivy::Document, field_name: SimpleTextIndexFieldNames) -> tantivy::Result<String> {
        Ok(doc.get_first(self.get_field(field_name)).unwrap().as_text().unwrap().to_string())
    }
}
