fn main(){
    let mut count = 0u32;
    println!("Let's count util infinity!");
    loop {
        count +=1;
        if count == 3{
            println!("three");
            continue;
        }
        println!("{}",count);
        if count == 5{
            println!("OK, it's enough!");
            break;
        }
    }
}