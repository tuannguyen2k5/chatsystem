fn main() {
    match a_chat::run() {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => ratio,
    }
}
