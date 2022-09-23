use spice;


#[test]
fn test_spice_basic() {

    spice::furnsh("tests/test-data/spice/spk_pre_220614_221109_220810_jm0450.bsp");
    spice::furnsh("tests/test-data/spice/naif0012.tls");

    let et = spice::str2et("2022-AUG-17 15:13:57");
    let (position, light_time) = spice::spkpos("JUNO", et, "J2000", "NONE", "JUPITER");

    assert_eq!(position, [-15447.779100029416, 98600.66143504962, -412.3663683490047]);
    assert_eq!(light_time, 0.3329112444576964);

    spice::unload("tests/test-data/spk_pre_220614_221109_220810_jm0450.bsp");
    spice::unload("tests/test-data/spice/naif0012.tls");
}