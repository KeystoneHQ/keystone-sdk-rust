use minicbor::data::{Int, Type};
use minicbor::Decoder;

pub(crate) fn cbor_map<'b, F, T>(
    d: &mut Decoder<'b>,
    obj: &mut T,
    mut cb: F,
) -> Result<(), minicbor::decode::Error>
where
    F: FnMut(Int, &mut T, &mut Decoder<'b>) -> Result<(), minicbor::decode::Error>,
{
    let entries = d.map()?;
    let mut index = 0;
    loop {
        let key = d.int()?;
        (cb)(key, obj, d)?;
        index += 1;
        if let Some(len) = entries {
            if len == index {
                break;
            }
        }
        if let Type::Break = d.datatype()? {
            d.skip()?;
            break;
        }
    }
    Ok(())
}

pub(crate) fn cbor_array<'b, F, T>(
    d: &mut Decoder<'b>,
    obj: &mut T,
    mut cb: F,
) -> Result<(), minicbor::decode::Error>
where
    F: FnMut(u64, &mut T, &mut Decoder<'b>) -> Result<(), minicbor::decode::Error>,
{
    let entries = d.array()?;
    let mut index = 0;
    loop {
        (cb)(index, obj, d)?;
        index += 1;
        if let Some(len) = entries {
            if len == index {
                break;
            }
        }
        if let Type::Break = d.datatype()? {
            d.skip()?;
            break;
        }
    }
    Ok(())
}

pub(crate) fn cbor_type(data_type: Type) -> Type {
    match data_type {
        Type::U8
        | Type::U16
        | Type::U32
        | Type::U64
        | Type::I8
        | Type::I16
        | Type::I32
        | Type::I64
        | Type::Int => Type::Int,
        _ => data_type,
    }
}
