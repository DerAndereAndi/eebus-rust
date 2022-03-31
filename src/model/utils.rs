use std::fmt;

use serde::{Serialize};

pub fn provide_enum_display<T>(value: &T, f: &mut fmt::Formatter) -> fmt::Result
where
    T: ?Sized + Serialize,
{
	let result: String = serde_json::to_string(value).unwrap();
	write!(f, "{}", result)
}
