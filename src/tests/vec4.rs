use std::f64::consts::PI;

use std::f64::INFINITY;

use crate::{Vec2, Vec3, Vec4, VecX};

#[test]
fn from_f64() {
    assert_eq!(Vec4::from(2.0), Vec4(2.0, 2.0, 2.0, 2.0))
}

#[test]
fn from_vec() {
    assert_eq!(Vec4::from(Vec2(1.0, 2.0)), Vec4(1.0, 2.0, 0.0, 0.0));
    assert_eq!(Vec4::from(Vec3(1.0, 2.0, 3.0)), Vec4(1.0, 2.0, 3.0, 0.0));
}

#[test]
fn from_vec64() {
    assert_eq!(Vec4::from(vec![1.0]), Vec4(1.0, 0.0, 0.0, 0.0));
    assert_eq!(Vec4::from(vec![1.0, 2.0]), Vec4(1.0, 2.0, 0.0, 0.0));
    assert_eq!(Vec4::from(vec![1.0, 2.0, 3.0]), Vec4(1.0, 2.0, 3.0, 0.0));
    assert_eq!(
        Vec4::from(vec![1.0, 2.0, 3.0, 4.0]),
        Vec4(1.0, 2.0, 3.0, 4.0)
    );
    assert_eq!(
        Vec4::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
        Vec4(1.0, 2.0, 3.0, 4.0)
    );
}

#[test]
fn is_indexable() {
    let v = Vec4(0.0, 1.0, 2.0, 3.0);
    assert_eq!(v[0], 0.0);
    assert_eq!(v[1], 1.0);
    assert_eq!(v[2], 2.0);
    assert_eq!(v[3], 3.0);
}

#[test]
fn add() {
    let v = Vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v + Vec4(-1.0, -2.0, -3.0, -4.0), Vec4(0.0, 0.0, 0.0, 0.0));
    assert_eq!(v + Vec4(3.0, 4.0, 5.0, 6.0), Vec4(4.0, 6.0, 8.0, 10.0));
    assert_eq!(
        v + Vec4(-3.0, -4.0, -5.0, -6.0),
        Vec4(-2.0, -2.0, -2.0, -2.0)
    );
}

#[test]
fn add_assign() {
    let mut v = Vec4(1.0, 2.0, 3.0, 4.0);
    v += Vec4(-1.0, -2.0, -3.0, -4.0);
    assert_eq!(v, Vec4(0.0, 0.0, 0.0, 0.0));
}

#[test]
fn sub() {
    let v = Vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v - Vec4(-1.0, -2.0, -3.0, -4.0), Vec4(2.0, 4.0, 6.0, 8.0));
    assert_eq!(v - Vec4(3.0, 4.0, 5.0, 6.0), Vec4(-2.0, -2.0, -2.0, -2.0));
    assert_eq!(v - Vec4(-3.0, -4.0, -5.0, -6.0), Vec4(4.0, 6.0, 8.0, 10.0));
    assert_eq!(Vec4(-2.0, -2.0, -2.0, -2.0), -Vec4(2.0, 2.0, 2.0, 2.0));
}

#[test]
fn sub_assign() {
    let mut v = Vec4(1.0, 2.0, 3.0, 4.0);
    v -= Vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v, Vec4(0.0, 0.0, 0.0, 0.0));
}

#[test]
fn mul() {
    let v = Vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(
        v * Vec4(-1.0, -2.0, -3.0, -4.0),
        Vec4(-1.0, -4.0, -9.0, -16.0)
    );
    assert_eq!(v * 2.0, Vec4(2.0, 4.0, 6.0, 8.0));
    assert_eq!(2.0 * v, Vec4(2.0, 4.0, 6.0, 8.0));
}

#[test]
fn mul_assign() {
    let mut v = Vec4(1.0, 2.0, 3.0, 4.0);
    v *= Vec4(-1.0, -2.0, -3.0, -4.0);
    assert_eq!(v, Vec4(-1.0, -4.0, -9.0, -16.0));

    v *= 6.0;
    assert_eq!(v, Vec4(-6.0, -24.0, -54.0, -96.0));
}

