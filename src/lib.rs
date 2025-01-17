use include_directory::{include_directory, Dir};

const Z80_TESTS: Dir<'_> = include_directory!("z80/v1");

pub fn get_z80_tests(name: &str) -> Option<&'static str> {
    Z80_TESTS.get_file(name).map(|f| f.contents_utf8().unwrap())
}