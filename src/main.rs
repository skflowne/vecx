use std::f64::consts::PI;
use std::f64::INFINITY;

use vecx::Vec2;
use vecx::Vec3;
use vecx::VecX;

fn main() {
    let v1 = Vec2(1.0, 1.0);
    let v2 = Vec2(2.0, 2.0);
    let v3 = Vec2(3.0, 3.0);
    let v4 = Vec2(4.0, 4.0);

    let v_asym = Vec2(5.0, 10.0);

    let f = 5.0;
    let z = 0.0;
    let v_z = Vec2(z, z);

    assert_eq!(v1[0], 1.0);
    assert_eq!(v1[1], 1.0);
    assert_eq!(v2[0], 2.0);
    assert_eq!(v3[0], 3.0);
    assert_eq!(v4[0], 4.0);
    assert_eq!(v1.x(), 1.0);
    assert_eq!(v_z.y(), 0.0);

    assert_eq!(Vec3::from(Vec2(2.0, 3.0)), Vec3(2.0, 3.0, 0.0));
    assert_eq!(Vec3::from((Vec2(2.0, 3.0), 4.0)), Vec3(2.0, 3.0, 4.0));
    assert_eq!(Vec3::from((4.0, Vec2(2.0, 3.0))), Vec3(4.0, 2.0, 3.0));
    assert_eq!(Vec3::from(7.5), Vec3(7.5, 7.5, 7.5));
    assert_eq!(Vec3::from(f), Vec3(5.0, 5.0, 5.0));

    let mut add_assign_v2_v2 = Vec2(2.0, 4.0);
    add_assign_v2_v2 += Vec2(3.0, -2.0);
    assert_eq!(add_assign_v2_v2, Vec2(5.0, 2.0));

    let mut sub_assign_v2_v2 = Vec2(2.0, 4.0);
    sub_assign_v2_v2 -= Vec2(1.0, 2.0);
    assert_eq!(sub_assign_v2_v2, Vec2(1.0, 2.0));

    let mut mul_assign_v2_v2 = Vec2(2.0, 4.0);
    mul_assign_v2_v2 *= Vec2(4.0, 2.0);
    assert_eq!(mul_assign_v2_v2, Vec2(8.0, 8.0));

    let mut mul_assign_v2_f = Vec2(2.0, 4.0);
    mul_assign_v2_f *= 4.0;
    assert_eq!(mul_assign_v2_f, Vec2(8.0, 16.0));

    let mut div_assign_v2_v2 = Vec2(2.0, 4.0);
    div_assign_v2_v2 /= Vec2(2.0, 0.0);

    println!("/= -> {:?}", div_assign_v2_v2);

    assert_eq!(Vec2(2.0, 4.0) / 2.0, Vec2(1.0, 2.0));
    assert_eq!(Vec2(2.0, 4.0) / Vec2(2.0, 4.0), Vec2(1.0, 1.0));
    assert_eq!(Vec2(2.0, 4.0) / Vec2(0.0, 0.0), Vec2(INFINITY, INFINITY));

    let mut mod_assign_v2_f = Vec2(5.0, 8.0);
    mod_assign_v2_f %= 5.0;
    assert_eq!(mod_assign_v2_f, Vec2(0.0, 3.0));

    assert_eq!(Vec2(3.0, 4.0).magnitude(), 5.0);
    assert_eq!(Vec2(1.0, 0.0).magnitude(), 1.0);
    assert_eq!(Vec2(0.0, 1.0).magnitude(), 1.0);
    assert_eq!(Vec2(-3.0, 4.0).magnitude(), 5.0);
    assert_eq!(Vec2(-3.0, -4.0).magnitude(), 5.0);

    let v_mul = Vec2(0.0, 0.0) * 2.0 * Vec3(1.0, 1.0, 1.0).s2("xy") * 6.0;
    assert_eq!(v_mul, Vec2(0.0, 0.0));

    let v_mul_2 = 3.0 * Vec2(2.0, 2.0) * Vec3(4.0, 4.0, 4.0).s2("xy");
    assert_eq!(v_mul_2, Vec2(24.0, 24.0));

    let v2_swz = Vec2(0.0, 1.0);
    let v3_swz = Vec3(3.0, 5.0, 7.0);

    /*assert_eq!(v2_swz.s::<Vec2>("xx"), Vec2(0.0, 0.0));
    assert_eq!(v2_swz.s::<Vec2>("yy"), Vec2(1.0, 1.0));
    assert_eq!(v2_swz.s::<Vec2>("yx"), Vec2(1.0, 0.0));
    assert_eq!(v2_swz.s::<Vec3>("xxx"), Vec3(0.0, 0.0, 0.0));
    assert_eq!(v2_swz.s::<Vec3>("yxy"), Vec3(1.0, 0.0, 1.0));

    assert_eq!(v3_swz.r("xxx").v3(), Vec3(3.0, 3.0, 3.0));
    assert_eq!(v3_swz.r("rgb").v3(), Vec3(3.0, 5.0, 7.0));
    assert_eq!(v3_swz.r("evo").v3(), Vec3(0.0, 0.0, 0.0));
    assert_eq!(v3_swz.r("xxx").v2(), Vec2(3.0, 3.0));
    assert_eq!(v3_swz.r("xy").v3(), Vec3(3.0, 5.0, 0.0));

    let swizz = v3_swz.r("xxxx").v2();
    dbg!(swizz);

    let swz = swizz.get("xx");
    v3_swz.r("xxxx").v3();*/

    let swz = v3_swz.s3("xxx");
    assert_eq!(v3_swz.s3("xxx"), Vec3(3.0, 3.0, 3.0));
    assert_eq!(v3_swz.s3("yyy"), Vec3(5.0, 5.0, 5.0));
    assert_eq!(v3_swz.s2("xx"), Vec2(3.0, 3.0));
    assert_eq!(v3_swz.s2("yy"), Vec2(5.0, 5.0));
    assert_eq!(v3_swz.s3("rgb"), v3_swz);

    let v_o = Vec2(2.0, 2.0);
    let v_codir = Vec2(3.0, 3.0);
    let v_orth = Vec2(3.0, -3.0);
    let v_opp = Vec2(-3.0, -3.0);

    assert_eq!(v_o.dot_product(&v_orth), 0.0);
    assert_eq!(v_o.dot(&v_codir), 1.0);
    assert_eq!(v_o.dot(&v_opp), -1.0);
    assert_eq!(v_o.angle(&v_orth), PI / 2.0);
    assert_eq!(v_o.angle(&v_codir), 0.0);
    assert_eq!(v_o.angle(&v_opp), PI);
    assert_eq!(v_o.angle(&v_orth).to_degrees(), 90.0);
    assert_eq!(v_o.angle(&v_opp).to_degrees(), 180.0);

    assert_eq!(Vec2(2.0, 2.0).dot_product(&Vec2(3.0, -3.0)), 0.0);
    assert_eq!(Vec2(2.0, 2.0).dot_product(&Vec2(2.0, 2.0)), 8.0);
    assert_eq!(Vec2(2.0, 2.0).dot(&Vec2(-3.0, -3.0)), -1.0);
    assert_eq!(Vec2(2.0, 2.0).dot(&Vec2(3.0, 3.0)), 1.0);

    //assert_eq!(Vec2(0.0, 0.0).angle(&Vec2(0.0, 0.0)), INFINITY);

    /*let add_v1_v2 = v1 + v2;
    assert_eq!(add_v1_v2, Vec2(3.0, 3.0));
    println!("{:?} + {:?} = {:?}", v1, v2, add_v1_v2);

    let add_v1_f = v1 + f;
    assert_eq!(add_v1_f, Vec2(6.0, 6.0));
    println!("{:?} + {:?} = {:?}", v1, f, add_v1_f);

    let mul_v1_v2 = v1 * v2;
    assert_eq!(mul_v1_v2, Vec2(2.0, 2.0));
    println!("{:?} * {:?} = {:?}", v1, v2, mul_v1_v2);

    let mul_v2_f = v2 * f;
    assert_eq!(mul_v2_f, Vec2(10.0, 10.0));
    println!("{:?} * {:?} = {:?}", v2, f, mul_v2_f);

    let div_v4_v2 = match v4 / v2 {
        None => Vec2(0.0, 0.0),
        Some(v) => v,
    };
    assert_eq!(div_v4_v2, Vec2(2.0, 2.0));
    println!("{:?} / {:?} = {:?}", v4, v2, div_v4_v2);

    let div_v1_vz = v1 / v_z;
    assert_eq!(div_v1_vz, None);
    println!("{:?} / {:?} = {:?}", v1, v_z, div_v1_vz);

    let div_v3_f = match v3 / f {
        None => Vec2(0.0, 0.0),
        Some(v) => v,
    };
    assert_eq!(div_v3_f, Vec2(0.6, 0.6));
    println!("{:?} / {:?} = {:?}", v3, f, div_v3_f);

    let div_v3_z = v3 / z;
    assert_eq!(div_v3_z, None);
    println!("{:?} / {:?} = {:?}", v3, z, div_v3_z);

    let mod_v3_v2 = match v3 % v2 {
        None => Vec2(0.0, 0.0),
        Some(v) => v,
    };
    assert_eq!(mod_v3_v2, Vec2(1.0, 1.0));
    println!("{:?} % {:?} = {:?}", v3, v2, mod_v3_v2);

    let mod_v1_vz = v1 % v_z;
    assert_eq!(mod_v1_vz, None);*/
}

