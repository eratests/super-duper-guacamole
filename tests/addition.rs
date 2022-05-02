use super_duper_guacamole::addition;

#[test]
fn validate_addition() {
    assert_eq!(addition(2, 2), 4);
    assert_eq!(addition(0, 2), 2);
    assert_eq!(addition(0, 0), 0);
    assert_eq!(addition(-2, 2), 0);
    assert_eq!(addition(-2, -2), -4);
    assert_eq!(addition(0, -2), -2)
}

//#[test]
#[allow(dead_code)]
fn should_fail() {
    panic!("Test failed with sucess.");
}
