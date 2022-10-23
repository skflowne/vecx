use std::f64::consts::PI;

use crate::{Matrix, Vec3};

use std::f64::INFINITY;

use crate::{Vec2, VecX};

#[test]
fn from_f64() {
    assert_eq!(Vec3::from(2.0), Vec3(2.0, 2.0, 2.0))
}

#[test]
fn from_vec() {
    assert_eq!(Vec3::from(Vec2(1.0, 2.0)), Vec3(1.0, 2.0, 0.0));
    assert_eq!(Vec3::from(Vec3(1.0, 2.0, 3.0)), Vec3(1.0, 2.0, 3.0));
}

#[test]
fn from_vec64() {
    assert_eq!(Vec3::from(vec![1.0]), Vec3(1.0, 0.0, 0.0));
    assert_eq!(Vec3::from(vec![1.0, 2.0]), Vec3(1.0, 2.0, 0.0));
    assert_eq!(Vec3::from(vec![1.0, 2.0, 3.0]), Vec3(1.0, 2.0, 3.0));
    assert_eq!(Vec3::from(vec![1.0, 2.0, 3.0, 4.0]), Vec3(1.0, 2.0, 3.0));
}

#[test]
fn is_indexable() {
    let v = Vec3(0.0, 1.0, 2.0);
    assert_eq!(v[0], 0.0);
    assert_eq!(v[1], 1.0);
    assert_eq!(v[2], 2.0);
}

#[test]
fn add() {
    let v = Vec3(1.0, 2.0, 3.0);
    assert_eq!(v + Vec3(-1.0, -2.0, -3.0), Vec3(0.0, 0.0, 0.0));
    assert_eq!(v + Vec3(3.0, 4.0, 5.0), Vec3(4.0, 6.0, 8.0));
    assert_eq!(v + Vec3(-3.0, -4.0, -5.0), Vec3(-2.0, -2.0, -2.0));
}

#[test]
fn add_assign() {
    let mut v = Vec3(1.0, 2.0, 3.0);
    v += Vec3(-1.0, -2.0, -3.0);
    assert_eq!(v, Vec3(0.0, 0.0, 0.0));
}

#[test]
fn sub() {
    let v = Vec3(1.0, 2.0, 3.0);
    assert_eq!(v - Vec3(-1.0, -2.0, -3.0), Vec3(2.0, 4.0, 6.0));
    assert_eq!(v - Vec3(3.0, 4.0, 5.0), Vec3(-2.0, -2.0, -2.0));
    assert_eq!(v - Vec3(-3.0, -4.0, -5.0), Vec3(4.0, 6.0, 8.0));
    assert_eq!(Vec3(-2.0, -2.0, -2.0), -Vec3(2.0, 2.0, 2.0));
}

#[test]
fn sub_assign() {
    let mut v = Vec3(1.0, 2.0, 3.0);
    v -= Vec3(1.0, 2.0, 3.0);
    assert_eq!(v, Vec3(0.0, 0.0, 0.0));
}

#[test]
fn mul() {
    let v = Vec3(1.0, 2.0, 3.0);
    assert_eq!(v * Vec3(-1.0, -2.0, -3.0), Vec3(-1.0, -4.0, -9.0));
    assert_eq!(v * Vec3(3.0, 4.0, 5.0), Vec3(3.0, 8.0, 15.0));
    assert_eq!(v * Vec3(-3.0, -4.0, -5.0), Vec3(-3.0, -8.0, -15.0));
    assert_eq!(v * 2.0, Vec3(2.0, 4.0, 6.0));
    assert_eq!(2.0 * v, Vec3(2.0, 4.0, 6.0));
    assert_eq!(v * 0.0, Vec3(0.0, 0.0, 0.0));
    assert_eq!(0.0 * v, Vec3(0.0, 0.0, 0.0));
    assert_eq!(v * Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0));
}

#[test]
fn mul_assign() {
    let mut v = Vec3(1.0, 2.0, 3.0);
    v *= Vec3(-1.0, -2.0, -3.0);
    assert_eq!(v, Vec3(-1.0, -4.0, -9.0));

    v *= 6.0;
    assert_eq!(v, Vec3(-6.0, -24.0, -54.0));
}

