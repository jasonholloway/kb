use super::Glob;

#[test]
fn globs() {
    let found = Glob::glob("/dev/input/*vent{0,1,2}").unwrap();

    println!("{:?}", &found.paths);
    
    assert!(found.paths.len() > 0)
}


