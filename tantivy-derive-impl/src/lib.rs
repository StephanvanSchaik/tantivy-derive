use darling::{ast, util, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Ident, Type};

#[derive(Debug, FromField)]
#[darling(attributes(tantivy))]
struct Field {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    coerce: bool,
    #[darling(default)]
    fast: bool,
    #[darling(default)]
    fieldnorms: bool,
    #[darling(default)]
    indexed: bool,
    #[darling(default)]
    stored: bool,
    #[darling(default)]
    store_target: Option<String>,
    #[darling(default)]
    string: bool,
    #[darling(default)]
    text: bool,
    #[darling(default)]
    fast_tokenizer: Option<String>,
    #[darling(default)]
    tokenizer: Option<String>,
    #[darling(default)]
    index_option: Option<String>,
    #[darling(default)]
    precision: Option<String>,
}

impl Field {
    fn parse(
        &self,
    ) -> (
        TokenStream,
        TokenStream,
        TokenStream,
        TokenStream,
        TokenStream,
    ) {
        let Field {
            ident,
            ty,
            coerce,
            fast,
            fieldnorms,
            indexed,
            stored,
            string,
            text,
            fast_tokenizer,
            tokenizer,
            index_option,
            precision,
            ..
        } = self;

        let name = ident.as_ref().expect("must be a named struct").to_string();
        let name = name.trim_start_matches('_');

        let count_token = quote! {
            count += <#ty>::count_fields();
        };

        let from_token = if *stored {
            quote! {
                let #ident = <#ty>::extract_from_document(&document, field_id)?;
                field_id += <#ty>::count_fields();
            }
        } else {
            quote! {
                field_id += <#ty>::count_fields();
            }
        };

        let field_token = if *stored {
            quote! { #ident, }
        } else {
            TokenStream::new()
        };

        let coerce = if *coerce {
            quote! { options.set_coerce(true); }
        } else {
            TokenStream::new()
        };

        let fast = if *fast {
            quote! { options.set_fast(true); }
        } else {
            TokenStream::new()
        };

        let fieldnorms = if *fieldnorms {
            quote! { options.set_fieldnorms(true); }
        } else {
            TokenStream::new()
        };

        let indexed = if *indexed {
            quote! { options.set_indexed(true); }
        } else {
            TokenStream::new()
        };

        let stored = if *stored {
            quote! { options.set_stored(true); }
        } else {
            TokenStream::new()
        };

        let string = if *string {
            quote! { options.set_string(true); }
        } else {
            TokenStream::new()
        };

        let text = if *text {
            quote! { options.set_text(true); }
        } else {
            TokenStream::new()
        };

        let index_option = match index_option.as_ref().map(|s| s.as_str()) {
            Some("basic") => quote! { options.set_index_option(IndexRecordOption::Basic); },
            Some("frequency") => quote! { options.set_index_option(IndexRecordOption::WithFreqs); },
            Some("frequency-and-position") => {
                quote! { options.set_index_option(IndexRecordOption::WithFreqsAndPositions); }
            }
            _ => TokenStream::new(),
        };

        let fast_tokenizer = if let Some(tokenizer) = fast_tokenizer {
            quote! { options.set_fast_tokenizer(#tokenizer); }
        } else {
            TokenStream::new()
        };

        let tokenizer = if let Some(tokenizer) = tokenizer {
            quote! { options.set_tokenizer(#tokenizer); }
        } else {
            TokenStream::new()
        };

        let precision = match precision.as_ref().map(|s| s.as_str()) {
            Some("seconds") => quote! { options.set_precision(DateTimePrecision::Seconds); },
            Some("milliseconds") => {
                quote! { options.set_precision(DateTimePrecision::Milliseconds); }
            }
            Some("microseconds") => {
                quote! { options.set_precision(DateTimePrecision::Microseconds); }
            }
            Some("nanoseconds") => {
                quote! { options.set_precision(DateTimePrecision::Nanoseconds); }
            }
            _ => TokenStream::new(),
        };

        let schema_token = quote! {
            let mut options: tantivy_derive::FieldOptions = Default::default();
            #coerce
            #fast
            #fieldnorms
            #indexed
            #stored
            #string
            #text
            #fast_tokenizer
            #tokenizer
            #index_option
            #precision
            <#ty>::add_field(builder, #name, options);
        };

        let into_token = quote! {
            <#ty>::insert_into_document(document, field_id, &value.#ident);
            field_id += <#ty>::count_fields();
        };

        (
            schema_token,
            count_token,
            from_token,
            field_token,
            into_token,
        )
    }

    fn parse_stored(&self) -> TokenStream {
        let Field {
            ident,
            ty,
            stored,
            store_target,
            ..
        } = self;

        if *stored {
            if let Some(target) = store_target {
                quote! {
                    #ident: #target,
                }
            } else {
                quote! {
                    #ident: #ty,
                }
            }
        } else {
            TokenStream::new()
        }
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tantivy), supports(struct_named))]
struct Document {
    ident: Ident,
    generics: syn::Generics,
    data: ast::Data<util::Ignored, Field>,
}

impl ToTokens for Document {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let name = &self.ident;
        let stored_name = format_ident!("Stored{name}");

        let fields = self
            .data
            .as_ref()
            .take_struct()
            .expect("must be struct")
            .fields;

        let mut schema_tokens = Vec::with_capacity(fields.len());
        let mut count_tokens = Vec::with_capacity(fields.len());
        let mut from_tokens = Vec::with_capacity(fields.len());
        let mut field_tokens = Vec::with_capacity(fields.len());
        let mut into_tokens = Vec::with_capacity(fields.len());

        for field in fields {
            let (schema_token, count_token, from_token, field_token, into_token) = field.parse();

            schema_tokens.push(schema_token);
            count_tokens.push(count_token);
            from_tokens.push(from_token);
            field_tokens.push(field_token);
            into_tokens.push(into_token);
        }

        tokens.extend(quote! {
            impl #impl_generics tantivy_derive::Field for #name #ty_generics #where_clause {
                type Target = #stored_name;

                fn add_field(builder: &mut tantivy::schema::SchemaBuilder, name: &str, options: tantivy_derive::FieldOptions) {
                    use tantivy::schema::*;
                    use tantivy_derive::Field as _;

                    #(
                        #schema_tokens
                    )*
                }

                fn count_fields() -> u32 {
                    let mut count = 0;

                    #(
                        #count_tokens
                    )*

                    count
                }

                fn insert_into_document(
                    document: &mut tantivy::schema::TantivyDocument,
                    mut field_id: u32,
                    value: &Self,
                ) {
                    #(
                        #into_tokens
                    )*
                }
            }

            impl #impl_generics tantivy_derive::Extractable for #name #ty_generics #where_clause {
                fn extract_from_document(
                    document: &tantivy::schema::TantivyDocument,
                    mut field_id: u32,
                ) -> Option<Self::Target> {
                    use tantivy_derive::{Extractable as _, Field as _};

                    #(
                        #from_tokens
                    )*

                    Some(Self::Target {
                        #(
                            #field_tokens
                        )*
                    })
                }
            }

            impl #impl_generics std::convert::From<#name> for tantivy::schema::TantivyDocument #ty_generics #where_clause {
                fn from(value: #name) -> tantivy::schema::TantivyDocument {
                    use tantivy_derive::Field as _;

                    let mut document = tantivy::schema::TantivyDocument::new();
                    #name::insert_into_document(&mut document, 0, &value);
                    document
                }
            }

            impl #impl_generics std::convert::From<tantivy::schema::TantivyDocument> for #stored_name #ty_generics #where_clause {
                fn from(document: tantivy::schema::TantivyDocument) -> Self {
                    use tantivy_derive::{Extractable as _, Field as _};

                    #name::extract_from_document(&document, 0).expect("missing field")
                }
            }

            impl #impl_generics tantivy_derive::Schema for #name #ty_generics #where_clause {
                fn schema() -> tantivy::schema::Schema {
                    use tantivy::schema::*;
                    use tantivy_derive::Field as _;

                    let mut builder = Schema::builder();
                    Self::add_field(&mut builder, "", Default::default());
                    builder.build()
                }
            }
        });
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(tantivy),
    supports(struct_named),
    forward_attrs(allow, cfg, derive)
)]
struct StoredDocument {
    ident: Ident,
    vis: syn::Visibility,
    data: ast::Data<util::Ignored, Field>,
    attrs: Vec<syn::Attribute>,
}

