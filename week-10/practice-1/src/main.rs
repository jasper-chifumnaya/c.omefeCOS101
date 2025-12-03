fn main() {
    let v = vec![101,250, 330, 400];
    //vector v owns the object in heap

    //only a single variablee owns the heap memory at ny given time
    let v2 = v;
    //here two variables owns heap value,
    //two pointers to the same content is not allowed in rust

    //Rust is very smart in terms of memory access, so it detects a race condition//as twwo variables point to same heap

 
    println!("{:?}", v);
}
