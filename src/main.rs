fn foo(v : i32) {
    println!("foo : {}", v);
}

fn main() {
    let vev = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let some = &vev[0..];
    some.iter()
    .inspect(|test| { println!("{}", test); })
    .for_each(|x| { foo(*x) }); // performing copy because pointers/refs are likely just as big as an i32
    some.iter()
    .filter(|c| if **c > 8 { true } else { false })
    .next();
    println!("Hello, world!");
}
