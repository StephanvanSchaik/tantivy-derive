use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{Index, IndexWriter, ReloadPolicy, TantivyDocument};
use tantivy_derive::{Schema, tantivy_document};
use tempfile::TempDir;

#[tantivy_document]
#[derive(Debug)]
pub struct Document {
    #[tantivy(text)]
    pub body: String,
    #[tantivy(stored, text)]
    pub _title: String,
}

fn main() -> tantivy::Result<()> {
    let index_path = TempDir::new()?;

    let schema = Document::schema();

    let index = Index::create_in_dir(&index_path, schema.clone())?;

    let mut index_writer: IndexWriter = index.writer(50_000_000)?;

    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    let document = Document {
        _title: "The Old Man and the Sea".to_string(),
        body: "He was an old man who fished alone in a skiff in the Gulf Stream and he had gone \
            eight-four-days now without taking a fish."
            .to_string(),
    };

    index_writer.add_document(document.into())?;

    let document = Document {
        _title: "Of Mine and Men".to_string(),
        body: "A few miles south of Soledad, the Salinas River drops in close to the hillside \
            bank and runs deep and green. The water is warm too, for it has slipped twinkling \
            over the yellow sands in the sunlight before reaching the narrow pool. On one \
            side of the river the golden foothill slopes curve up to the strong and rocky \
            Gabilan Mountains, but on the valley side the water is lined with trees—willows \
            fresh and green with every spring, carrying in their lower leaf junctures the \
    debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
            limbs and branches that arch over the pool"
            .to_string(),
    };

    index_writer.add_document(document.into())?;

    index_writer.commit()?;

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, vec![title, body]);

    let query = query_parser.parse_query("sea whale")?;

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        let document: StoredDocument = retrieved_doc.into();
        println!("{document:?}");
    }

    Ok(())
}
