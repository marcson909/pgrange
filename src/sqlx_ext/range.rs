use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgTypeKind, PgValueRef};
use sqlx::{Decode, Encode, Postgres, Type};
use std::fmt::Debug;

use crate::PgRange;
use bitflags::bitflags;
// https://github.com/postgres/postgres/blob/2f48ede080f42b97b594fb14102c82ca1001b80c/src/include/utils/rangetypes.h#L35-L44
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct RangeFlags: u8 {
        const EMPTY = 0x01;
        const LB_INC = 0x02;
        const UB_INC = 0x04;
        const LB_INF = 0x08;
        const UB_INF = 0x10;
        const LB_NULL = 0x20; // not used
        const UB_NULL = 0x40; // not used
        const CONTAIN_EMPTY = 0x80; // internal
    }
}

impl<T> From<PgRange<T>> for sqlx::postgres::types::PgRange<T> {
    fn from(v: PgRange<T>) -> Self {
        Self {
            start: v.start,
            end: v.end,
        }
    }
}

impl<T> From<sqlx::postgres::types::PgRange<T>> for PgRange<T> {
    fn from(v: sqlx::postgres::types::PgRange<T>) -> Self {
        Self {
            start: v.start,
            end: v.end,
        }
    }
}

impl Type<Postgres> for PgRange<bool> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(r#""_boolrange""#)
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<bool>(ty)
    }
}

impl Type<Postgres> for PgRange<i8> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(r#""_int1range""#)
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<i8>(ty)
    }
}

impl Type<Postgres> for PgRange<i16> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(r#""_int2range""#)
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<i16>(ty)
    }
}

impl Type<Postgres> for PgRange<i32> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("int4range")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<i32>(ty)
    }
}

impl Type<Postgres> for PgRange<f32> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(r#""_float4range""#)
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<f32>(ty)
    }
}

impl Type<Postgres> for PgRange<f64> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(r#""_float8range""#)
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<f64>(ty)
    }
}

impl Type<Postgres> for PgRange<i64> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("int8range")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<i64>(ty)
    }
}

#[cfg(feature = "with-bigdecimal")]
impl Type<Postgres> for PgRange<bigdecimal::BigDecimal> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("numrange")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<bigdecimal::BigDecimal>(ty)
    }
}

#[cfg(feature = "with-rust_decimal")]
impl Type<Postgres> for PgRange<rust_decimal::Decimal> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("numrange")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<rust_decimal::Decimal>(ty)
    }
}

#[cfg(feature = "with-chrono")]
impl Type<Postgres> for PgRange<chrono::NaiveDate> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("daterange")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<chrono::NaiveDate>(ty)
    }
}

#[cfg(feature = "with-chrono")]
impl Type<Postgres> for PgRange<chrono::NaiveTime> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(r#""_naivetimerange""#)
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<chrono::NaiveTime>(ty)
    }
}

#[cfg(feature = "with-chrono")]
impl Type<Postgres> for PgRange<chrono::NaiveDateTime> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("tsrange")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<chrono::NaiveDateTime>(ty)
    }
}

#[cfg(feature = "with-chrono")]
impl<Tz: chrono::TimeZone> Type<Postgres> for PgRange<chrono::DateTime<Tz>> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("tstzrange")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<chrono::DateTime<Tz>>(ty)
    }
}

#[cfg(feature = "time")]
impl Type<Postgres> for PgRange<time::Date> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("daterange")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<time::Date>(ty)
    }
}

#[cfg(feature = "time")]
impl Type<Postgres> for PgRange<time::Time> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(r#""_naivetimerange""#)
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<time::Time>(ty)
    }
}

#[cfg(feature = "time")]
impl Type<Postgres> for PgRange<time::PrimitiveDateTime> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("tsrange")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<time::PrimitiveDateTime>(ty)
    }
}

#[cfg(feature = "time")]
impl Type<Postgres> for PgRange<time::OffsetDateTime> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("tstzrange")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        range_compatible::<time::OffsetDateTime>(ty)
    }
}

impl PgHasArrayType for PgRange<i32> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("int4range")
    }
}

impl PgHasArrayType for PgRange<i64> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("int8range")
    }
}

#[cfg(feature = "with-bigdecimal")]
impl PgHasArrayType for PgRange<bigdecimal::BigDecimal> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("numrange")

    }
}

#[cfg(feature = "with-rust_decimal")]
impl PgHasArrayType for PgRange<rust_decimal::Decimal> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("numrange")

    }
}

#[cfg(feature = "with-chrono")]
impl PgHasArrayType for PgRange<chrono::NaiveDate> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("daterange")

    }
}

#[cfg(feature = "with-chrono")]
impl PgHasArrayType for PgRange<chrono::NaiveDateTime> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("tsrange")

    }
}

#[cfg(feature = "with-chrono")]
impl<Tz: chrono::TimeZone> PgHasArrayType for PgRange<chrono::DateTime<Tz>> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("tstzrange")

    }
}

#[cfg(feature = "time")]
impl PgHasArrayType for PgRange<time::Date> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("daterange")

    }
}

#[cfg(feature = "time")]
impl PgHasArrayType for PgRange<time::PrimitiveDateTime> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("tsrange")

    }
}

#[cfg(feature = "time")]
impl PgHasArrayType for PgRange<time::OffsetDateTime> {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("tstzrange")

    }
}

impl<'q, T> Encode<'q, Postgres> for PgRange<T>
where
    T: Encode<'q, Postgres>,
{
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {

        let sqlx_range = sqlx::postgres::types::PgRange {
            start: self.start.as_ref(),
            end: self.end.as_ref(),
        };

        sqlx_range.encode_by_ref(buf)
    }
}

impl<T> Decode<'_, Postgres> for PgRange<T>
where
    T: Type<Postgres> + for<'a> Decode<'a, Postgres>,
{
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let sqlx_range = <sqlx::postgres::types::PgRange<T> as Decode<Postgres>>::decode(value)?;
        Ok(PgRange::from(sqlx_range))
    }
}

fn range_compatible<E: Type<Postgres>>(ty: &PgTypeInfo) -> bool {
    // we require the declared type to be a _range_ with an
    // element type that is acceptable
    if let PgTypeKind::Range(element) = &ty.kind() {
        return E::compatible(element);
    }

    false
}