#[test]
fn div() {
    let v = Vec3(1.0, 2.0, 3.0);
    assert_eq!(v / 1.0, v);
    assert_eq!(v / 2.0, Vec3(0.5, 1.0, 1.5));
    assert_eq!(v / Vec3(1.0, 1.0, 1.0), v);
    assert_eq!(v / Vec3(2.0, 4.0, 6.0), Vec3(0.5, 0.5, 0.5));
    assert_eq!(v / 0.0, Vec3(INFINITY, INFINITY, INFINITY));
    assert_eq!(v / Vec3(0.0, 0.0, 0.0), Vec3(INFINITY, INFINITY, INFINITY));
}

#[test]
fn div_assign() {
    let mut v = Vec3(1.0, 2.0, 3.0);
    v /= Vec3(-1.0, -2.0, -3.0);
    assert_eq!(v, Vec3(-1.0, -1.0, -1.0));

    v /= 2.0;
    assert_eq!(v, Vec3(-0.5, -0.5, -0.5));
}

#[test]
#[should_panic]
fn div_zero_by_zero_v3() {
    let _v = Vec3(0.0, 0.0, 0.0) / Vec3(1.0, 1.0, 0.0);
}

#[test]
#[should_panic]
fn div_zero_by_zero() {
    let _v = Vec3(0.0, 1.0, 2.0) / 0.0;
}

#[test]
#[should_panic]
fn div_assign_by_zero_v2() {
    let mut v = Vec3(0.0, 1.0, 2.0);
    v /= Vec3(0.0, 1.0, 2.0);
}

#[test]
#[should_panic]
fn div_assign_by_zero() {
    let mut v = Vec3(0.0, 1.0, 2.0);
    v /= 0.0;
}

#[test]
fn rem() {
    let v = Vec3(1.0, 2.0, 3.0);
    assert_eq!(v % 1.0, Vec3(0.0, 0.0, 0.0));
    assert_eq!(v % 2.0, Vec3(1.0, 0.0, 1.0));
    assert_eq!(v % Vec3(1.0, 1.0, 1.0), Vec3(0.0, 0.0, 0.0));
    assert_eq!(v % Vec3(2.0, 4.0, 6.0), Vec3(1.0, 2.0, 3.0));

    let v = Vec3(5.0, 10.0, 15.0);
    assert_eq!(v % 2.0, Vec3(1.0, 0.0, 1.0));
    assert_eq!(v % 4.0, Vec3(1.0, 2.0, 3.0));
    assert_eq!(v % Vec3(3.0, 7.0, 11.0), Vec3(2.0, 3.0, 4.0));
}

#[test]
fn rem_assign() {
    let mut v = Vec3(5.0, 10.0, 15.0);
    v %= Vec3(3.0, 7.0, 11.0);
    assert_eq!(v, Vec3(2.0, 3.0, 4.0));

    v %= 2.0;
    assert_eq!(v, Vec3(0.0, 1.0, 0.0));
}

#[test]
#[should_panic]
fn rem_by_zero_v3() {
    let _v = Vec3(2.0, 3.0, 4.0) % Vec3(1.0, 0.0, 1.0);
}

#[test]
#[should_panic]
fn rem_by_zero() {
    let _v = Vec3(2.0, 3.0, 4.0) % 0.0;
}

#[test]
#[should_panic]
fn rem_assign_by_zero_v3() {
    let mut v = Vec3(1.0, 1.0, 1.0);
    v %= Vec3(0.0, 1.0, 1.0);
}

#[test]
#[should_panic]
fn rem_assign_by_zero() {
    let mut v = Vec3(1.0, 1.0, 1.0);
    v %= 0.0;
}

#[test]
fn magnitude() {
    assert_eq!(Vec3(0.0, 0.0, 0.0).magnitude(), 0.0);
    assert_eq!(Vec3(1.0, 0.0, 0.0).magnitude(), 1.0);
    assert_eq!(Vec3(0.0, 1.0, 0.0).magnitude(), 1.0);
    assert_eq!(Vec3(1.0, 1.0, 1.0).magnitude(), f64::sqrt(3.0));
    assert_eq!(Vec3(4.0, 3.0, 2.0).magnitude(), f64::sqrt(16.0 + 9.0 + 4.0));
    assert_eq!(
        Vec3(-6.0, -2.0, -1.0).mag(),
        Vec3(6.0, 2.0, 1.0).magnitude()
    );
}

#[test]
fn normalized() {
    assert_eq!(Vec3(1.0, 0.0, 0.0).normalized(), Vec3(1.0, 0.0, 0.0));
    assert_eq!(Vec3(0.0, 1.0, 0.0).normalized(), Vec3(0.0, 1.0, 0.0));

    assert_eq!(Vec3(2.0, 0.0, 0.0).normalized(), Vec3(1.0, 0.0, 0.0));
    assert_eq!(Vec3(0.0, 2.0, 0.0).normalized(), Vec3(0.0, 1.0, 0.0));
}

