use addr;

#[macro_use]
extern crate lazy_static;


lazy_static! {
    static ref ADDRESS: String = String::from("123456");
}

#[test]
fn it_adds_two() {
    assert_eq!(4, addr::add_two(2));
}

#[test]
fn it_adds_three() {
    let test = "123456";
    assert_eq!(*ADDRESS, test);
    assert_eq!(&*ADDRESS, test);

    let s = String::from("123456");
    let ss: &str;
    ss= &s;
    assert_eq!(*ADDRESS, ss);
}
