use include_dir::Dir;
const Z80_TESTS: Dir<'_> = include_dir::include_dir!("z80/v1");

pub fn get_test(name: &str) -> Option<&'static str> {
    Z80_TESTS.get_file(name).map(|f| f.contents_utf8().unwrap())
}