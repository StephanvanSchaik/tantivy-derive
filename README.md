# About

tantivy-derive provides macros to generate the code to convert structs from and to `TantivyDocument` objects as well as to build the corresponding database schema to avoid having to write this boilerplate yourself.

When using tantivy, it is common to build a schema as follows:

```
let mut schema_builder = Schema::builder();

schema_builder.add_text_field("title", TEXT | STORED);
schema_builder.add_text_field("body", TEXT);

let schema = schema_builder.build();
```

It is also possible to add a document as follows:

```
let title = schema.get_field("title").unwrap();
let body = schema.get_field("body").unwrap();

let mut document = TantivyDocument::default();
document.add_text(title, "The Old Man and the Sea");
document.add_text(
    body,
    "He was an old man who fished alone in a skiff in the Gulf Stream and he had gone \
    eighty-four days now without taking a fish.",
);

index_writer.add_document(document)?;
```

However, this adds a lot of boilerplate for more complicated schemas.
While tantivy does provide a `doc!` macro to reduce the boilerplate a bit when adding documents, it does not really provide a way to easily extract the various fields from a `TantivyDocument` object.

tantivy-derive relies on the [darling](https://crates.io/crates/darling) crate to provide an experience similar to what [serde](https://crates.io/crates/serde) offers to serialize and deserialize documents.
With tantivy-derive you can represent the document above as follows:

```
use tantivy_derive::{Schema, tantivy_document};

#[tantivy_document]
#[derive(Debug)]
pub struct Document {
    #[tantivy(text)]
    pub body: String,
    #[tantivy(stored, text)]
    pub _title: String,
}
```

`#[tantivy_document]` will generate two variants of the above struct:

 * `Document` which contains all fields for building the schema and adding the document to the tantivy database.
 * `StoredDocument` which only contains the fields that are marked as stored for document retrieval.

The schema for `Document` can simply be built as follows:

```
let schema = Document::schema();
```

A document can be added as follows:

```
let document = Document {
_title: "The Old Man and the Sea".to_string(),
body: "He was an old man who fished alone in a skiff in the Gulf Stream and he had gone \
    eight-four-days now without taking a fish."
    .to_string(),
};

index_writer.add_document(document.into())?;
```

Finally, a document can be retrieved from the tantivy database and converted back into a `StoredDocument` as follows:

```
let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
let document: StoredDocument = retrieved_doc.into();
println!("{document:?}");
```
