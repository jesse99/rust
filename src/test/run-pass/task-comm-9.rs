// xfail-fast
#[legacy_modes];

extern mod std;

fn main() { test00(); }

fn test00_start(c: pipes::Chan<int>, number_of_messages: int) {
    let mut i: int = 0;
    while i < number_of_messages { c.send(i + 0); i += 1; }
}

fn test00() {
    let r: int = 0;
    let mut sum: int = 0;
    let p = pipes::PortSet();
    let number_of_messages: int = 10;
    let ch = p.chan();

    let mut result = None;
    do task::task().future_result(|+r| { result = Some(move r); }).spawn
          |move ch| {
        test00_start(ch, number_of_messages);
    }

    let mut i: int = 0;
    while i < number_of_messages {
        sum += p.recv();
        log(debug, r);
        i += 1;
    }

    option::unwrap(move result).recv();

    assert (sum == number_of_messages * (number_of_messages - 1) / 2);
}
