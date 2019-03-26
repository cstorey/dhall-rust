#![feature(proc_macro_hygiene)]
use dhall::*;
use dhall_generator::*;

#[test]
fn test_dhall_type() {
    assert_eq!(bool::dhall_type(), dhall_expr!(Bool));
    assert_eq!(String::dhall_type(), dhall_expr!(Text));
    assert_eq!(
        <(bool, Option<String>)>::dhall_type(),
        dhall_expr!({ _1: Bool, _2: Optional Text })
    );

    #[derive(DhallType)]
    #[allow(dead_code)]
    struct A {
        field1: bool,
        field2: Option<bool>,
    }
    assert_eq!(
        A::dhall_type(),
        dhall_expr!({ field1: Bool, field2: Optional Bool })
    );

    #[derive(DhallType)]
    #[allow(dead_code)]
    struct B<'a, T: 'a> {
        field1: &'a T,
        field2: Option<T>,
    }
    assert_eq!(<B<'static, bool>>::dhall_type(), A::dhall_type());
}
