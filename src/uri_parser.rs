use std::str::Chars;

pub fn try_parse_scheme(parser: &mut Chars) -> Option<String> {
    match parser.next() {
        Some(first) if ((first >= 'a') && (first <= 'z')) || ((first >= 'A') && (first <= 'Z')) => {
            let mut buffer = String::new();

            // scheme must begin with alpha character
            buffer.push(first);
            loop {
                match parser.next() {
                    Some(c) if ((c >= 'a') && (c <= 'z')) || ((c >= 'A') && (c <= 'Z')) || ((c >= '0') && (c <= '9')) => {

                        // valid character, keep parsing
                        buffer.push(c);
                    },
                    Some(c) if (c == ':') => {
                        if (Some('/') == parser.next()) && (Some('/') == parser.next()) {

                            // found "://" sequence at current location, we're done with scheme parsing
                            return Some(buffer);
                        }
                        return None;
                    }
                    _ => return None
                }
            }
        },
        _ => return None
    };
}
