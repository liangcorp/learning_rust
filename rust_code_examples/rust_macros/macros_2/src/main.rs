macro_rules! make_functions {
    ($($func_name:ident: $return_type:ty => $return_expr:expr),+) => {
        $(
            fn $func_name() -> $return_type {
                $return_expr
            }
        )+
    };
}

make_functions!(foo: i32 => 42, bar: String => "hello world".to_owned());

// fn foo() -> i32 {
//     42
// }
// fn bar() -> String {
//     "hello world".to_owned()
// }

fn main() {
    let result1 = foo();

    let result2 = bar();

    println!("foo result: {}", result1);

    println!("bar result: {}", result2);
}

