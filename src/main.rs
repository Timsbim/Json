#![allow(dead_code, unused)]

use json::{self, Json, JsonError, JsonGet};

fn main() {
    let string = r#"
        {
            "a": 1,
            "b": [1, 2, 3, {}],
            "c":
                {
                    "def:,]": 1,
                    "e": [[{"z": {}}], null]
                }
        }
    "#;

    let json = json::parse(string);
    dbg!(&json);
    if let Ok(json) = json {
        dbg!(json.get(JsonGet::Key("c")));
        dbg!(json.get(JsonGet::Index(2)));
    }

    let json = Json::new_object()
        .insert(
            "foo",
            Json::new_array()
                .push(Json::Null)
                .push(Json::Null)
        )
        .insert("bar", Json::String("bar".to_string()));
    dbg!(&json);
}
