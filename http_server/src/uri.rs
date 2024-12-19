use std::collections::HashMap;

use crate::utils::StringScanner;
use crate::utils::URLEncoding;

pub struct URI {
    pub path: String,

    #[allow(dead_code)]
    pub query: HashMap<String, String>,

    #[allow(dead_code)]
    pub fragment: String,
}

#[derive(PartialEq, Eq)]
enum URIParsingState {
    Path,
    Query,
    Fragment,
    Finished
}

impl URI {
    pub fn from_string(string: String) -> Self {
        let mut scanner = StringScanner::new(&string);

        let mut state = URIParsingState::Path;

        let mut path = String::new();
        let mut query: HashMap<String, String> = HashMap::new();
        let mut fragment = String::new();

        while state != URIParsingState::Finished {
            match state {
                URIParsingState::Path => {
                    path = scanner.consume_until(|char| char == '?' || char == '#');

                    state = match scanner.finished() {
                        true => URIParsingState::Finished,
                        false => match scanner.consume_exact(1).chars().nth(0).unwrap() {
                            '?' => URIParsingState::Query,
                            '#' => URIParsingState::Fragment,

                            _ => panic!("Unexpected character when parsing URI")
                        }
                    }
                },
                URIParsingState::Query => {
                    let query_string = scanner.consume_until(|char| char == '#');

                    // NOTE: Query string format is not standardized - this implementation is in line with the
                    // algorithm recommended for decoding application/x-www-form-urlencoded payloads:
                    // https://www.w3.org/TR/2014/REC-html5-20141028/forms.html#url-encoded-form-data

                    // 1. Let strings be the result of strictly splitting the string payload on U+0026 AMPERSAND characters (&).
                    let strings = query_string.split("&").map(|string| String::from(string)).collect::<Vec<String>>();

                    // 2. If the isindex flag is set and the first string in strings does not contain a "=" (U+003D)
                    // character, insert a "=" (U+003D) character at the start of the first string in strings.

                    // 3. Let pairs be an empty list of name-value pairs.
                    let mut pairs: HashMap<String, String> = HashMap::new();

                    // 4. For each string string in strings, run these substeps:
                    for string in strings {
                        let mut string_scanner = StringScanner::new(&string);

                        // 1. If string contains a "=" (U+003D) character, then let name be the substring of string
                        //    from the start of string up to but excluding its first "=" (U+003D) character, and let
                        //    value be the substring from the first character, if any, after the first "=" (U+003D)
                        //    character up to the end of string. If the first "=" (U+003D) character is the first
                        //    character, then name will be the empty string. If it is the last character, then value
                        //    will be the empty string.
                        // 
                        //    Otherwise, string contains no "=" (U+003D) characters. Let name have the value of string
                        //    and let value be the empty string.
                        
                        let name = string_scanner.consume_until_char('=');
                        string_scanner.consume_exact(1);
                        let value = string_scanner.consume_rest();

                        // 2. Replace any "+" (U+002B) characters in name and value with U+0020 SPACE characters.
                        // 3. Replace any escape in name and value with the character represented by the escape. This
                        //    replacement must not be recursive.
                        // 4. Convert the name and value strings to their byte representation in ISO-8859-1 (i.e. convert the Unicode string to a byte string, mapping code points to byte values directly).
                        // 5. Add a pair consisting of name and value to pairs.
                        pairs.insert(URLEncoding::decode(name), URLEncoding::decode(value));
                    }

                    // 5. If any of the name-value pairs in pairs have a name component consisting of the string
                    //    "_charset_" encoded in US-ASCII, and the value component of the first such pair, when decoded
                    //    as US-ASCII, is the name of a supported character encoding, then let encoding be that
                    //    character encoding (replacing the default passed to the algorithm).
                    // 6. Convert the name and value components of each name-value pair in pairs to Unicode by
                    //    interpreting the bytes according to the encoding encoding.

                    // 7. Return pairs.
                    query = pairs;

                    state = match scanner.finished() {
                        true => URIParsingState::Finished,
                        false => URIParsingState::Fragment
                    }
                },
                URIParsingState::Fragment => {
                    fragment = scanner.consume_rest();

                    state = URIParsingState::Finished;
                },
                URIParsingState::Finished => panic!("Should not run state machine in finished state")
            }
        }

        return Self {
            path,
            query,
            fragment
        };
    }
}