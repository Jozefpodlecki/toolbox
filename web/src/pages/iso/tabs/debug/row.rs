use iso_viewer::FormattedSize;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum DebugValue {
    String(String),
    Number(u64),
    Size(u64),
    Date(String),
    Boolean(bool),
    None,
}

pub fn format_iso_date(date: &str) -> String {
    if date.len() < 17 {
        return date.to_string();
    }
    
    if date.chars().all(|c| c == '0') {
        return "Not Set".to_string();
    }
    
    let year = &date[0..4];
    let month = &date[4..6];
    let day = &date[6..8];
    let hour = &date[8..10];
    let minute = &date[10..12];
    let second = &date[12..14];
    let centisecond = &date[14..16];
    
    if year == "0000" && month == "00" && day == "00" {
        return "Not Set".to_string();
    }
    
    format!(
        "{}-{}-{} {}:{}:{}.{}",
        year, month, day, hour, minute, second, centisecond
    )
}

impl DebugValue {
    pub fn as_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::Size(s) => FormattedSize::from(*s).to_string(),
            Self::Date(d) => format_iso_date(d),
            Self::Boolean(b) => if *b { "Yes".to_string() } else { "No".to_string() },
            Self::None => "—".to_string(),
        }
    }
}

impl From<String> for DebugValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for DebugValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<u64> for DebugValue {
    fn from(n: u64) -> Self {
        Self::Number(n)
    }
}

impl From<usize> for DebugValue {
    fn from(n: usize) -> Self {
        Self::Number(n as u64)
    }
}

impl From<bool> for DebugValue {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<Option<String>> for DebugValue {
    fn from(opt: Option<String>) -> Self {
        match opt {
            Some(s) if s.is_empty() => Self::None,
            Some(s) => Self::String(s),
            None => Self::None,
        }
    }
}

impl From<Option<&str>> for DebugValue {
    fn from(opt: Option<&str>) -> Self {
        match opt {
            Some(s) if s.is_empty() => Self::None,
            Some(s) => Self::String(s.to_string()),
            None => Self::None,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DebugRowProps {
    pub label: String,
    pub value: DebugValue,
}

#[function_component(DebugRow)]
pub fn debug_row(props: &DebugRowProps) -> Html {
    let formatted = props.value.as_string();
    
    html! {
        <div class="flex items-start gap-4 py-0.5">
            <span class="text-gray-500 w-40 flex-shrink-0">{&props.label}</span>
            <span class="text-white break-all">{formatted}</span>
        </div>
    }
}