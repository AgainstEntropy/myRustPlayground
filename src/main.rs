#![allow(unused)]
fn main() {


}
#[test]
fn test_const_and_static() {
    const DIGEST_SIZE: usize = 3;
    static ZERO: Option<u8> = Some(42);

    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    println!("digest: {digest:?}");

    let text: &str = "Hello";
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
        println!("digest: {digest:?}");
    }
}

use std::mem::transmute;

macro_rules! dbg_bits {
    ($e:expr, $bit_type:ty) => {
        println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
    };
}

#[test]
fn test_bitwise_repr() {
    unsafe {
        println!("bool:");
        dbg_bits!(false, u8);
        dbg_bits!(true, u8);

        println!("Option<bool>:");
        dbg_bits!(None::<bool>, u8);
        dbg_bits!(Some(false), u8);
        dbg_bits!(Some(true), u8);

        println!("Option<Option<bool>>:");
        dbg_bits!(Some(Some(false)), u8);
        dbg_bits!(Some(Some(true)), u8);
        dbg_bits!(Some(None::<bool>), u8);
        dbg_bits!(None::<Option<bool>>, u8);

        println!("Option<&i32>:");
        dbg_bits!(None::<&i32>, usize);
        dbg_bits!(Some(&0i32), usize);
    }
}

#[derive(Debug)]
#[repr(u32)]
enum Bar {
    A,  // 0
    B = 10000,
    C,  // 10001
}

#[test]
fn test_enum_discriminant() {
    let a = Bar::A;
    println!("a: {:?}", a);
    println!("a(u32): {:?}", a as u32);
    let b = Bar::B;
    println!("b: {:?}", b as u16);

    println!("C(u32): {}", Bar::C as u32);
    println!("C(u8): {}", Bar::C as u8);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

#[test]
fn test_enum() {
    let mut m = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);

    m = PlayerMove::Teleport { x: 1, y: 2 };
    println!("On this turn: {:?}", m);
}

#[test]
fn test_tuple_struct() {
    let mut p = Point(17, 23);
    p.0 = 2;
    println!("({}, {})", p.0, p.1);
}

struct Point(i32, i32);

#[test]
fn test_named_struct() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);

    let jackie = Person {
        name: String::from("Jackie"),
        ..avery
    };
    describe(&jackie);
}

struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

#[test]
fn test_exclusive_reference() {
    let mut point = (1, 2);
    let x_coord_ref = &mut point.0;
    // let x_coord = point.0;
    // let mut x_coord_ref_2 = &point.0;
    // let x_coord_ref_3 = &mut point.0;
    let y_coord_ref = &point.1;
    *x_coord_ref = 20;
    println!("point: {point:?}");
}

#[test]
fn test_shared_reference() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);

    r = &b;
    let s = &b;
    println!("r: {}", *r);
    println!("s: {}", *r);
}

#[test]
fn test_transpose() {
    let matrix = [
        [1, 2, 3],
        [4, 5, 6], 
        [7, 8, 9]
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);

    assert_eq!(transposed, [
        [1, 4, 7],
        [2, 5, 8], 
        [3, 6, 9]
    ]);

}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // unimplemented!()
    let mut transposed = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = matrix[j][i];
        }
    }

    transposed
}

#[test]
fn test_ested_array() {
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
    ];

    println!("matrix: {matrix:?}");
    println!("matrix[0]: {:?}", matrix[0]);
    println!("matrix[0][1]: {}", matrix[0][1]);
}

#[test]
fn test_destructuring() {
    describe_point((1, 0));
    describe_point((2, 3));

    let triple = [0, -2, 3, 4, 5, 6];
    println!("Tell me about {triple:?}");
    match triple {
        // [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..]                  => println!("First is 1 and the rest were ignored"),
        [_, .., 4]               => println!("First is ignored, the last is 4"),
        [a@.., b] => println!("the last is {b}, a = {a:?}"),
        [.., b]             => println!("the last is {b}, whatever the previous elements are"),
        _                        => println!("All elements were ignored"),
    }
}

fn describe_point(point: (i32, i32)) {
    match point {
        (0, _) => println!("on Y axis"),
        (_, 0) => println!("on X axis"),
        (x, _) if x < 0 => println!("left of Y axis"),
        (_, y) if y < 0 => println!("below X axis"),
        _ => println!("first quadrant"),
    }
}

#[test]
fn test_pattern_matching() {
    let input = 'x';
    match input {
        'q'                       => println!("Quitting"),
        'a' | 's' | 'w' | 'd'     => println!("Moving around"),
        '0'..='9'                 => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _                         => println!("Something else"),
    }
}

#[test]
fn test_array_iteration() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0); 
        }   
    }   
}

#[test]
fn test_array_and_tuple() {
    // a is an array
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");

    // t is a tuple
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
}

#[test]
fn test_control_flow() {
    let x = 10;
    let size = if x < 20 {
        "small"
    } else {
        "large"
    };
    println!("number size: {}", size);
}

#[test]
fn test_loops() {
    // 1..5 is a range that yields 1, 2, 3, 4
    for x in 1..=5 {
        println!("x: {x}");
    }

    'outer: for x in 1..5 {
        println!("x: {x}");
        let mut i = 0;
        let result = 'inner: loop {
            println!("x: {x}, i: {i}");
            i += 1;
            if i >= x {
                break 'inner  i;
            }
            if i == 3 {
                break 'outer;
            }
        };

        println!("result: {result}");
    }
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11),  15);
    assert_eq!(collatz_length(3),   8);
}

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    loop {
        if n == 1 {
            break;
        }
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }

    length
}

#[test]
fn test_block_and_scope() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
}

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

#[test]
fn test_varibles() {
    println!("Hello 🌍!");

    let mut x: i32 = 10;
    println!("x: {x}");
    x = 20;
    println!("x: {x}");

    println!("result: {}", interproduct(120, 100, 248));

    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    // planet = "🌍";
    let mut sentence: String = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
    println!("{:?}", &sentence[11..15]);

    println!(r#"<a href="link.html">link</a>"#);
    println!(r##"<a href="link.html">link</a>"##);
    println!("<a href=\"link.html\">link</a>");
}

#[test]
fn test_type_inference() {
    // The compiler infers the type of the variable from the value assigned to it.
    // Such declaration is identical to the explicit declaration of a type. 
    let x = 10;
    takes_u32(x);

    let y = 20;
    takes_i8(y);

    let n = 6;
    println!("fib({n}) = {}", fib(n));
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

#[test]
fn test_fib() {
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(4), 3);
    assert_eq!(fib(6), 8);
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        // The base case.
        // unimplemented!("Implement this");
        return 1;
    } else {
        // The recursive case.
        // todo!("Implement this");
        return fib(n - 1) + fib(n - 2);
    }
}