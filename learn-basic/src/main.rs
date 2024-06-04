fn main() {
    println!("Hello, world!");

    println!("yoyoy");
}

#[test]
fn hello_test() {
    println!("Hello this is test");
}

#[test]
fn test2(){
    println!("ini test 2");
}

#[test]
fn test_name(){
    let name = "Boa Kwon";
    println!("hellow {}", name);

    let mut nama = "jojon";

    nama = "Joni";
    println!("hellow {}", nama);
}


#[test]
fn shadowing(){
    let name = "Boa Kwon";
    println!("hellow {}", name);

    let name = 10;
    println!("hellow {}", name);
}