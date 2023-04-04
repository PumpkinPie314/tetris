fn main() {
    let mut s:Vec<i32> = vec![];
    s = add_eight(s);
    s = add_eight(s);
    s = add_eight(s);
    s = add_eight(s);
    println!("{s:?}")
}

fn add_eight(mut s: Vec<i32>) -> Vec<i32>{
    s.push(8);
    s
}