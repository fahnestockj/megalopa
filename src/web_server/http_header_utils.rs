use std::collections::HashMap;

use httparse;

pub fn parse_headers<'a>(
    header_buf:  &mut [httparse::Header<'a>],
) -> Result<HashMap<&'a str, &'a str>, std::str::Utf8Error> {
    let mut headers = HashMap::new();
    for header in header_buf.into_iter().filter(|header| header.name != "") {
        // assume it's utf-8 and 400 bad req otherwise
        let value = std::str::from_utf8(header.value)?;
        headers.insert(header.name, value);
    }
    Ok(headers)
}
