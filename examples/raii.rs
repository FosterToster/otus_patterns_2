use otus_patterns_2::raii_guard::{License, LicenseGuard};

fn ensure_legal(result: Result<LicenseGuard, String>) -> Option<LicenseGuard> {
    match result {
        Ok(guard) => Some(guard),
        Err(error) => {
            println!("Illegal usage!!! {}", error);
            None
        }
    }
}

fn main() {
    let license = License::with_max_count(2);

    let guard1 = ensure_legal(license.protect()).unwrap();
    println!("Legal usage");

    let guard2 = ensure_legal(license.protect()).unwrap();
    println!("Legal usage (last one)");

    ensure_legal(license.protect());
    println!("^^^^^^^ Did you seen illegal usage?!! ^^^^^^^");

    drop(guard2);
    let guard3 = ensure_legal(license.protect()).unwrap();
    println!("Legal usage again!!!");

    drop(guard3);
    drop(guard1);
}
