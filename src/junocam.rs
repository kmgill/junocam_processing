use sciimg::vector::Vector;

/*
      cx = INS-6150#_DISTORTION_X
      cy = INS-6150#_DISTORTION_Y
      k1 = INS-6150#_DISTORTION_K1
      k2 = INS-6150#_DISTORTION_K2
      fl = INS-6150#_FOCAL_LENGTH/INS-6150#_PIXEL_SIZE
*/

pub struct FrameletParameters {
    pub id: i32,
    cx: f64,
    cy: f64,
    k1: f64,
    k2: f64,
    focal_length: f64,
    pixel_size: f64,
}

impl FrameletParameters {
    pub fn fl(&self) -> f64 {
        self.focal_length / self.pixel_size
    }

    /*
        def undistort(c):
            xd, yd = c[0], c[1]
            for i in range(5): # fixed number of iterations for simplicity
            r2 = (xd**2+yd**2)
            dr = 1+k1*r2+k2*r2*r2
            xd = c[0]/dr
            yd = c[1]/dr
            return [xd, yd]
    */
    pub fn undistort(&self, c0: f64, c1: f64) -> (f64, f64) {
        let mut xd = c0;
        let mut yd = c1;
        for _ in 0..5 {
            let r2 = xd.powi(2) + yd.powi(2);
            let dr = 1.0 + self.k1 * r2 + self.k2 * r2 * r2;
            xd = c0 / dr;
            yd = c1 / dr;
        }
        (xd, yd)
    }

    /*
        def distort(c):
            xd, yd = c[0], c[1]
            r2 = (xd**2+yd**2)
            dr = 1+k1*r2+k2*r2*r2
            xd *= dr
            yd *= dr
            return [xd, yd]
    */
    pub fn distort(&self, c0: f64, c1: f64) -> (f64, f64) {
        let mut xd = c0;
        let mut yd = c1;
        let r2 = xd.powi(2) + yd.powi(2);
        let dr = 1.0 + self.k1 * r2 + self.k2 * r2 * r2;
        xd *= dr;
        yd *= dr;
        (xd, yd)
    }

    /*
    given a vector v in the JUNO_JUNOCAM reference frame, the following
    computes an XY coordinate in Junocam framelet coordinates with 0,0
    in the upper left:

        alpha = v[2]/fl
        cam = [v[0]/alpha, v[1]/alpha]
        cam = distort(cam)
        x = cam[0]+cx
        y = cam[1]+cy
    */
    pub fn vector_to_xy(&self, v: &Vector) -> (f64, f64) {
        let alpha = v.z / self.fl();
        let cam0 = v.x / alpha;
        let cam1 = v.y / alpha;
        let cam = self.distort(cam0, cam1);
        let x = cam.0 + self.cx;
        let y = cam.1 + self.cy;
        (x, y)
    }

    /*
    and the following takes an XY coordinate in Junocam framelet
    coordinates and produces a vector in the JUNO_JUNOCAM reference
    frame (of arbitrary length).

        cam[0] = x-cx
        cam[1] = y-cy
        cam = undistort(cam)
        v = [cam[0], cam[1], fl]
    */
    pub fn xy_to_vector(&self, x: f64, y: f64) -> Vector {
        let cam0 = x - self.cx;
        let cam1 = y - self.cy;
        let cam = self.undistort(cam0, cam1);
        Vector {
            x: cam.0,
            y: cam.1,
            z: self.fl(),
        }
    }
}

pub static JUNO_JUNOCAM_METHANE: FrameletParameters = FrameletParameters {
    id: -61504,
    k1: -5.962_420_945_566_733e-8,
    k2: 2.738_191_004_225_615e-14,
    cx: 814.21,
    cy: 315.48,
    focal_length: 10.95637,
    pixel_size: 0.0074,
};

pub static JUNO_JUNOCAM_BLUE: FrameletParameters = FrameletParameters {
    id: -61501,
    k1: -5.962_420_945_566_733e-8,
    k2: 2.738_191_004_225_615e-14,
    cx: 814.21,
    cy: 158.48,
    focal_length: 10.95637,
    pixel_size: 0.0074,
};

pub static JUNO_JUNOCAM: FrameletParameters = FrameletParameters {
    id: -61500,
    k1: -5.962_420_945_566_733e-8,
    k2: 2.738_191_004_225_615e-14,
    cx: 814.21,
    cy: 78.48,
    focal_length: 10.95637,
    pixel_size: 0.0074,
};

pub static JUNO_JUNOCAM_GREEN: FrameletParameters = FrameletParameters {
    id: -61502,
    k1: -5.962_420_945_566_733e-8,
    k2: 2.738_191_004_225_615e-14,
    cx: 814.21,
    cy: 3.48,
    focal_length: 10.95637,
    pixel_size: 0.0074,
};

pub static JUNO_JUNOCAM_RED: FrameletParameters = FrameletParameters {
    id: -61503,
    k1: -5.962_420_945_566_733e-8,
    k2: 2.738_191_004_225_615e-14,
    cx: 814.21,
    cy: -151.52,
    focal_length: 10.95637,
    pixel_size: 0.0074,
};
