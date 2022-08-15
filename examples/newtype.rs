use otus_patterns_2::newtype::Password;

fn main() {
    let password: Password = "SomeAwesomePassword".into();

    println!("This is safe password display: {}", password)
}
