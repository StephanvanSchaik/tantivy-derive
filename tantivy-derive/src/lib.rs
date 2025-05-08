mod options;

use std::net::Ipv6Addr;
use tantivy::schema::*;
pub use tantivy_derive_impl::{tantivy_document, Document};

pub use crate::options::FieldOptions;

pub trait Field: Sized {
    type Target;

    fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions);
    fn count_fields() -> u32 {
        1
    }
    fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self);
}

pub trait Mappable: Field {
    fn map_value(value: &OwnedValue) -> Option<Self::Target>;
}

pub trait Extractable: Field {
    fn extract_from_document(document: &TantivyDocument, field_id: u32) -> Option<Self::Target>;
}

pub trait Schema {
    fn schema() -> tantivy::schema::Schema;
}

impl<T> Extractable for T
where
    T: Mappable,
{
    fn extract_from_document(document: &TantivyDocument, field_id: u32) -> Option<Self::Target> {
        let field = tantivy::schema::Field::from_field_id(field_id);

        document
            .get_first(field)
            .and_then(|v| Self::map_value(&v.into()))
    }
}

impl Field for bool {
    type Target = Self;

    fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
        let options: NumericOptions = options.into();
        builder.add_bool_field(name, options);
    }

    fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
        let field = tantivy::schema::Field::from_field_id(field_id);

        document.add_bool(field, *value);
    }
}

impl Mappable for bool {
    fn map_value(value: &OwnedValue) -> Option<Self::Target> {
        value.as_bool()
    }
}

impl Field for u64 {
    type Target = Self;

    fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
        let options: NumericOptions = options.into();
        builder.add_u64_field(name, options);
    }

    fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
        let field = tantivy::schema::Field::from_field_id(field_id);

        document.add_u64(field, *value);
    }
}

impl Mappable for u64 {
    fn map_value(value: &OwnedValue) -> Option<Self::Target> {
        value.as_u64()
    }
}

impl Field for i64 {
    type Target = Self;

    fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
        let options: NumericOptions = options.into();
        builder.add_u64_field(name, options);
    }

    fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
        let field = tantivy::schema::Field::from_field_id(field_id);

        document.add_i64(field, *value);
    }
}

impl Mappable for i64 {
    fn map_value(value: &OwnedValue) -> Option<Self::Target> {
        value.as_i64()
    }
}

impl Field for f64 {
    type Target = Self;

    fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
        let options: NumericOptions = options.into();
        builder.add_f64_field(name, options);
    }

    fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
        let field = tantivy::schema::Field::from_field_id(field_id);

        document.add_f64(field, *value);
    }
}

impl Mappable for f64 {
    fn map_value(value: &OwnedValue) -> Option<Self::Target> {
        value.as_f64()
    }
}

impl Field for String {
    type Target = Self;

    fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
        let options: TextOptions = options.into();
        builder.add_text_field(name, options);
    }

    fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
        let field = tantivy::schema::Field::from_field_id(field_id);

        document.add_text(field, value.as_str());
    }
}

impl Mappable for String {
    fn map_value(value: &OwnedValue) -> Option<Self::Target> {
        value.as_str().map(|s| s.to_string())
    }
}

impl Field for Facet {
    type Target = String;

    fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
        let options: FacetOptions = options.into();
        builder.add_facet_field(name, options);
    }

    fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
        let field = tantivy::schema::Field::from_field_id(field_id);

        document.add_facet(field, value.clone());
    }
}

impl Mappable for Facet {
    fn map_value(value: &OwnedValue) -> Option<Self::Target> {
        Some(value.as_facet()?.to_string())
    }
}

impl Field for Ipv6Addr {
    type Target = Self;

    fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
        let options: IpAddrOptions = options.into();
        builder.add_ip_addr_field(name, options);
    }

    fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
        let field = tantivy::schema::Field::from_field_id(field_id);

        document.add_ip_addr(field, *value);
    }
}

impl Mappable for Ipv6Addr {
    fn map_value(value: &OwnedValue) -> Option<Self::Target> {
        value.as_ip_addr()
    }
}

