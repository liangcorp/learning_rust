fn fun1() {
    fn add() {
        println!("{}", 3 + 4);
    }
    add();
}

fn main() {
    fn add() {
        println!("{}", 2 + 3);
    }

    add();
    fun1();
}