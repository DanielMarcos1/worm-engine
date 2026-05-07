use worm_engine::geometry::vector::Vector3d;

#[test]
fn test_determinism_basic() {
    let v1 = Vector3d::new(1.0, 2.0, 3.0);
    let v2 = Vector3d::new(4.0, 5.0, 6.0);

    // Add
    let sum = v1.add(&v2);
    assert_eq!(sum.x, 5.0);
    assert_eq!(sum.y, 7.0);
    assert_eq!(sum.z, 9.0);

    // Magnitude
    let mag = Vector3d::new(3.0, 4.0, 0.0).magnitude();
    assert_eq!(mag, 5.0);
}
