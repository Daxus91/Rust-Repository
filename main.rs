fn main() {

    let mut v = Vec::new();

    for i in 1..11{
        v.push(i);
    }
    
    for x in 0..v.len() {
        if v[x] % 2 != 0 {
            println!("{:?}", v[x]);  
        }
    }  
}

