//! Assertion macro tests

#[macro_use]
extern crate nalgebra;

use nalgebra::{ApproxEq, Vector2};
use std::fmt::Debug;

// Replace the assert_approx_eq! macro so that we can have type inference.
fn test_approx_eq<T, N>(given: &T, expected: &T) where T: Debug + ApproxEq<N>, N: Debug {
    if !given.approx_eq(expected) {
        panic!("assertion failed: `left ≈ right` (left: `{:?}`, right: `{:?}`, tolerance: `{:?}`)",
            *given, *expected,
            T::approx_epsilon()
        )
    }
}

#[test]
fn assert_approx_eq_f64() {
    let a = 1.0f64;
    let b = 1.0f64 + 1.0e-12f64;
    test_approx_eq(&a, &b);
    test_approx_eq(&(&a), &(&b));
}

#[test]
#[should_panic]
fn assert_approx_eq_vec2_f32_fail() {
    let a = Vector2::new(1.0f32, 0.0);
    let b = Vector2::new(1.1f32, 0.1);
    test_approx_eq(&a, &b);
}

#[test]
fn assert_approx_eq_eps_f32() {
    assert_approx_eq_eps!(1.0f32, 1.1, 0.2);
    assert_approx_eq_eps!(&mut 1.0f32, &mut 1.1, 0.2);
}

#[test]
#[should_panic]
fn assert_approx_eq_eps_f64_fail() {
    assert_approx_eq_eps!(1.0f64, 1.1, 0.05);
}

#[test]
fn assert_approx_eq_ulps_f32() {
    let x = 1000000_f32;
    let y = 1000000.1_f32;
    assert!(x != y);
    assert_approx_eq_ulps!(x, y, 3);
    assert_approx_eq_ulps!(&x, &y, 3);
}

#[test]
#[should_panic]
fn assert_approx_eq_ulps_f32_fail() {
    let x = 1000000_f32;
    let y = 1000000.1_f32;
    assert_approx_eq_ulps!(x, y, 2);
}

#[test]
fn assert_approx_eq_ulps_f64() {
    let x = 1000000_f64;
    let y = 1000000.0000000003_f64;
    assert!(x != y);
    assert_approx_eq_ulps!(x, y, 4);
}

#[test]
#[should_panic]
fn assert_approx_eq_ulps_f64_fail() {
    let x = 1000000_f64;
    let y = 1000000.0000000003_f64;
    assert!(x != y);
    assert_approx_eq_ulps!(x, y, 3);
}
