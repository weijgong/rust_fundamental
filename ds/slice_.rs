use std::mem;

fn anlyze_slice(slice:&[i32]){
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements",slice.len());
    println!("{:?}",slice);
}


fn main(){
    let xs: [i32; 5] = [1,2,3,4,5];
    let ys: [i32; 500] = [0;500];
    println!("first element of the array:{}",xs[0]);
    println!("second element of the array:{}",xs[1]);
    println!("array size is {}",xs.len());
    println!("array occupies is {} bytes",mem::size_of_val(&xs));
    println!("borrow the whole array as a slice");
    anlyze_slice(&xs);
    println!("borrow the section of the array as a slice");
    anlyze_slice(&xs[1..4]);
    // println!("{}", xs[5]);
}