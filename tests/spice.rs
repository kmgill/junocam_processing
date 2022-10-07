use spice;

use junocam::jcspice::{self, JUNO_JUNOCAM};

#[test]
#[ignore = "Lack of cspice on GitHub"]
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

#[test]
#[ignore = "Lack of cspice on GitHub"]
fn test_furnish_base() {
    jcspice::furnish_base();
    jcspice::furnish("kernels/spk/spk_rec_220728_220909_220913.bsp");
    jcspice::furnish("kernels/ck/juno_sc_rec_220814_220820_v01.bc");


    let et = spice::str2et("2022-AUG-17 15:13:57");
    let (spacecraft_state, lt) = spice::spkpos("JUNO_SPACECRAFT", et, "IAU_JUPITER", "NONE", "JUPITER");
    let m_jup_j2000 = spice::pxform("IAU_JUPITER", "J2000", et);
    let m_juno_jupiter = spice::pxform("JUNO_SPACECRAFT", "IAU_JUPITER", et);
    let m_junocam_jupiter = spice::pxform("JUNO_JUNOCAM", "IAU_JUPITER", et);


    // let (shape, frame, bsight, n, bounds) = spice::getfov(JUNO_JUNOCAM, 4, 32, 32);
    // println!("Shape: {:?}");
}