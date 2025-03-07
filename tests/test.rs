use findtext_pdf::search;

const RESOURCE_FILEPATH_01: &str = "./tests/fixtures/file1.pdf";
const DUMMY_FILEPATH_01: &str = "./tests/fixtures/_file0.pdf";

#[test]
fn found_01() {
    const EXPECT: usize = 2;

    let ret = search("hej", RESOURCE_FILEPATH_01);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);

    let ret = ret.unwrap();
    assert_eq!(ret.len(), EXPECT, "Expected Ok({}), got {:?}", EXPECT, ret);
}

#[test]
fn found_02() {
    const EXPECT: usize = 1;

    let ret = search("HEJ", RESOURCE_FILEPATH_01);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);

    let ret = ret.unwrap();
    assert_eq!(ret.len(), EXPECT, "Expected Ok({}), got {:?}", EXPECT, ret);
}

#[test]
fn missing_01() {
    const EXPECT: usize = 0;

    let ret = search("heJ", RESOURCE_FILEPATH_01);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);

    let ret = ret.unwrap();
    assert_eq!(ret.len(), EXPECT, "Expected Ok({}), got {:?}", EXPECT, ret);
}

#[test]
fn error_01() {
    let ret = search("hej", DUMMY_FILEPATH_01);

    assert!(ret.is_err(), "Expected Err, got {:?}", ret);
}
