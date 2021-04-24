use super::Glob;

#[test]
fn globs() {
    let res = Glob::glob("/dev/input/*vent{0,1,2}").unwrap();

    dbg!("{:?}", &res.paths);
    
    assert!(res.paths.len() > 0);
}

