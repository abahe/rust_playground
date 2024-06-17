const MAXIMUM: i32 = 100;

fn main() {
    println!("Hello, world!");

    println!("yoyoy");
}

#[test]
fn hello_test() {
    println!("Hello this is test");

    let str = " nu gelo ";

    println!("{}", str);

    let str2 = str.trim();

    println!("{}", str2);
}

#[test]
fn test2() {
    println!("ini test 2");
}

// #[test]
// fn stack_heap() {
//     function_a();
//     function_b();
// }

// fn function_a() {
//     let a = 10;
//     let b = String::from("Kanu");

//     println!("{}, {}", a, b);
// }

// fn function_b() {
//     let a = 10;
//     let b = String::from("Nwanko");

//     println!("{}, {}", a, b);
// }

#[test]
fn test_name() {
    let name = "Boa Kwon";
    println!("hellow {}", name);

    let mut nama = "jojon";

    nama = "Joni";
    println!("hellow {}", nama);
}

#[test]
fn shadowing() {
    let name = "Boa Kwon";
    println!("hellow {}", name);

    let name = 10;
    println!("hellow {}", name);
}

/*
multi baris
ok good

Data type
Scalar type - single value -> integer, float, boolean, dan char

Compound type - tuple, array


 */
#[test]
fn explicit() {
    let age: i8 = 20;
    let phi = 3.14;
    let b: i16 = age as i16;
    println!("age : {} and phi : {}", b, phi);

    let mut a = 10;
    a += 10;
    println!("{}", a);

    let aa = 10 > 20;
    print!("hasil nya : {}", aa);

    let xC = 'a';
    let xCC = "a";
}

#[test]
fn tuplex() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    // println!("{}", data);
    println!("{:?}", data);

    //akses tuple
    println!("{}", data.0);
    println!("{}", data.2);

    //destructing tuple
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    //default tuple, immutable
    data.0 = 77;

    println!("{}", data.0);
}

#[test]
fn array_test() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let mut array = [2.0, 3.1, 4.5];
    println!("{:?}", array);

    println!("{}", array[1]);

    array[1] = 3.5;

    let length = array.len();

    println!("{} jum {} ", array[1], length);
}
