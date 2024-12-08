use std::collections::HashMap;

use crate::{ParseError, Value};

#[derive(Debug, Clone)]
pub struct ValueMap(pub HashMap<String, Result<Value, ParseError>>);

impl ValueMap {
    pub fn from_partial_html(html: &str) -> Self {
        Self(HashMap::from_iter(html.split("<br>").filter_map(|l| {
            match l.split(':').collect::<Vec<_>>()[..] {
                [key, value] => Some((key.trim().to_string(), Value::from_str(value))),
                _ => None,
            }
        })))
    }
}

#[test]
fn test_value_map() {
    assert_eq!(
        ValueMap::from_partial_html(
            r#"運転モード:ピークシフトモード<br>
            PV余剰電力充電:有<br>
            太陽光発電電力:3323[W]<br>
            パワコン発電電力:0[W]<br>
            逆潮流電力:-1221[W]<br>
            家庭内使用電力:2102[W]<br>
            充電開始時刻:21時5分<br>
            充電終了時刻:8時55分<br>
            メイン放電開始時刻:15時0分<br>
            メイン放電終了時刻:21時0分<br>"#,
        )
        .0,
        HashMap::from([
            (
                "運転モード".to_string(),
                Ok(Value::Mode("ピークシフト".to_string()))
            ),
            ("PV余剰電力充電".to_string(), Ok(Value::Boolean(true))),
            ("太陽光発電電力".to_string(), Ok(Value::Wattage(3323))),
            ("パワコン発電電力".to_string(), Ok(Value::Wattage(0))),
            ("逆潮流電力".to_string(), Ok(Value::Wattage(-1221))),
            ("家庭内使用電力".to_string(), Ok(Value::Wattage(2102))),
            (
                "充電開始時刻".to_string(),
                Ok(Value::TimeInMinutes(21 * 60 + 5))
            ),
            (
                "充電終了時刻".to_string(),
                Ok(Value::TimeInMinutes(8 * 60 + 55))
            ),
            (
                "メイン放電開始時刻".to_string(),
                Ok(Value::TimeInMinutes(15 * 60 + 0))
            ),
            (
                "メイン放電終了時刻".to_string(),
                Ok(Value::TimeInMinutes(21 * 60 + 0))
            ),
        ])
    );
}
