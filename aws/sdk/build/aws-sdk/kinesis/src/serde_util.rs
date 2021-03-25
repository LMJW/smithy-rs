// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

pub fn stdoptionoptionsmithytypesblob_ser<S>(
    _inp: &std::option::Option<smithy_types::Blob>,
    _serializer: S,
) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
where
    S: serde::Serializer,
{
    use serde::Serialize;
    let el = _inp;
    el.as_ref()
        .map(|el| crate::blob_serde::BlobSer(el))
        .serialize(_serializer)
}

pub fn stdoptionoptionsmithytypesblob_deser<'de, D>(
    _deser: D,
) -> Result<std::option::Option<smithy_types::Blob>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    Ok(std::option::Option::<crate::blob_serde::BlobDeser>::deserialize(_deser)?.map(|el| el.0))
}

pub fn stdoptionoptionsmithytypesinstant_epoch_seconds_ser<S>(
    _inp: &std::option::Option<smithy_types::Instant>,
    _serializer: S,
) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
where
    S: serde::Serializer,
{
    use serde::Serialize;
    let el = _inp;
    el.as_ref()
        .map(|el| crate::instant_epoch::InstantEpoch(*el))
        .serialize(_serializer)
}

pub fn stdoptionoptionsmithytypesinstant_epoch_seconds_deser<'de, D>(
    _deser: D,
) -> Result<std::option::Option<smithy_types::Instant>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    Ok(
        std::option::Option::<crate::instant_epoch::InstantEpoch>::deserialize(_deser)?
            .map(|el| el.0),
    )
}
