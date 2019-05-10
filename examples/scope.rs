use clint::HandlerArray;

static HANDLERS: HandlerArray = HandlerArray::new();

fn main() {
    let mut cl = || println!("whoa!");

    HANDLERS.with_overrides(|arr| {
        arr.register(0, &mut cl);

        dummy_int();
        dummy2_int();
    });
}

fn dummy_int() {
    HANDLERS.call(0)
}
fn dummy2_int() {
    HANDLERS.call(1)
}
