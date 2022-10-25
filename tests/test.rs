use hello_rust_macros::Display;

#[derive(Display)]
// No #[display(...)] item necessary
enum MyStruct {
    One,
    Two,
    Three,
}

// Transformation function
fn lowercase(s: &str) -> String {
    s.to_lowercase()
}

#[derive(Display)]
// Since the transform field is optional, the #[display(...)] item is
// completely ignored in the MyStruct declaration.
#[display(transform = "lowercase")]
enum MyLowercasedStruct {
    One,
    Two,
    Three,
}

#[test]
fn test() {
    // normal
    assert_eq!(MyStruct::One.to_string(), "One");
    assert_eq!(MyStruct::Two.to_string(), "Two");
    assert_eq!(MyStruct::Three.to_string(), "Three");

    // lowercased
    assert_eq!(MyLowercasedStruct::One.to_string(), "one");
    assert_eq!(MyLowercasedStruct::Two.to_string(), "two");
    assert_eq!(MyLowercasedStruct::Three.to_string(), "three");
}
