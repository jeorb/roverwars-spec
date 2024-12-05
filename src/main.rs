use std::io;
use serde::{Deserialize, Serialize};

use map::Map;

mod map;

fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut state = State::default();

    state = parse_stream(state, stdin);
    println!("{}", serde_json::to_string_pretty(&state).unwrap());
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Action {
    Move,
    Left,
    Right,
    Grab,
    Drop,
    Wait,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    bot_id: String,
    action: Action,
    say: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    bot_id: String,
    actions: Vec<Action>,
    map: Map,
}

impl Default for State {
    fn default() -> Self {
        State {
            bot_id: "Um92ZXJXYXJzLmNvbQo=".to_owned(),
            actions: Vec::new(),
            map: Map::default(),
        }
    }
}

//fn parse(data: &str) -> Message {
//    return serde_json::from_str(data).unwrap();
//}

fn parse_stream(mut state: State, reader: impl io::Read) -> State {
    let deserializer = serde_json::Deserializer::from_reader(reader);
    let iterator = deserializer.into_iter::<Message>();

    for message in iterator {
        match message {
            Ok(message) => {
                println!("Got {:?}", message);
                println!("{}", serde_json::to_string(&message).unwrap());
                state.actions.push(message.action);
            }
            Err(e) => {
                println!("Error {:?}", e);
                break;
            }
        };
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn test_parse() {
        let mut state = State::default();

        let data = r#"
            {
                "bot_id": "Um92ZXJXYXJzLmNvbQo=",
                "action": "move",
                "say": "hello?"
            }
        "#;

        let message: Message = parse(data);
        assert_eq!(message.action, Action::Move);
    }*/

    #[test]
    fn test_parse_stream() {
        let mut state = State::default();

        let data = r#"
            {
                "bot_id": "Um92ZXJXYXJzLmNvbQo=",
                "action": "move",
                "say": "hello?"
            }
        "#
        .as_bytes();

        state = parse_stream(state, data);

        assert_eq!(state.actions.len(), 1);
        assert_eq!(state.actions[0], Action::Move);
    }
}
