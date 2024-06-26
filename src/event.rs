use casper_event_standard::casper_types;
use casper_event_standard::Schema;
use casper_types::{
    bytesrepr::{
        FromBytes, ToBytes, OPTION_NONE_TAG, OPTION_SOME_TAG, RESULT_ERR_TAG, RESULT_OK_TAG,
    },
    CLType, CLValue,
};

use crate::error::ToolkitError;

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub fields: Vec<(String, CLValue)>,
}

impl Event {
    pub fn to_ces_bytes(&self) -> Result<Vec<u8>, ToolkitError> {
        let mut result: Vec<u8> = vec![];

        let prefixed_name = String::from(EVENT_PREFIX) + &self.name;
        let event_name =
            String::to_bytes(&prefixed_name).map_err(|_e| ToolkitError::SerializationError {
                context: "event_name",
            })?;
        result.extend_from_slice(&event_name);

        for (_field_name, field_value) in &self.fields {
            let field_bytes = field_value.inner_bytes();
            result.extend_from_slice(field_bytes);
        }

        Ok(result)
    }
}

const EVENT_PREFIX: &str = "event_";

pub fn parse_dynamic_event_data(
    dynamic_event_schema: Schema,
    event_data: &[u8],
) -> Result<Vec<(String, CLValue)>, ToolkitError> {
    let mut event_fields = vec![];

    let mut remainder = event_data;
    let schema_fields = dynamic_event_schema.to_vec();
    for (field_name, field_type) in schema_fields {
        let cltype = field_type.downcast();
        let (field_value, new_remainder) = parse_dynamic_clvalue(&cltype, remainder)?;
        remainder = new_remainder;

        event_fields.push((field_name, field_value));
    }

    Ok(event_fields)
}

