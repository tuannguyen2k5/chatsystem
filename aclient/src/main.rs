fn main() {
    match aclient::run() {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => ratio,
    }
}
