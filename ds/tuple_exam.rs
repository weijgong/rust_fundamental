fn reverse(pair:(i32, bool)) -> (bool, i32){
    let (interger,boolean) = pair;
    (boolean,interger)
}

#[derive(Debug)]
struct Matrix(f32, f32,f32,f32);

fn main(){
    let longtuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64, 'a', true);
    println!("long tuple first value:{}, long tuple second value:{}",
            longtuple.0, longtuple.1);
    let tuple_of = ((1u8,2u16,2u32),(4u64,-1i8),-2i16);
    println!("tuple of tuple {:?}", tuple_of);
    let pair = (1,true);
    println!("pair {:?}",pair);
    println!("the reversed pair is:{:?}",reverse(pair));
    let tuple = (1,"hello",4.5, true);
    let (a,b,c,d) = tuple;
    println!("{:?} {:?} {:?} {:?}", a, b, c, d);
    let matrix = (1.1,1.2,1.3,1.4);
    println!("{:?}",matrix);
}