#[test]
fn div() {
    let v = Vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v / 1.0, v);
    assert_eq!(v / 2.0, Vec4(0.5, 1.0, 1.5, 2.0));
    assert_eq!(v / 0.0, Vec4(INFINITY, INFINITY, INFINITY, INFINITY));
    assert_eq!(
        v / Vec4(0.0, 0.0, 0.0, 0.0),
        Vec4(INFINITY, INFINITY, INFINITY, INFINITY)
    );
}

#[test]
fn div_assign() {
    let mut v = Vec4(1.0, 2.0, 3.0, 4.0);
    v /= Vec4(-1.0, -2.0, -3.0, -4.0);
    assert_eq!(v, Vec4(-1.0, -1.0, -1.0, -1.0));

    v /= 2.0;
    assert_eq!(v, Vec4(-0.5, -0.5, -0.5, -0.5));
}

#[test]
#[should_panic]
fn div_zero_by_zero_v4() {
    let _v = Vec4(0.0, 0.0, 0.0, 0.0) / Vec4(1.0, 1.0, 1.0, 0.0);
}

#[test]
#[should_panic]
fn div_zero_by_zero() {
    let _v = Vec4(1.0, 2.0, 3.0, 0.0) / 0.0;
}

#[test]
#[should_panic]
fn div_assign_by_zero_v4() {
    let mut v = Vec4(1.0, 2.0, 3.0, 0.0);
    v /= Vec4(1.0, 2.0, 3.0, 0.0);
}

#[test]
#[should_panic]
fn div_assign_by_zero() {
    let mut v = Vec4(1.0, 2.0, 3.0, 0.0);
    v /= 0.0;
}

#[test]
fn rem() {
    let v = Vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v % 1.0, Vec4(0.0, 0.0, 0.0, 0.0));
    assert_eq!(v % 2.0, Vec4(1.0, 0.0, 1.0, 0.0));
    assert_eq!(v % Vec4(1.0, 1.0, 1.0, 1.0), Vec4(0.0, 0.0, 0.0, 0.0));
    assert_eq!(v % Vec4(2.0, 4.0, 6.0, 8.0), Vec4(1.0, 2.0, 3.0, 4.0));
}

#[test]
fn rem_assign() {
    let mut v = Vec4(5.0, 10.0, 15.0, 20.0);
    v %= Vec4(3.0, 7.0, 11.0, 15.0);
    assert_eq!(v, Vec4(2.0, 3.0, 4.0, 5.0));

    v %= 2.0;
    assert_eq!(v, Vec4(0.0, 1.0, 0.0, 1.0));
}

#[test]
#[should_panic]
fn rem_by_zero_v4() {
    let _v = Vec4(2.0, 3.0, 4.0, 5.0) % Vec4(1.0, 1.0, 1.0, 0.0);
}

#[test]
#[should_panic]
fn rem_by_zero() {
    let _v = Vec4(2.0, 3.0, 4.0, 5.0) % 0.0;
}

#[test]
#[should_panic]
fn rem_assign_by_zero_v4() {
    let mut v = Vec4(1.0, 1.0, 1.0, 1.0);
    v %= Vec4(1.0, 1.0, 1.0, 0.0);
}

#[test]
#[should_panic]
fn rem_assign_by_zero() {
    let mut v = Vec4(1.0, 1.0, 1.0, 1.0);
    v %= 0.0;
}

