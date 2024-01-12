
fn main() {
    
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

#[test]
fn destruct_struct() {
    let foo = Foo { x: (2, 2), y: 2 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i } => println!("y = 2, x = {i:?}"),
        Foo { y, .. }             => println!("y = {y}, other fields were ignored"),
    }
}

#[derive(Debug)]
enum Result {
    Ok(u32),
    Err(String),
}

fn divide_in_two(n: u32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

#[test]
fn destruct_enum() {
    let n = 99;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}


fn sleep_for(secs: f32) {
    let dur_ = if let Ok(dur) = std::time::Duration::try_from_secs_f32(secs) {
        dur
    } else {
        std::time::Duration::from_millis(500)
    };
    std::thread::sleep(dur_);
    println!("slept for {:?}", dur_);
}


#[test]
fn if_let() {
    sleep_for(-10.0);
    sleep_for(0.8);
    sleep_for(0.5);
}

fn hex_or_die_trying(maybe_string: Option<String>) -> Result {
    let s = if let Some(s) = maybe_string {
        s
    } else {
        return Result::Err(String::from("got None"));
    };

    let first_byte_char = if let Some(first_byte_char) = s.chars().next() {
        first_byte_char
    } else {
        return Result::Err(String::from("got empty string"));
    };

    if let Some(digit) = first_byte_char.to_digit(16) {
        Result::Ok(digit)
    } else {
        Result::Err(String::from("not a hex digit"))
    }
}

fn hex_or_die_trying_flatten(maybe_string: Option<String>) -> Result {
    let Some(s) = maybe_string else {
        return Result::Err(String::from("got None"));
    };

    let Some(first_byte_char) = s.chars().next() else {
        return Result::Err(String::from("got empty string"));
    };

    let Some(digit) = first_byte_char.to_digit(16) else {
        return Result::Err(String::from("not a hex digit"));
    };

    return Result::Ok(digit);
}


#[test]
fn let_else() {
    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
    println!("result: {:?}", hex_or_die_trying(Some(String::from("Aoo"))));
    println!("result: {:?}", hex_or_die_trying(Some(String::from(""))));
    println!("result: {:?}", hex_or_die_trying(None));

    println!("result: {:?}", hex_or_die_trying_flatten(None));
}

#[test]
fn while_let() {
    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop() {
        println!("character: {c}");
    }
}