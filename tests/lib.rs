use num_traits::FromPrimitive;
use scancode::Scancode;

#[test]
fn test_from_primitive() {
    assert_eq!(Scancode::from_i32(0), None);
    assert_eq!(Scancode::from_i32(4), Some(Scancode::A));
}
