use tantivy::schema::*;

#[derive(Clone, Debug, Default)]
pub struct FieldOptions {
    pub(crate) coerce: bool,
    pub(crate) fast: bool,
    pub(crate) fieldnorms: bool,
    pub(crate) indexed: bool,
    pub(crate) stored: bool,
    pub(crate) string: bool,
    pub(crate) text: bool,
    pub(crate) fast_tokenizer: Option<String>,
    pub(crate) tokenizer: Option<String>,
    pub(crate) index_option: Option<IndexRecordOption>,
    pub(crate) precision: Option<DateTimePrecision>,
}

impl FieldOptions {
    pub fn set_coerce(&mut self, value: bool) {
        self.coerce = value;
    }

    pub fn set_fast(&mut self, value: bool) {
        self.fast = value;
    }

    pub fn set_fieldnorms(&mut self, value: bool) {
        self.fieldnorms = value;
    }

    pub fn set_indexed(&mut self, value: bool) {
        self.indexed = value;
    }

    pub fn set_stored(&mut self, value: bool) {
        self.stored = value;
    }

    pub fn set_string(&mut self, value: bool) {
        self.string = value;
    }

    pub fn set_text(&mut self, value: bool) {
        self.text = value;
    }

    pub fn set_fast_tokenizer<S: ToString>(&mut self, tokenizer: S) {
        self.fast_tokenizer = Some(tokenizer.to_string());
    }

    pub fn set_tokenizer<S: ToString>(&mut self, tokenizer: S) {
        self.tokenizer = Some(tokenizer.to_string());
    }

    pub fn set_index_option(&mut self, index_option: IndexRecordOption) {
        self.index_option = Some(index_option);
    }

    pub fn set_precision(&mut self, precision: DateTimePrecision) {
        self.precision = Some(precision);
    }
}

impl From<FieldOptions> for BytesOptions {
    fn from(value: FieldOptions) -> BytesOptions {
        let mut options: BytesOptions = Default::default();

        if value.fast {
            options = options.set_fast();
        }

        if value.fieldnorms {
            options = options.set_fieldnorms();
        }

        if value.indexed {
            options = options.set_indexed();
        }

        if value.stored {
            options = options.set_stored();
        }

        options
    }
}

impl From<FieldOptions> for DateOptions {
    fn from(value: FieldOptions) -> DateOptions {
        let mut options: DateOptions = Default::default();

        if value.fast {
            options = options.set_fast();
        }

        if value.fieldnorms {
            options = options.set_fieldnorm();
        }

        if value.indexed {
            options = options.set_indexed();
        }

        if value.stored {
            options = options.set_stored();
        }

        if let Some(precision) = value.precision {
            options = options.set_precision(precision);
        }

        options
    }
}

impl From<FieldOptions> for FacetOptions {
    fn from(value: FieldOptions) -> FacetOptions {
        let mut options: FacetOptions = Default::default();

        if value.stored {
            options = options.set_stored();
        }

        options
    }
}

impl From<FieldOptions> for IpAddrOptions {
    fn from(value: FieldOptions) -> IpAddrOptions {
        let mut options = IpAddrOptions::default();

        if value.fast {
            options = options.set_fast();
        }

        if value.fieldnorms {
            options = options.set_fieldnorms();
        }

        if value.indexed {
            options = options.set_indexed();
        }

        if value.stored {
            options = options.set_stored();
        }

        options
    }
}

impl From<FieldOptions> for NumericOptions {
    fn from(value: FieldOptions) -> NumericOptions {
        let mut options: NumericOptions = Default::default();

        if value.coerce {
            options = options.set_coerce();
        }

        if value.fast {
            options = options.set_fast();
        }

        if value.fieldnorms {
            options = options.set_fieldnorm();
        }

        if value.indexed {
            options = options.set_indexed();
        }

        if value.stored {
            options = options.set_stored();
        }

        options
    }
}

impl From<FieldOptions> for TextOptions {
    fn from(value: FieldOptions) -> TextOptions {
        let mut options: TextOptions = Default::default();

        if value.string {
            options = STRING;
        }

        if value.text {
            options = TEXT;
        }

        if value.coerce {
            options = options.set_coerce();
        }

        if value.fast || value.fast_tokenizer.is_some() {
            options = options.set_fast(value.fast_tokenizer.as_deref());
        }

        if value.stored {
            options = options.set_stored();
        }

        if value.indexed || value.tokenizer.is_some() || value.index_option.is_some() {
            let mut indexing = TextFieldIndexing::default();

            if let Some(ref tokenizer) = value.tokenizer {
                indexing = indexing.set_tokenizer(tokenizer);
            }

            if let Some(index_option) = value.index_option {
                indexing = indexing.set_index_option(index_option);
            }

            if value.fieldnorms {
                indexing = indexing.set_fieldnorms(true);
            }

            options = options.set_indexing_options(indexing);
        }

        options
    }
}
