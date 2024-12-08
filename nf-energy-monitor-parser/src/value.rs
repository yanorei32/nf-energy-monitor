use std::num::ParseIntError;

#[derive(PartialEq, Eq, Clone)]
pub enum Value {
    Wattage(i32),
    TimeInMinutes(u32),
    Mode(String),
    Boolean(bool),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Wattage(n) => write!(f, "{n}[W]"),
            Value::TimeInMinutes(n) => write!(f, "{:02}:{:02}", n / 60, n % 60),
            Value::Mode(s) => write!(f, "{s}[モード]"),
            Value::Boolean(b) => write!(f, "{b}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    ParseIntError(ParseIntError),
    InvalidTimeFormat,
    UnknownDataType,
}

impl std::str::FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
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
    assert_eq!("0[W]".parse(), Ok(Value::Wattage(0)));
    assert_eq!("100[W]".parse(), Ok(Value::Wattage(100)));
    assert_eq!("-100[W]".parse(), Ok(Value::Wattage(-100)));
    assert_eq!("0時0分".parse(), Ok(Value::TimeInMinutes(0)));
    assert_eq!(
        "10時10分".parse(),
        Ok(Value::TimeInMinutes(10 * 60 + 10))
    );
    assert_eq!(
        "22時10分".parse(),
        Ok(Value::TimeInMinutes(22 * 60 + 10))
    );
    assert_eq!(
        "23時59分".parse(),
        Ok(Value::TimeInMinutes(24 * 60 - 1))
    );
    assert_eq!(
        "ピークシフトモード".parse(),
        Ok(Value::Mode("ピークシフト".to_string()))
    );
    assert_eq!(
        "有".parse(),
        Ok(Value::Boolean(true))
    );
    assert_eq!(
        "無".parse(),
        Ok(Value::Boolean(false))
    );
}
