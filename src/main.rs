use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use rayon::prelude::*;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::doc;
use std::fs;
use std::io::{self};


fn main() -> tantivy::Result<()> {
    let path = "/home/tlm/Projects/Enactor/2_7_644";
    let skip_patterns = vec!["node_modules", "target", ".git", ".svn"];

    let (index, schema) = create_index()?;

    scan_directories(path, &skip_patterns, &index, &schema)?;

    let reader = index.reader()?;
    let searcher = reader.searcher();
    let query_parser = tantivy::query::QueryParser::for_index(&index, vec![schema.get_field("content").unwrap()]);

    loop {
        println!("Enter your search query (type \\exit to quit):");
        let mut query_str = String::new();
        io::stdin().read_line(&mut query_str).expect("Failed to read line");
        query_str = query_str.trim().to_string(); // Trim the newline character

        if query_str == "\\exit" {
            break;
        }

        let query = match query_parser.parse_query(&query_str) {
            Ok(q) => q,
            Err(e) => {
                println!("Error parsing query: {}", e);
                continue;
            }
        };

        let top_docs = match searcher.search(&query, &tantivy::collector::TopDocs::with_limit(10)) {
            Ok(docs) => docs,
            Err(e) => {
                println!("Error executing search: {}", e);
                continue;
            }
        };

        if top_docs.is_empty() {
            println!("No results found.");
        } else {
            for (_score, doc_address) in top_docs {
                let retrieved_doc = searcher.doc(doc_address)?;
                println!("{:?}", schema.to_json(&retrieved_doc));
            }
        }

        println!(); // Print a newline for spacing
    }

    Ok(())
}

fn create_index() -> tantivy::Result<(Index, Schema)> {
    let index_path = "/tmp/tantivy_index";

    // make sure the index directory is empty
    let _ = fs::remove_dir_all(index_path);
    fs::create_dir(index_path)?;

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("path", TEXT | STORED);
    schema_builder.add_text_field("content", TEXT | STORED);
    let schema = schema_builder.build();

    let index = Index::create_in_dir(&index_path, schema.clone())?;

    Ok((index, schema))
}

fn scan_directories<P: AsRef<Path>>(path: P, skip_patterns: &[&str], index: &Index, schema: &Schema) -> tantivy::Result<()> {
    let mut index_writer = index.writer(50_000_000)?;

    let path_field = schema.get_field("path").unwrap();
    let content_field = schema.get_field("content").unwrap();

    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !should_skip(e, skip_patterns))
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "java"))
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|entry| {
            let path = entry.path();
            if path.is_file() {
                if let Ok(content) = fs::read_to_string(path) {
                    index_writer.add_document(doc!(
                        path_field => path.to_str().unwrap(),
                        content_field => content
                    )).unwrap();
                }
            }
        });

    // Committing outside the parallel iterator to ensure thread safety
    index_writer.commit()?;

    Ok(())
}

fn should_skip(entry: &DirEntry, skip_patterns: &[&str]) -> bool {
    entry.file_name().to_str().map(|s| skip_patterns.contains(&s)).unwrap_or(false)
}
