use postgres::test_redis;

fn main() {
    test_redis().expect("key couldnt be found");
}
