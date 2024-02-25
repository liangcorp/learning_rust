macro_rules! make_struct {
    ($name:ident {$($field:ident: $ty:ty),* }) => {
        struct $name {
            $($field: $ty,)*
        }
    }
}

make_struct!(Sample { num1: f64, num2: i32 });

fn main() {
    let first_struct = Sample { num1: 0.5, num2: 3  };

    println!("{}, {}", first_struct.num1, first_struct.num2);
}
