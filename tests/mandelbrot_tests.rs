#[test]
fn test_pixel_to_point() {
    let ps = PixelSpace {
        pixel_dims: (100, 100),
        complex_corners: (Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }),
    };
    assert_eq!(ps.pixel_to_point((25, 75)), Complex { re: -0.5, im: -0.5 })
}
