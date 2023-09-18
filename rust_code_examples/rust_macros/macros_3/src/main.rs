macro_rules! sum_macro {
    ($($x:expr),*) => {
        {
            let mut sum = 0;
            $(sum += $x;)*
            sum
        }
    };
}

fn main() {
    let mut sum = 0;
    sum += 1;
    sum += 2;
    sum += 3;
    sum += 4;
    sum += 5;
    let result = sum;

    let result = sum_macro!(1, 2, 3, 4, 5);
}

