/* #[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    message: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.message.push(msg);
    }
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.message.pop()
    }
} */

fn main() {
    /*  let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { message: vec![] },
    };
    println!("t0: {:?}", sat_a);
    base.send(&mut sat_a, Message::from("hello three!"));
    println!("t1: {:?}", sat_a);
    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);
    println!("msg: {:?}", msg); */

    /*  let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    println!("{}", r2); */
    let s = String::from("hello world");
    let h = &s[1];
    let s1 = "hello world";

    let word = first_word(&s);

    //s1.clear(); // error!

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}
