use mockall::*;
use mockall::predicate::*;

use std::{path::Path, convert::AsRef};

#[automock]
pub trait FileIO {
    fn exists<P: AsRef<Path> + 'static>(&self, path: P) -> bool;
}

pub fn find_nearest(file_io: &impl FileIO) -> String {
    println!("file_io.exists('/') = {}", file_io.exists("/"));
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut file_io = MockFileIO::new();
        file_io.expect_exists::<&str>().return_const(true);

        let result = find_nearest(&file_io);

        assert_eq!(result, "");
    }
}

// References
// Testing I/O in Rust: https://users.rust-lang.org/t/whats-the-recommended-way-of-testing-i-o-functions/17282
// Traits in Rust: https://www.youtube.com/watch?v=T0Xfltu4h3A&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8
// Closures in Rust: https://www.youtube.com/watch?v=kZXJvLfjUS4
// AsRef trait: https://www.youtube.com/watch?v=iKFljZP6JD0
// Mocking (Alteratives at the bottom of the article): https://blog.logrocket.com/guide-mocking-rust-mockall/
