mod crypt;

fn main() {
    let password = String::from("hello");
    let salt = String::from("world");

    println!("{:?}", crypt::md5(&password, &salt));
}
