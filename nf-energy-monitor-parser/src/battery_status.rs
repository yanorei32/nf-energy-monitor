use scraper::{Html, Selector};

#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Normal,
    Charge,
    Discharge,
}

impl State {
    fn try_from_alt(alt: &str) -> Option<Self> {
        match alt {
            "pts_battery_empty" => Some(Self::Normal),
            "pts_battery_discharge" => Some(Self::Discharge),
            "pts_battery_charge" => Some(Self::Charge),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Battery {
    pub state: State,
    pub remaining: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    Remaining,
    State,
}

fn get_remaining(html: &Html) -> Option<u32> {
    let font_selector = Selector::parse("font").unwrap();

    html.select(&font_selector)
        .next()
        .and_then(|e| e.inner_html().parse().ok())
}

fn get_state(html: &Html) -> Option<State> {
    let font_selector = Selector::parse("img.battery_bak").unwrap();

    html.select(&font_selector)
        .next()
        .and_then(|e| e.attr("alt").and_then(State::try_from_alt))
}

impl Battery {
    pub fn from_partial_html(html: &str) -> Result<Battery, ParseError> {
        let doc = Html::parse_fragment(html);

        let remaining = get_remaining(&doc).ok_or(ParseError::Remaining)?;
        let state = get_state(&doc).ok_or(ParseError::State)?;

        Ok(Battery { state, remaining })
    }
}

#[test]
fn test_battery() {
    let html = r#"<img src="pts_battery_empty.png" alt="pts_battery_empty" class="battery_bak">
                  <img src="pts_battery_g_010.png" alt="pts_battery_g_010" class="battery_p">
                  <p class="battery_p_str2"><font size="30">10</font>%</p>"#;

    assert_eq!(Battery::from_partial_html(html), Ok(Battery { state: State::Normal, remaining: 10 }));

    let html = r#"<img src="pts_battery_discharge.png" alt="pts_battery_discharge" class="battery_bak">
                  <img src="pts_battery_g_060.png" alt="pts_battery_g_060" class="battery_p">
                  <p class="battery_p_str2"><font size="30">56</font>%</p>"#;

    assert_eq!(Battery::from_partial_html(html), Ok(Battery { state: State::Discharge, remaining: 56 }));

    let html = r#"<img src="pts_battery_empty.png" alt="pts_battery_empty" class="battery_bak">
                  <img src="pts_battery_g_100.png" alt="pts_battery_g_100" class="battery_p">
                  <p class="battery_p_str3"><font size="30">100</font>%</p>"#;

    assert_eq!(Battery::from_partial_html(html), Ok(Battery { state: State::Normal, remaining: 100 }));
}
