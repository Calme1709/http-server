use super::StringScanner;

#[derive(PartialEq, Eq)]
enum DecodeState {
    Initial,
    InEscape,
    Finished
}

pub struct URLEncoding {}

impl URLEncoding {
    pub fn decode(original_string: String) -> String {
        let mut scanner = StringScanner::new(&original_string.replace("+", " "));
        let mut output = String::new();
        let mut state = DecodeState::Initial;

        while state != DecodeState::Finished {
            match state {
                DecodeState::Initial => {
                    output.push_str(&scanner.consume_until_char('%'));

                    state = match scanner.finished() {
                        true => DecodeState::Finished,
                        false => DecodeState::InEscape
                    };
                },
                DecodeState::InEscape => {
                    let escape_sequence = scanner.consume_exact(3);

                    let decoded_sequence = match escape_sequence.len() == 3 && escape_sequence.chars().nth(1).unwrap().is_ascii_hexdigit() && escape_sequence.chars().nth(2).unwrap().is_ascii_hexdigit() {
                        true => match u32::from_str_radix(&escape_sequence[1..], 16) {
                            Ok(char_code) => match char::from_u32(char_code) {
                                // TODO: Support non-ascii characters in percent encodings
                                Some(char) => match char.is_ascii() {
                                    true => char.to_string(),
                                    false => escape_sequence
                                },
                                None => escape_sequence
                            },
                            Err(_err) => escape_sequence
                        },
                        false => escape_sequence
                    };

                    output.push_str(&decoded_sequence);

                    state = DecodeState::Initial;
                },
                _ => panic!("Invalid URLEncoding decode state")
            }
        }

        return output;
    }
}