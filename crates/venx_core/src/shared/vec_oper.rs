use glam::{uvec3, UVec3};

pub fn vec3_checked_add(left: &UVec3, right: &UVec3) -> Option<UVec3> {
    let (lx, ly, lz) = (left.x, left.y, left.z);
    let (rx, ry, rz) = (right.x, right.y, right.z);
    match (lx.checked_add(rx), ly.checked_add(ry), lz.checked_add(rz)) {
        (Some(x), Some(y), Some(z)) => Some((x, y, z).into()),
        _ => None,
    }
}

pub fn vec3_checked_sub(left: &UVec3, right: &UVec3) -> Option<UVec3> {
    let (lx, ly, lz) = (left.x, left.y, left.z);
    let (rx, ry, rz) = (right.x, right.y, right.z);
    match (lx.checked_sub(rx), ly.checked_sub(ry), lz.checked_sub(rz)) {
        (Some(x), Some(y), Some(z)) => Some((x, y, z).into()),
        _ => None,
    }
}

#[test]
fn test_checked_operations() {
    let left = uvec3(0, 0, 0);
    let right = uvec3(1, 1, 1);

    assert_eq!(vec3_checked_sub(&left, &right), None);

    let left = uvec3(1, 1, 0);
    let right = uvec3(1, 1, 0);
    assert_eq!(vec3_checked_sub(&left, &right).unwrap(), (0, 0, 0).into());
}