// Deserializes bytes into CLValue, based on runtime CLType.
//
// CAUTION: There is no recursion limit.
//
// NOTE: Implementation is a dirty combination of:
// - type resolution - similar to `depth_limited_from_bytes()`,
// - deserializing data - based on `from_bytes()`,
// - serializing data - based on `to_bytes()`.
//
fn parse_dynamic_clvalue<'a>(
    cltype: &CLType,
    bytes: &'a [u8],
) -> Result<(CLValue, &'a [u8]), ToolkitError> {
    let result = match cltype {
        casper_types::CLType::Bool => {
            let (value, new_remainder) =
                bool::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::Bool",
                })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::Bool",
                })?;
            (
                CLValue::from_components(casper_types::CLType::Bool, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::I32 => {
            let (value, new_remainder) =
                i32::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::I32",
                })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::I32",
                })?;
            (
                CLValue::from_components(casper_types::CLType::I32, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::I64 => {
            let (value, new_remainder) =
                i64::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::I64",
                })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::I64",
                })?;
            (
                CLValue::from_components(casper_types::CLType::I64, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::U8 => {
            let (value, new_remainder) =
                u8::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::U8",
                })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::U8",
                })?;
            (
                CLValue::from_components(casper_types::CLType::U8, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::U32 => {
            let (value, new_remainder) =
                u32::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::U32",
                })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::U32",
                })?;
            (
                CLValue::from_components(casper_types::CLType::U32, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::U64 => {
            let (value, new_remainder) =
                u64::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::U64",
                })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::U64",
                })?;
            (
                CLValue::from_components(casper_types::CLType::U64, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::U128 => {
            let (value, new_remainder) = casper_types::U128::from_bytes(bytes).map_err(|_e| {
                ToolkitError::DeserializationError {
                    context: "CLType::U128",
                }
            })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::U128",
                })?;
            (
                CLValue::from_components(casper_types::CLType::U128, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::U256 => {
            let (value, new_remainder) = casper_types::U256::from_bytes(bytes).map_err(|_e| {
                ToolkitError::DeserializationError {
                    context: "CLType::U256",
                }
            })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::U256",
                })?;
            (
                CLValue::from_components(casper_types::CLType::U256, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::U512 => {
            let (value, new_remainder) = casper_types::U512::from_bytes(bytes).map_err(|_e| {
                ToolkitError::DeserializationError {
                    context: "CLType::U512",
                }
            })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::U512",
                })?;
            (
                CLValue::from_components(casper_types::CLType::U512, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::Unit => {
            let (value, new_remainder) =
                <()>::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::Unit",
                })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::Unit",
                })?;
            (
                CLValue::from_components(casper_types::CLType::Unit, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::String => {
            let (value, new_remainder) =
                String::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::String",
                })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::String",
                })?;
            (
                CLValue::from_components(casper_types::CLType::String, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::Key => {
            let (value, new_remainder) = casper_types::Key::from_bytes(bytes).map_err(|_e| {
                ToolkitError::DeserializationError {
                    context: "CLType::Key",
                }
            })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::Key",
                })?;
            (
                CLValue::from_components(casper_types::CLType::Key, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::URef => {
            let (value, new_remainder) = casper_types::URef::from_bytes(bytes).map_err(|_e| {
                ToolkitError::DeserializationError {
                    context: "CLType::URef",
                }
            })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::URef",
                })?;
            (
                CLValue::from_components(casper_types::CLType::URef, value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::PublicKey => {
            let (value, new_remainder) =
                casper_types::PublicKey::from_bytes(bytes).map_err(|_e| {
                    ToolkitError::DeserializationError {
                        context: "CLType::PublicKey",
                    }
                })?;
            let value_bytes = value
                .to_bytes()
                .map_err(|_e| ToolkitError::SerializationError {
                    context: "CLType::PublicKey",
                })?;
            (
                CLValue::from_components(casper_types::CLType::PublicKey, value_bytes),
                new_remainder,
            )
        }
        // More complex types.
        casper_types::CLType::Option(t) => {
            let (tag, remainder) =
                u8::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::Option",
                })?;
            match tag {
                OPTION_NONE_TAG => {
                    let value_bytes = vec![tag];
                    let new_remainder = remainder;
                    (
                        CLValue::from_components(
                            casper_types::CLType::Option(t.clone()),
                            value_bytes,
                        ),
                        new_remainder,
                    )
                }
                OPTION_SOME_TAG => {
                    let (t_parsed, new_remainder) = parse_dynamic_clvalue(t, remainder)?;
                    let mut value_bytes = vec![tag];
                    value_bytes.extend(t_parsed.inner_bytes());
                    (
                        CLValue::from_components(
                            casper_types::CLType::Option(t.clone()),
                            value_bytes,
                        ),
                        new_remainder,
                    )
                }
                _ => Err(ToolkitError::DeserializationError {
                    context: "CLType::Option",
                })?,
            }
        }
        casper_types::CLType::List(t) => {
            let (count, mut remainder) =
                u32::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::List",
                })?;
            let mut value_bytes = vec![];
            value_bytes.extend(count.to_bytes().map_err(|_e| {
                ToolkitError::SerializationError {
                    context: "CLType::List",
                }
            })?);
            for _ in 0..count {
                let (t1_parsed, next_remainder) = parse_dynamic_clvalue(t, remainder)?;
                remainder = next_remainder;
                value_bytes.extend(t1_parsed.inner_bytes());
            }
            (
                CLValue::from_components(casper_types::CLType::List(t.clone()), value_bytes),
                remainder,
            )
        }
        casper_types::CLType::ByteArray(t) => {
            let fixed_length = *t as usize;
            let mut value_bytes = vec![];
            let (t_parsed, new_remainder) = bytes.split_at(fixed_length);
            value_bytes.extend(t_parsed);
            (
                CLValue::from_components(casper_types::CLType::ByteArray(*t), value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::Result {
            ok: t_ok,
            err: t_err,
        } => {
            let (variant, remainder) =
                u8::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::Result",
                })?;
            match variant {
                RESULT_ERR_TAG => {
                    let (t_err_parsed, new_remainder) = parse_dynamic_clvalue(t_err, remainder)?;
                    let mut value_bytes = vec![variant];
                    value_bytes.extend(t_err_parsed.inner_bytes());
                    (
                        CLValue::from_components(
                            casper_types::CLType::Result {
                                ok: t_ok.clone(),
                                err: t_err.clone(),
                            },
                            value_bytes,
                        ),
                        new_remainder,
                    )
                }
                RESULT_OK_TAG => {
                    let (t_ok_parsed, new_remainder) = parse_dynamic_clvalue(t_ok, remainder)?;
                    let mut value_bytes = vec![variant];
                    value_bytes.extend(t_ok_parsed.inner_bytes());
                    (
                        CLValue::from_components(
                            casper_types::CLType::Result {
                                ok: t_ok.clone(),
                                err: t_err.clone(),
                            },
                            value_bytes,
                        ),
                        new_remainder,
                    )
                }
                _ => Err(ToolkitError::DeserializationError {
                    context: "CLType::Result",
                })?,
            }
        }
        casper_types::CLType::Map {
            key: t_key,
            value: t_value,
        } => {
            let (num_keys, mut remainder) =
                u32::from_bytes(bytes).map_err(|_e| ToolkitError::DeserializationError {
                    context: "CLType::Map",
                })?;
            let mut value_bytes = vec![];
            value_bytes.extend(num_keys.to_bytes().map_err(|_e| {
                ToolkitError::SerializationError {
                    context: "CLType::Map",
                }
            })?);
            for _ in 0..num_keys {
                let (key_parsed, next_remainder) = parse_dynamic_clvalue(t_key, remainder)?;
                let (value_parsed, next_remainder) =
                    parse_dynamic_clvalue(t_value, next_remainder)?;
                remainder = next_remainder;
                value_bytes.extend(key_parsed.inner_bytes());
                value_bytes.extend(value_parsed.inner_bytes());
            }
            (
                CLValue::from_components(
                    casper_types::CLType::Map {
                        key: t_key.clone(),
                        value: t_value.clone(),
                    },
                    value_bytes,
                ),
                remainder,
            )
        }
        casper_types::CLType::Tuple1([t1]) => {
            let (t1_parsed, new_remainder) = parse_dynamic_clvalue(t1, bytes)?;
            let mut value_bytes = vec![];
            value_bytes.extend(t1_parsed.inner_bytes());
            (
                CLValue::from_components(casper_types::CLType::Tuple1([t1.clone()]), value_bytes),
                new_remainder,
            )
        }
        casper_types::CLType::Tuple2([t1, t2]) => {
            let (t1_parsed, remainder) = parse_dynamic_clvalue(t1, bytes)?;
            let (t2_parsed, new_remainder) = parse_dynamic_clvalue(t2, remainder)?;
            let mut value_bytes = vec![];
            value_bytes.extend(t1_parsed.inner_bytes());
            value_bytes.extend(t2_parsed.inner_bytes());
            (
                CLValue::from_components(
                    casper_types::CLType::Tuple2([t1.clone(), t2.clone()]),
                    value_bytes,
                ),
                new_remainder,
            )
        }
        casper_types::CLType::Tuple3([t1, t2, t3]) => {
            let (t1_parsed, remainder) = parse_dynamic_clvalue(t1, bytes)?;
            let (t2_parsed, remainder) = parse_dynamic_clvalue(t2, remainder)?;
            let (t3_parsed, new_remainder) = parse_dynamic_clvalue(t3, remainder)?;
            let mut value_bytes = vec![];
            value_bytes.extend(t1_parsed.inner_bytes());
            value_bytes.extend(t2_parsed.inner_bytes());
            value_bytes.extend(t3_parsed.inner_bytes());
            (
                CLValue::from_components(
                    casper_types::CLType::Tuple3([t1.clone(), t2.clone(), t3.clone()]),
                    value_bytes,
                ),
                new_remainder,
            )
        }
        casper_types::CLType::Any => {
            // Consume all the remaining bytes and put it in `Any` type.
            let (new_remainder, value_bytes) = bytes.split_at(0);
            (
                CLValue::from_components(casper_types::CLType::Any, value_bytes.to_vec()),
                new_remainder,
            )
        }
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use casper_types::{AsymmetricType, CLTyped};

    use super::*;

    fn roundtrip_assert<T: CLTyped + ToBytes>(value: T) {
        // Serialize with Casper format.
        let clvalue = CLValue::from_t(value).unwrap();

        // Extract serialization components.
        let cltype = clvalue.cl_type();
        let bytes = clvalue.inner_bytes();

        println!("type: {:?}", cltype);

        // Dynamically parse data back.
        let (parsed_clvalue, remainder) = parse_dynamic_clvalue(cltype, bytes).unwrap();

        // Asserts.
        assert_eq!(
            parsed_clvalue, clvalue,
            "Roundtrip should give the same CLValue."
        );
        assert!(remainder.is_empty(), "All bytes should have been consumed.");
    }

    #[test]
    fn test_string_roundtrip() {
        let string = String::from("hello");

        roundtrip_assert(string);
    }

    #[test]
    fn test_publickey_roundtrip() {
        let pub_key = casper_types::PublicKey::system();

        roundtrip_assert(pub_key);
    }

    #[test]
    fn test_option_roundtrip() {
        let num: u64 = 20;
        let option = Some(num);

        roundtrip_assert(option);
        roundtrip_assert(None::<u64>);
    }

    #[test]
    fn test_list_roundtrip() {
        let list: Vec<u64> = vec![1, 6, 3, 3];

        roundtrip_assert(list);
    }

    #[test]
    fn test_bytearray_roundtrip() {
        let array: [u8; 6] = [1, 0, 3, 2, 6, 6];

        roundtrip_assert(array);
    }

    #[test]
    fn test_result_roundtrip() {
        let res1: Result<u64, bool> = Ok(32);
        let res2: Result<u64, bool> = Err(false);

        roundtrip_assert(res1);
        roundtrip_assert(res2);
    }

    #[test]
    fn test_map_roundtrip() {
        let mut map: BTreeMap<u64, bool> = BTreeMap::new();
        map.insert(30, true);
        map.insert(40, false);
        map.insert(50, true);

        roundtrip_assert(map);
    }

    #[test]
    fn test_tuple2_roundtrip() {
        let num: u64 = 42;
        let pub_key = casper_types::PublicKey::system();
        let tuple = (num, pub_key);

        roundtrip_assert(tuple);
    }
}
