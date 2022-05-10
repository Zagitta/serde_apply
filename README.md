# Deserialize partial structs and update existing state

```rust
#[derive(Default, Debug, Deserialize, serde_apply_macros::SerdeApply)]
struct Foobar {
    a: String,
    b: Option<String>,
}

#[derive(Default, Debug, Deserialize, serde_apply_macros::SerdeApply)]
struct Bar {
    foo: Foobar,
    baz: i32,
}

fn main() {
    let mut my_bar = Bar::default();
    println!("Before update: {:#?}", my_bar);
    serde_apply::apply(
        &mut my_bar,
        &mut serde_json::Deserializer::from_str(r#"{}"#),
    )
    .unwrap();
    println!("After first (noop) update: {:#?}", my_bar);
    serde_apply::apply(
        &mut my_bar,
        &mut serde_json::Deserializer::from_str(r#"{"foo": {"b": "Hello World!"}}"#),
    )
    .unwrap();
    println!("After second update: {:#?}", my_bar);
}
```
```
Before update: Bar {
    foo: Foobar {
        a: "",
        b: None,
    },
    baz: 0,
}
After first (noop) update: Bar {
    foo: Foobar {
        a: "",
        b: None,
    },
    baz: 0,
}
After second update: Bar {
    foo: Foobar {
        a: "",
        b: Some(
            "Hello World!",
        ),
    },
    baz: 0,
}
```
