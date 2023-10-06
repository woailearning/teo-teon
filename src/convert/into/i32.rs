use crate::error::Error;
use crate::value::Value;

impl TryInto<i32> for Value {

    type Error = Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Value::Int(b) => Ok(b),
            _ => Err(Error::new(format!("Cannot convert {} into i32", self.type_hint()))),
        }
    }
}

impl TryInto<i32> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Value::Int(b) => Ok(*b),
            _ => Err(Error::new(format!("Cannot convert {} into i32", self.type_hint()))),
        }
    }
}

impl TryInto<Option<i32>> for Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<i32>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Int(b) => Ok(Some(b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<i32>", self.type_hint()))),
        }
    }
}

impl TryInto<Option<i32>> for &Value {

    type Error = Error;

    fn try_into(self) -> Result<Option<i32>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Int(b) => Ok(Some(*b)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<i32>", self.type_hint()))),
        }
    }
}