#[test]
fn magnitude() {
    assert_eq!(Vec4(0.0, 0.0, 0.0, 0.0).magnitude(), 0.0);
    assert_eq!(Vec4(1.0, 0.0, 0.0, 0.0).magnitude(), 1.0);
    assert_eq!(Vec4(0.0, 1.0, 0.0, 0.0).magnitude(), 1.0);
    assert_eq!(Vec4(1.0, 1.0, 1.0, 1.0).magnitude(), f64::sqrt(4.0));
    assert_eq!(
        Vec4(4.0, 3.0, 2.0, 1.0).magnitude(),
        f64::sqrt(16.0 + 9.0 + 4.0 + 1.0)
    );
    assert_eq!(
        Vec4(-6.0, -2.0, -1.0, 2.0).mag(),
        Vec4(6.0, 2.0, 1.0, 2.0).magnitude()
    );
}

#[test]
fn normalized() {
    assert_eq!(
        Vec4(1.0, 0.0, 0.0, 0.0).normalized(),
        Vec4(1.0, 0.0, 0.0, 0.0)
    );
    assert_eq!(
        Vec4(0.0, 0.0, 0.0, 1.0).normalized(),
        Vec4(0.0, 0.0, 0.0, 1.0)
    );

    assert_eq!(
        Vec4(2.0, 0.0, 0.0, 0.0).normalized(),
        Vec4(1.0, 0.0, 0.0, 0.0)
    );
    assert_eq!(
        Vec4(0.0, 0.0, 0.0, 2.0).normalized(),
        Vec4(0.0, 0.0, 0.0, 1.0)
    );
}

#[test]
#[should_panic]
fn not_normalizable() {
    Vec4(0.0, 0.0, 0.0, 0.0).normalized();
}

#[test]
fn dot_product() {
    let v_o = Vec4(0.0, 1.0, 0.0, 1.0);
    let v_orth = Vec4(0.0, 0.0, -2.0, 1.0);

    assert_eq!(v_o.dot_product(&v_orth), 1.0);
}

#[test]
fn angle() {
    let v_o = Vec4(0.0, 2.0, 0.0, 0.0);
    let v_codir = Vec4(0.0, 4.0, 0.0, 0.0);
    let v_opp = Vec4(0.0, -3.0, 0.0, 0.0);

    assert_eq!(v_o.angle(&v_codir), 0.0);
    assert_eq!(v_o.angle(&v_opp), PI);

    assert_eq!(v_o.angle(&v_opp).to_degrees(), 180.0);
    assert_eq!(v_o.angle(&v_codir).to_degrees(), 0.0);
}

#[test]
fn swizzle() {
    let v = Vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v.s2("xy"), Vec2(1.0, 2.0));
    assert_eq!(v.s2("xx"), Vec2(1.0, 1.0));
    assert_eq!(v.s2("yy"), Vec2(2.0, 2.0));
    assert_eq!(v.s2("zz"), Vec2(3.0, 3.0));
    assert_eq!(v.s2("zx"), Vec2(3.0, 1.0));

    assert_eq!(v.s3("xxx"), Vec3(1.0, 1.0, 1.0));
    assert_eq!(v.s3("yyy"), Vec3(2.0, 2.0, 2.0));
    assert_eq!(v.s3("zzz"), Vec3(3.0, 3.0, 3.0));
    assert_eq!(v.s3("zyx"), Vec3(3.0, 2.0, 1.0));

    assert_eq!(v.s4("xxxx"), Vec4(1.0, 1.0, 1.0, 1.0));
    assert_eq!(v.s4("rrrr"), Vec4(1.0, 1.0, 1.0, 1.0));
}

#[test]
#[should_panic]
fn bad_swz_3_into_4() {
    Vec4(1.0, 2.0, 3.0, 4.0).s4("xxx");
}

#[test]
#[should_panic]
fn bad_swz_2_into_4() {
    Vec4(1.0, 2.0, 3.0, 4.0).s4("xx");
}

#[test]
#[should_panic]
fn bad_swz_1_into_4() {
    Vec4(1.0, 2.0, 3.0, 4.0).s4("x");
}

#[test]
#[should_panic]
fn bad_swz_invalid_component_s4() {
    Vec4(1.0, 2.0, 3.0, 4.0).s4("xxcx");
}
