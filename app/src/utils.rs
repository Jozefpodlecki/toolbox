pub fn widestr_to_string(ws: &[u16]) -> String {
    let len = ws.iter().position(|&c| c == 0).unwrap_or(ws.len());
    String::from_utf16_lossy(&ws[..len])
}