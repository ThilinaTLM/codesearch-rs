use std::fmt;
use std::fmt::Error;

use tantivy::query::QueryParserError;
use tokio::task;

#[derive(Debug)]
pub struct SearchError {
    pub(crate) error: String,
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SearchError: {}", self.error)
    }
}

impl From<QueryParserError> for SearchError {
    fn from(err: QueryParserError) -> Self {
        SearchError {
            error: format!("{}", err),
        }
    }
}

impl From<Error> for SearchError {
    fn from(err: Error) -> Self {
        SearchError {
            error: format!("{}", err),
        }
    }
}

impl From<walkdir::Error> for SearchError {
    fn from(err: walkdir::Error) -> Self {
        SearchError {
            error: format!("{}", err),
        }
    }
}

impl From<std::io::Error> for SearchError {
    fn from(err: std::io::Error) -> Self {
        SearchError {
            error: format!("{}", err),
        }
    }
}

impl From<tantivy::TantivyError> for SearchError {
    fn from(err: tantivy::TantivyError) -> Self {
        SearchError {
            error: format!("{}", err),
        }
    }
}

impl From<task::JoinError> for SearchError {
    fn from(err: task::JoinError) -> Self {
        SearchError {
            error: format!("{}", err),
        }
    }
}
