fn fibo(nterms: i32) {
    let mut n1: i32 = 0;
    let mut n2: i32 = 1;
    let mut count: i32 = 0;
    while count < nterms {
       println!("term {} is: {}", count, n1);
       let nth: i32 = n1 + n2;
       n1 = n2;
       n2 = nth;
       count += 1;
    }
}
