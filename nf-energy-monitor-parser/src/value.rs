use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Wattage(i32),
    TimeInMinutes(u32),
    Mode(String),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntError(ParseIntError),
    InvalidTimeFormat,
    UnknownDataType,
}

impl Value {
    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        match s.trim() {
            "有" => Ok(Self::Boolean(true)),
            "無" => Ok(Self::Boolean(false)),
            s if s.ends_with("[W]") => Ok(Self::Wattage(
                s.strip_suffix("[W]")
                    .unwrap()
                    .parse()
                    .map_err(ParseError::ParseIntError)?,
            )),
            s if s.ends_with("モード") => {
                Ok(Self::Mode(s.strip_suffix("モード").unwrap().to_string()))
            }
            s if s.ends_with("分") => {
                let mut s = s.split("時");

                let h: u32 = s
                    .next()
                    .unwrap()
                    .parse()
                    .map_err(ParseError::ParseIntError)?;

                let m: u32 = s
                    .next()
                    .ok_or(ParseError::InvalidTimeFormat)?
                    .strip_suffix("分")
                    .unwrap()
                    .parse()
                    .map_err(ParseError::ParseIntError)?;

                Ok(Self::TimeInMinutes(h * 60 + m))
            }
            _ => Err(ParseError::UnknownDataType),
        }
    }
}

#[test]
fn test_value() {
    assert_eq!(Value::from_str("0[W]"), Ok(Value::Wattage(0)));
    assert_eq!(Value::from_str("100[W]"), Ok(Value::Wattage(100)));
    assert_eq!(Value::from_str("-100[W]"), Ok(Value::Wattage(-100)));
    assert_eq!(Value::from_str("0時0分"), Ok(Value::TimeInMinutes(0)));
    assert_eq!(
        Value::from_str("10時10分"),
        Ok(Value::TimeInMinutes(10 * 60 + 10))
    );
    assert_eq!(
        Value::from_str("22時10分"),
        Ok(Value::TimeInMinutes(22 * 60 + 10))
    );
    assert_eq!(
        Value::from_str("23時59分"),
        Ok(Value::TimeInMinutes(24 * 60 - 1))
    );
    assert_eq!(
        Value::from_str("ピークシフトモード"),
        Ok(Value::Mode("ピークシフト".to_string()))
    );
    assert_eq!(
        Value::from_str("有"),
        Ok(Value::Boolean(true))
    );
    assert_eq!(
        Value::from_str("無"),
        Ok(Value::Boolean(false))
    );
}