#[test]
#[should_panic]
fn test_invalid_swizzle() {
    let v = Vec3(3.0, 5.0, 10.0);
    v.s3("xx");
}

#[test]
#[should_panic]
fn test_angle_of_zero_vecs() {
    let v1 = Vec2(0.0, 0.0);
    let v2 = Vec2(0.0, 0.0);
    dbg!(v1.angle(&v2));
}

#[test]
#[should_panic]
fn test_div_assign_v2_zero_by_zero() {
    let mut v = Vec2(0.0, 0.0);
    v /= Vec2(0.0, 0.0);
}

#[test]
#[should_panic]
fn test_div_v2_zero_by_zero() {
    let _v = Vec2(0.0, 1.0) / Vec2(0.0, 1.0);
}

#[test]
#[should_panic]
fn test_div_assign_v3_zero_by_zero() {
    let mut v = Vec3(0.0, 1.0, 1.0);
    v /= Vec3(0.0, 0.0, 0.0);
}

#[test]
#[should_panic]
fn test_div_v3_zero_by_zero() {
    let _v = Vec3(0.0, 0.0, 0.0) / Vec3(0.0, 0.0, 0.0);
}

#[test]
#[should_panic]
fn test_mod_zero() {
    let _v = Vec2(2.0, 2.0) % Vec2(0.0, 0.0);
}

#[test]
#[should_panic]
fn test_mod_assign_zero() {
    let mut mod_assign_v2_z = Vec2(4.0, 5.0);
    mod_assign_v2_z %= 0.0;
}