impl ToTokens for StoredDocument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let vis = &self.vis;

        let fields = self
            .data
            .as_ref()
            .take_struct()
            .expect("must be struct")
            .fields;

        let mut field_tokens = Vec::with_capacity(fields.len());

        for field in fields {
            let token = field.parse_stored();

            field_tokens.push(token);
        }

        let attrs: Vec<TokenStream> = self.attrs.iter().map(|attr| quote! { #attr }).collect();

        tokens.extend(quote! {
            #(
                #attrs
            )*
            #vis struct #name {
                #(
                    #field_tokens
                )*
            }
        });
    }
}

#[proc_macro_derive(Document, attributes(tantivy))]
pub fn derive_document(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let receiver = Document::from_derive_input(&input).expect("cannot parse");
    quote!(#receiver).into()
}

#[proc_macro_attribute]
pub fn tantivy_document(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use syn::{punctuated::Punctuated, Meta};
    let args = parse_macro_input!(args with Punctuated::<Meta, syn::Token![,]>::parse_terminated);
    let mut input = parse_macro_input!(input as DeriveInput);
    let mut struct_name = format_ident!("Stored{}", input.ident);

    for arg in &args {
        let Ok(arg) = arg.require_name_value() else {
            continue;
        };
        let Some(ident) = arg.path.get_ident() else {
            continue;
        };
        let name = ident.to_string();

        if name != "name" {
            continue;
        }

        let syn::Expr::Lit(ref value) = arg.value else {
            continue;
        };
        let syn::Lit::Str(ref value) = value.lit else {
            continue;
        };

        struct_name = format_ident!("{}", value.value());
    }

    let original = quote! { #input };

    input.ident = struct_name;
    let receiver = StoredDocument::from_derive_input(&input).expect("cannot parse");

    quote! {
        #[derive(tantivy_derive::Document)]
        #original
        #receiver
    }
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::StoredDocument;
        use darling::FromDeriveInput as _;

        let input = r#"#[derive(Debug)]
        pub struct Document {
            #[tantivy(stored, text)]
            title: String,
            #[tantivy(text)]
            body: String,
        }"#;
        let parsed = syn::parse_str(input).unwrap();
        let receiver = StoredDocument::from_derive_input(&parsed).unwrap();
        let tokens = quote::quote!(#receiver);

        panic!("{}", tokens.to_string());
    }
}
