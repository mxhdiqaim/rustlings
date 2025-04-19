#[derive(Debug)]

enum IpAddrKind {
    V4(u16, u16, u16, u16),
    V6(String),
}

// enum IpAddrKind2 {
//     V4(u16, u16, u16, u16),
//     V6(String),
// }

#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
}

fn main() {
    println!("{:#?}", Message::Resize);
    println!("{:#?}", Message::Move);
    println!("{:#?}", Message::Echo);
    println!("{:#?}", Message::ChangeColor);
    println!("{:#?}", Message::Quit);

    // let mut number: i8 = 127;

    // println!("1st number {}", number);

    // number += 1;

    // println!("2nd number {}", number);

    // let some_number = Some(10);
    // let some_char = Some('e');

    // let null_value: Option<String> = None;



    // let home_ip = IpAddrKind::V4(192, 168, 0, 1);
    // let loop_back = IpAddrKind::V6(String::from("::2"));

    // println!("Home IP {:?} and loop IP {:?}", home_ip, loop_back);
}
