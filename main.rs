fn main() {

    let mut v = Vec::new();

    for i in 1..11{

        if i % 2 != 0 {
            v.push(i);
        }
    }

    println!("{:?}", v);
    
}
