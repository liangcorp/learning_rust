/*
    1. You can create any number of immutable references
    2. You can create only one mutable reference in a scope
    3. You can NOT create mutable reference if you program
        uses more than one immutable references (vise versa).
*/

fn main() {
    let s1 = String::from("hello");
    let r1 = &s1;    // Works (rule 1)
    let r2 = &s1;    // Works (rule 1)
    let r3 = &s1;    // Works (rule 1)

    let mut s2 = String::from("hello") ;
    let r4 = &mut s2;   // Works (rule 2)
    let r5 = &mut s2;   // ERROR! second mutable borrow (rule 2)

    {
        let r6 = &mut s2;   // Works. One per scope (rule 2)
    }


    let mut s3 = String::from("hello");
    let r7 = &mut s3;
    let r8 = &s3;       // ERROR! rule 3
}