impl<T: Mappable> Field for Option<T> {
    type Target = Self;

    fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
        T::add_field(builder, name, options);
    }

    fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
        if let Some(value) = value {
            T::insert_into_document(document, field_id, value);
        }
    }
}

impl<T: Mappable<Target = T>> Extractable for Option<T> {
    fn extract_from_document(document: &TantivyDocument, field_id: u32) -> Option<Self::Target> {
        let field = tantivy::schema::Field::from_field_id(field_id);

        Some(document.get_first(field).and_then(|v| T::map_value(&v.into())))
    }
}

impl<T: Mappable> Field for Vec<T>
where
    std::vec::Vec<T>: FromIterator<<T as Field>::Target>,
{
    type Target = Self;

    fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
        T::add_field(builder, name, options);
    }

    fn insert_into_document(document: &mut TantivyDocument, mut field_id: u32, value: &Self) {
        for value in value {
            T::insert_into_document(document, field_id, value);
            field_id += 1;
        }
    }
}

impl<T: Mappable> Extractable for Vec<T>
where
    std::vec::Vec<T>: FromIterator<<T as Field>::Target>,
{
    fn extract_from_document(document: &TantivyDocument, field_id: u32) -> Option<Self::Target> {
        let field = tantivy::schema::Field::from_field_id(field_id);

        document.get_all(field).map(|v| T::map_value(&v.into())).collect()
    }
}

#[cfg(feature = "bytes")]
mod bytes {
    use crate::{Field, FieldOptions, Mappable};
    use bytes::Bytes;
    use tantivy::schema::*;

    impl Field for Bytes {
        type Target = Self;

        fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
            let options: BytesOptions = options.into();
            builder.add_bytes_field(name, options);
        }

        fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
            let field = tantivy::schema::Field::from_field_id(field_id);

            document.add_bytes(field, &value[..]);
        }
    }

    impl Mappable for Bytes {
        fn map_value(value: &OwnedValue) -> Option<Self::Target> {
            value.as_bytes().map(|bytes| Bytes::from(bytes.to_vec()))
        }
    }
}

#[cfg(feature = "chrono")]
mod chrono {
    use crate::{Field, FieldOptions, Mappable};
    use chrono::{DateTime, NaiveDate, Utc};
    use tantivy::schema::*;

    impl Field for DateTime<Utc> {
        type Target = Self;

        fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
            let options: DateOptions = options.into();
            builder.add_date_field(name, options);
        }

        fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
            let field = tantivy::schema::Field::from_field_id(field_id);

            let nanos = value.timestamp_nanos_opt().unwrap_or(0);
            let value = tantivy::DateTime::from_timestamp_nanos(nanos);
            document.add_date(field, value);
        }
    }

    impl Mappable for DateTime<Utc> {
        fn map_value(value: &OwnedValue) -> Option<Self::Target> {
            value
                .as_datetime()
                .map(|v| DateTime::from_timestamp_nanos(v.into_timestamp_nanos()))
        }
    }

    impl Field for NaiveDate {
        type Target = Self;

        fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
            let options: DateOptions = options.into();
            builder.add_date_field(name, options);
        }

        fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
            let field = tantivy::schema::Field::from_field_id(field_id);

            let value = value.and_hms_opt(0, 0, 0).unwrap().and_utc();
            let nanos = value.timestamp_nanos_opt().unwrap_or(0);
            let value = tantivy::DateTime::from_timestamp_nanos(nanos);
            document.add_date(field, value);
        }
    }

    impl Mappable for NaiveDate {
        fn map_value(value: &OwnedValue) -> Option<Self::Target> {
            value
                .as_datetime()
                .map(|v| DateTime::from_timestamp_nanos(v.into_timestamp_nanos()).date_naive())
        }
    }
}

#[cfg(feature = "decimal")]
mod decimal {
    use crate::{Field, FieldOptions, Mappable};
    use rust_decimal::Decimal;
    use tantivy::schema::*;

    impl Field for Decimal {
        type Target = Self;

        fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
            let options: BytesOptions = options.into();
            builder.add_bytes_field(name, options);
        }

        fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
            let field = tantivy::schema::Field::from_field_id(field_id);
            let slice = Decimal::serialize(value);

            document.add_bytes(field, &slice);
        }
    }

    impl Mappable for Decimal {
        fn map_value(value: &OwnedValue) -> Option<Self::Target> {
            value
                .as_bytes()
                .and_then(|bytes| {
                    if bytes.len() != 16 {
                        return None;
                    }

                    let mut slice = [0u8; 16];
                    slice.copy_from_slice(&bytes[..16]);

                    Some(Decimal::deserialize(slice))
                })
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[cfg_attr(
        feature = "serde",
        derive(serde::Deserialize, serde::Serialize),
        serde(transparent)
    )]
    pub struct FixedDecimal<const N: u32>(pub Decimal);

    impl<const N: u32> Field for FixedDecimal<N> {
        type Target = Self;

        fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
            let options: NumericOptions = options.into();
            builder.add_i64_field(name, options);
        }

        fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
            let field = tantivy::schema::Field::from_field_id(field_id);
            let mut value = value.0;
            value.rescale(N);
            let value = value.mantissa() as i64;

            document.add_i64(field, value);
        }
    }

    impl<const N: u32> Mappable for FixedDecimal<N> {
        fn map_value(value: &OwnedValue) -> Option<Self::Target> {
            value
                .as_i64()
                .map(|value| FixedDecimal(Decimal::from_i128_with_scale(value as i128, N)))
        }
    }
}

#[cfg(feature = "decimal")]
pub use decimal::FixedDecimal;

#[cfg(feature = "jiff")]
mod jiff {
    use crate::{Field, FieldOptions, Mappable};
    use jiff::Timestamp;
    use tantivy::schema::*;

    impl Field for Timestamp {
        type Target = Self;

        fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
            let options: DateOptions = options.into();
            builder.add_date_field(name, options);
        }

        fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
            let field = tantivy::schema::Field::from_field_id(field_id);

            let nanos = value.as_nanosecond() as i64;
            let value = tantivy::DateTime::from_timestamp_nanos(nanos);
            document.add_date(field, value);
        }
    }

    impl Mappable for Timestamp {
        fn map_value(value: &OwnedValue) -> Option<Self::Target> {
            value
                .as_datetime()
                .and_then(|v| Timestamp::from_nanosecond(v.into_timestamp_nanos() as i128).ok())
        }
    }
}

#[cfg(feature = "url")]
mod url {
    use crate::{Field, FieldOptions, Mappable};
    use std::str::FromStr as _;
    use tantivy::schema::*;
    use url::Url;

    impl Field for Url {
        type Target = Self;

        fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
            let options: TextOptions = options.into();
            builder.add_text_field(name, options);
        }

        fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
            let field = tantivy::schema::Field::from_field_id(field_id);

            let value = value.to_string();
            document.add_text(field, value);
        }
    }

    impl Mappable for Url {
        fn map_value(value: &OwnedValue) -> Option<Self::Target> {
            value.as_str().and_then(|v| Url::from_str(v).ok())
        }
    }
}

#[cfg(feature = "uuid")]
mod uuid {
    use crate::{Field, FieldOptions, Mappable};
    use tantivy::schema::*;
    use uuid::Uuid;

    impl Field for Uuid {
        type Target = Uuid;

        fn add_field(builder: &mut SchemaBuilder, name: &str, options: FieldOptions) {
            let options: TextOptions = options.into();
            builder.add_text_field(name, options);
        }

        fn insert_into_document(document: &mut TantivyDocument, field_id: u32, value: &Self) {
            let field = tantivy::schema::Field::from_field_id(field_id);

            let value = value.to_string();
            document.add_text(field, value);
        }
    }

    impl Mappable for Uuid {
        fn map_value(value: &OwnedValue) -> Option<Self::Target> {
            value.as_str().and_then(|v| Uuid::parse_str(v).ok())
        }
    }
}
