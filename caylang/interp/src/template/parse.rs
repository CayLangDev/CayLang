use crate::template::ast::{TemplateLiteral, TemplatePart};
use std::collections::VecDeque;


#[derive(PartialEq)]
enum State {
    LayerPart,
    Text,
    TextCancel,
    /// Iterations ending on EndFullPart conclude the building of the current full part.
    /// Iterations ending on EndFullPart start on NewState
    EndFullPart,
    /// Iterations starting on NewState switch to an appropriate state for their character
    /// Iterations ending on NewState conclude the building of the current subpart.
    NewState
}

/// Parsing function
pub fn parse(path: String) -> TemplateLiteral {
    if path.len() == 0 {
        panic!("Empty template path!");
    }
    let relative = path.starts_with("/");
    let mut input = path.chars().peekable();
    let mut output = TemplateLiteral { relative, parts: vec![]};
    let mut last_state = State::NewState;
    let mut state = State::NewState;

    let mut full_buffer = vec![];
    let mut buffer = "".to_string();
    loop {
        let mut switch = None;
        let Some(c) = input.next() else {
            output.parts.push(full_buffer);
            break;
        };
        let c2 = input.peek();


        match state {
            State::NewState => {
                match c {
                    '{' => {
                        switch = Some(State::LayerPart);
                    }
                    '/' => {
                        // state = last_state;
                        switch = Some(State::EndFullPart);
                        // continue;
                    }
                    '\\' => {
                        switch = Some(State::TextCancel);
                    }
                    _ => {
                        switch = Some(State::Text);
                        buffer.push(c);
                        match c2 {
                            Some('{') => {
                                state = State::Text;
                                switch = Some(State::NewState);
                            }
                            _ => { }
                        }
                    }
                }
            }
            State::Text => {
                match c {
                    // '{' => {
                    //     switch = Some(State::LayerPart);
                    // }
                    '\\' => {
                        switch = Some(State::TextCancel);
                    }
                    '/' => {
                        switch = Some(State::EndFullPart);
                    }
                    _ => {
                        buffer.push(c);
                        match c2 {
                            Some('{') => {
                                switch = Some(State::NewState);
                            }
                            _ => { }
                        }
                    }
                }
            }
            State::TextCancel => {
                match c {
                    '{' => {
                        buffer.push(c);
                        switch = Some(State::Text);
                    }
                    '\\' => {
                        buffer.push('\\');
                        switch = Some(State::Text);
                    }
                    _ => {
                        panic!("Unidentified escape sequence in path template.");
                    }
                }
            }
            State::LayerPart => {
                match c {
                    '}' => {
                        switch = Some(State::NewState);
                    }
                    '/' => {
                        switch = Some(State::EndFullPart);
                    }
                    '\\' => {
                        panic!("Escape sequence not allowed in identifier");
                    }
                    _ => {
                        buffer.push(c); // should verify against ident format
                    }
                }
            }
            State::EndFullPart => unreachable!()
        }

        if let Some(next_state) = switch {
            // if !(state == State::NewState && next_state == State::EndFullPart) {
            last_state = state;
            // }
            state = next_state;
        }

        match state {
            State::NewState => {
                match last_state {
                    State::Text => {
                        full_buffer.push(TemplatePart::Text(buffer));
                    }
                    State::LayerPart => {
                        full_buffer.push(TemplatePart::LayerPart(buffer));
                    }
                    _ => {panic!("Error in parsing, subpart ended in wrong state");}
                }
                buffer = "".to_string();
            }
            State::EndFullPart => {
                match last_state {
                    State::NewState => {}
                    State::Text => {
                        full_buffer.push(TemplatePart::Text(buffer));
                    },
                    // layer part ought to conclude before /
                    _ => {panic!("Error in parsing, full part ended in wrong state");}
                }

                buffer = "".to_string();
                output.parts.push(full_buffer);
                full_buffer = vec![];
                state = State::NewState;
            }
            _ => {}
        }
    }

    return output;
}