#[test]
#[should_panic]
fn not_normalizable() {
    Vec3(0.0, 0.0, 0.0).normalized();
}

#[test]
fn dot_product() {
    let v_o = Vec3(0.0, 1.0, 0.0);
    let v_orth = Vec3(0.0, 0.0, -2.0);

    assert_eq!(v_o.dot_product(&v_orth), 0.0);
}

#[test]
fn dot() {
    let v_o = Vec3(0.0, 2.0, 0.0);
    let v_codir = Vec3(0.0, 3.0, 0.0);
    let v_orth = Vec3(0.0, 0.0, 3.0);
    let v_opp = Vec3(0.0, -3.0, 0.0);

    assert_eq!(v_o.dot_product(&v_orth), 0.0);
    assert_eq!(v_o.dot(&v_codir), 1.0);
    assert_eq!(v_o.dot(&v_opp), -1.0);
}

#[test]
fn angle() {
    let v_o = Vec3(0.0, 2.0, 0.0);
    let v_codir = Vec3(0.0, 3.0, 0.0);
    let v_orth = Vec3(0.0, 0.0, 3.0);
    let v_opp = Vec3(0.0, -3.0, 0.0);

    assert_eq!(v_o.angle(&v_orth), PI / 2.0);
    assert_eq!(v_o.angle(&v_codir), 0.0);
    assert_eq!(v_o.angle(&v_opp), PI);
    assert_eq!(v_o.angle(&v_orth).to_degrees(), 90.0);
    assert_eq!(v_o.angle(&v_opp).to_degrees(), 180.0);
    assert_eq!(v_o.angle(&v_codir).to_degrees(), 0.0);
}

#[test]
fn swizzle() {
    let v = Vec3(1.0, 2.0, 3.0);
    assert_eq!(v.s2("xy"), Vec2(1.0, 2.0));
    assert_eq!(v.s2("xx"), Vec2(1.0, 1.0));
    assert_eq!(v.s2("yy"), Vec2(2.0, 2.0));
    assert_eq!(v.s2("zz"), Vec2(3.0, 3.0));
    assert_eq!(v.s2("zx"), Vec2(3.0, 1.0));

    assert_eq!(v.s3("xxx"), Vec3(1.0, 1.0, 1.0));
    assert_eq!(v.s3("yyy"), Vec3(2.0, 2.0, 2.0));
    assert_eq!(v.s3("zzz"), Vec3(3.0, 3.0, 3.0));
    assert_eq!(v.s3("zyx"), Vec3(3.0, 2.0, 1.0));
}

#[test]
#[should_panic]
fn bad_swz_2_into_3() {
    Vec3(1.0, 2.0, 3.0).s3("xx");
}

#[test]
#[should_panic]
fn bad_swz_3_into_2() {
    Vec3(1.0, 2.0, 3.0).s2("xxx");
}

#[test]
#[should_panic]
fn bad_swz_1_into_2() {
    Vec3(1.0, 2.0, 3.0).s2("x");
}

#[test]
#[should_panic]
fn bad_swz_1_into_3() {
    Vec3(1.0, 2.0, 3.0).s3("x");
}

#[test]
#[should_panic]
fn bad_swz_invalid_component_s3() {
    Vec3(1.0, 2.0, 3.0).s3("xxc");
}

#[test]
#[should_panic]
fn bad_swz_invalid_component_s2() {
    Vec3(1.0, 2.0, 3.0).s2("xc");
}

#[test]
fn cross() {
    let a = Vec3(1.0, 0.0, 0.0);
    let b = Vec3(0.0, 1.0, 0.0);

    assert_eq!(a.cross(&b), Vec3(0.0, 0.0, 1.0));
    assert_eq!(b.cross(&a), Vec3(0.0, 0.0, -1.0));
}

#[test]
fn from_matrix() {
    let mat_3x1 = Matrix::from(vec![vec![1.1], vec![2.1], vec![3.1]]);
    assert_eq!(Vec3::from(mat_3x1), Vec3(1.1, 2.1, 3.1));

    let mat_1x3 = Matrix::from(vec![vec![1.1, 1.2, 1.3]]);
    assert_eq!(Vec3::from(mat_1x3), Vec3(1.1, 1.2, 1.3));
}
