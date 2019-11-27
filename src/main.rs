use std::boxed::Box;

fn main()
{
    let v = 1;
    let v2 = v;
    println!("v is: {}", v);

    let mut obj = Box::new(0);
    *obj += 15;

    println!("{}", *obj);

    do_something(&12);
}

fn do_something(v: & i32) {
    let c = v + 1;
    // something with v
}