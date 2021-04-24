use spectral::prelude::*;
use super::find_file;

#[test]
fn take_first_file() {
    let res = find_file("/dev/input/*event*").unwrap();
    dbg!(&res);

    assert_that(&res).starts_with("/dev/input");
}
