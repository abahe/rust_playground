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
fn ownership_rules() {
    // a tidak bisa dikses disini, belum dideklarasikan
    let a = 10; // a bisa diakses mulai disini

    {
        let b = 20;
        println!("{}", b)
    }
}

#[test]
fn ownership_movement() {
    let num1 = 20; // in stack!
    println!("num1 : {}", num1);

    let num2 = num1;

    println!("num2 : {}", num2);
    println!("num1 : {}", num1); //bisa karena di copy

    let name1 = String::from("Jajang");

    println!("name1 : {}", name1);

    let name2 = name1;

    println!("name2 : {}", name2);
    // println!("name1 : {}", name1); //will be error
    // kalau mau di clone !

    let name3 = name2.clone();
    println!("name3 : {}", name3);
    println!("name2 : {}, masih bisa", name2);
}

#[test]
fn test2() {
    println!("ini test 2");
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Kanu");

    println!("{}, {}", a, b);
}

#[test]
fn if_expres() {
    let value = 9;
    let result = if value >= 8 { "Good" } else { "Not good" };

    println!("cek : {}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter: {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("Result: {}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value: {}", array[index]);
        index += 1;
    }

    for value in array {
        println!("for iterate : {} ", value);
    }

    //range exclusive
    let range = 0..5; // jadi 0-4 , 5 exclusi tidak dihitung
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in range {
        println!("for iterate in range : {} ", i);
    }

    let range_inclusive = 0..=4;
    for i in range_inclusive {
        println!("for iterate in range inclusive : {} ", i);
    }
}

fn function_b() {
    let a = 10;
    let b = String::from("Nwanko");

    println!("{}, {}", a, b);
}

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
