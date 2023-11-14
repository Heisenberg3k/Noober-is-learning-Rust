fn main() {
    let a = 10;

    {
        let a = 5;
        println!("a_2: {}", a);
    }

    println!("a_1: {}", a);

    let a = a + 5;
    println!("a latest: {}", a);
}
