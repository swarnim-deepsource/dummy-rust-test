fn main() {
    if false {
        // this is untouched comment
        // this is untouched comment
        // this is untouched comment
    } print!("main");
}

async fn foo(val: Option<i32>) {
    let Some(x) = val else { return };
}

#[test]
fn test() {
    main();
}

// Lorem ipsum dolor sit amet, consectetur adipiscing elit.
