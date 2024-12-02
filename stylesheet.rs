use stylist::style;

pub fn styles() {
    let style = style!(
        // A CSS string literal
        r#"
            background-color: red;
     
            .nested {
                background-color: blue;
                width: 100px
            }
        "#
     ).expect("Failed to mount style");     
}