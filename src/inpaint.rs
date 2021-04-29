


// https://www.researchgate.net/publication/238183352_An_Image_Inpainting_Technique_Based_on_the_Fast_Marching_Method


use crate::{
    constants, 
    path, 
    error, 
    enums, 
    imagebuffer::ImageBuffer, 
    vprintln,
    stats,
    ok
};

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
    score: u32
}

struct RgbVec {
    rgb: Vec<f32>,
    width: usize,
    height: usize
}

const DEFAULT_WINDOW_SIZE : i32 = 3;

fn determine_mask_file(instrument:enums::Camera) -> error::Result<&'static str> {
    match instrument {
        enums::Camera::RED => Ok(constants::cal::JNO_INPAINT_MASK_RED),
        enums::Camera::GREEN => Ok(constants::cal::JNO_INPAINT_MASK_GREEN),
        enums::Camera::BLUE => Ok(constants::cal::JNO_INPAINT_MASK_BLUE),
        _ => Err(constants::status::INVALID_ENUM_VALUE)
    }
}

pub fn inpaint_supported_for_camera(camera:enums::Camera) -> bool {
    let r = determine_mask_file(camera);
    match r {
        Ok(_) => true,
        Err(_) => false
    }
}

fn load_mask_file(filename:&str) -> error::Result<ImageBuffer> {
    vprintln!("Loading inpaint mask file {}", filename);

    if ! path::file_exists(filename) {
        return Err(constants::status::FILE_NOT_FOUND);
    }
    let mask = match ImageBuffer::from_file(filename) {
        Ok(m) => m,
        Err(e) => return Err(e)
    };
    
    Ok(mask)
}

fn load_mask(camera:enums::Camera) -> error::Result<ImageBuffer> {
    let mask_file = match determine_mask_file(camera) {
        Ok(m) => m,
        Err(e) => return Err(e)
    };

    load_mask_file(mask_file)
}

fn get_num_good_neighbors(mask:&ImageBuffer, x:i32, y:i32) -> u32 {

    // Juggling the possibility of negitive numbers and whether or now we allow that.
    let t = if y > 0 { mask.get(x as usize, (y-1) as usize).unwrap() == 0.0 } else { false };
    let tl = if x > 0 && y > 0 { mask.get((x-1) as usize, (y-1) as usize).unwrap() == 0.0 } else { false };
    let l = if x > 0 { mask.get((x-1)  as usize, y as usize).unwrap() == 0.0 } else { false };
    let bl = if x > 0 && y < mask.height as i32 - 1 { mask.get((x-1) as usize, (y+1) as usize).unwrap() == 0.0 } else { false };
    let b = if y < mask.height as i32 - 1 { mask.get(x as usize, (y+1) as usize).unwrap() == 0.0 } else { false };
    let br = if x < mask.width as i32 - 1 && y < mask.height as i32 - 1 { mask.get((x+1) as usize, (y+1) as usize).unwrap() == 0.0 } else { false };
    let r = if x < mask.width as i32 - 1 { mask.get((x+1) as usize, y as usize).unwrap() == 0.0 } else { false };
    let tr = if x < mask.width as i32 - 1 && y > 0 { mask.get((x+1) as usize, (y-1) as usize).unwrap() == 0.0 } else { false };

    let mut s = 0;

    s += if t  { 1 } else { 0 };
    s += if tl { 1 } else { 0 };
    s += if l  { 1 } else { 0 };
    s += if bl { 1 } else { 0 };
    s += if b  { 1 } else { 0 };
    s += if br { 1 } else { 0 };
    s += if r  { 1 } else { 0 };
    s += if tr { 1 } else { 0 };

    s
}

// SOOOOOOooooooooo sloooooooooooooooow :-(
fn find_starting_point(mask:&ImageBuffer) -> Option<Point> {
    for y in 0..mask.height {
        for x in 0..mask.width {
            match mask.get(x, y) {
                Ok(v) => {
                    if v > 0.0 {
                        return Some(Point{x:x, y:y, score:0});
                    }
                }
                _ => ()
            }
        }
    }
    None
}

fn isolate_window(buffer:&RgbVec, mask:&ImageBuffer, window_size:i32, x:usize, y:usize) -> error::Result<Vec<f32>> {
    let mut v:Vec<f32> = Vec::with_capacity(36);
    let start = window_size / 2 * -1;
    let end = window_size / 2 + 1;
    for _y in start..end as i32 {
        for _x in start..end as i32 {
            let get_x = x as i32 + _x;
            let get_y = y as i32 + _y;
            if get_x >= 0 
                && get_x < buffer.width as i32 
                && get_y >= 0 
                && get_y < buffer.height as i32
                && mask.get(get_x as usize, get_y as usize).unwrap() == 0.0
                {
                v.push(buffer.rgb[(get_y * buffer.width as i32 + get_x) as usize]);
            }
        }
    }
    Ok(v)
}

fn predict_value(buffer:&RgbVec, mask:&ImageBuffer, x:usize, y:usize) -> f32 {
    let window = isolate_window(&buffer, &mask, DEFAULT_WINDOW_SIZE, x, y).unwrap();
    let m = stats::mean(&window[0..]).unwrap();
    m
}


fn get_point_and_score_at_xy(mask:&ImageBuffer, x:i32, y:i32) -> Option<Point> {

    if x < 0 || x >= mask.width as i32 || y < 0 || y >= mask.height as i32 {
        return None;
    }

    let v = mask.get(x as usize, y as usize).unwrap();
    if v == 0.0 {
        return None;
    }

    let score = get_num_good_neighbors(&mask, x, y);

    Some(Point{x:x as usize, y:y as usize, score:score})
}


fn find_larger(left:Option<Point>, right:&Point) -> Option<Point> {
    match left {
        Some(pt) => {
            let m = if pt.score > right.score { pt } else { right.clone() };
            Some(m)
        },
        None => return Some(right.to_owned())
    }
}

fn find_next_point(mask:&ImageBuffer, x:i32, y:i32) -> Option<Point> {
    let mut pts : Vec<Option<Point>> = Vec::with_capacity(8);

    pts.push(get_point_and_score_at_xy(&mask, x, y - 1));
    pts.push(get_point_and_score_at_xy(&mask, x - 1, y - 1));
    pts.push(get_point_and_score_at_xy(&mask, x - 1, y));
    pts.push(get_point_and_score_at_xy(&mask, x - 1, y + 1));
    pts.push(get_point_and_score_at_xy(&mask, x, y + 1));
    pts.push(get_point_and_score_at_xy(&mask, x + 1, y + 1));
    pts.push(get_point_and_score_at_xy(&mask, x + 1, y));
    pts.push(get_point_and_score_at_xy(&mask, x + 1, y - 1));

    let mut largest_score : Option<Point> = None;

    for opt_pt in pts.iter() {
        match opt_pt {
            Some(pt) => {
                largest_score = find_larger(largest_score, pt);
            },
            None => ()
        }
    }

    largest_score
}


fn infill(buffer:&mut RgbVec, mask:&mut ImageBuffer, starting:&Point) -> error::Result<&'static str> {

    let mut current = starting.to_owned();
    loop {
        let pt_new_value = predict_value(&buffer, &mask, current.x, current.y);
        
        buffer.rgb[current.y * buffer.width + current.x] = pt_new_value;

        match mask.put(current.x, current.y, 0.0) {
            Ok(_) => (),
            Err(e) => return Err(e)
        }

        match find_next_point(&mask, current.x as i32, current.y as i32) {
            Some(pt) => current = pt.to_owned(),
            None => break
        }
    }
    ok!()
}

fn image_to_vec(img:&ImageBuffer) -> error::Result<RgbVec> {

    let mut v: Vec<f32> = Vec::with_capacity(img.width * img.height);
    v.resize(img.width * img.height, 0.0);

    for y in 0..img.height {
        for x in 0..img.width {
            let idx = y * img.width + x;
            let r = match img.get(x, y) {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            v[idx]= r;
        }
    }

    Ok(RgbVec{rgb:v, width:img.width, height:img.height})
}

fn vec_to_image(buffer:&RgbVec) -> error::Result<ImageBuffer> {
    let mut img = ImageBuffer::new(buffer.width, buffer.height).unwrap();

    for y in 0..buffer.height {
        for x in 0..buffer.width {
            let r = buffer.rgb[y * (buffer.width) + x];
            match img.put(x, y, r) {
                Ok(_) => (),
                Err(e) => return Err(e)
            };
        }
    }

    Ok(img)
}




// Embarrassingly slow and inefficient. Runs slow in debug. A lot faster with a release build.
pub fn apply_inpaint_to_buffer_with_mask(img:&ImageBuffer, mask_src:&ImageBuffer) -> error::Result<ImageBuffer> {

    let mut working_buffer = match image_to_vec(&img) {
        Ok(b) => b,
        Err(e) => return Err(e)
    };

    let mut mask = mask_src.clone();

    // For this to work, we need the mask to be mutable and we're
    // going to fill it in with 0x0 values as we go. If we don't, then
    // we'll keep finding starting points and this will be an infinite
    // loop. Which is bad. Perhaps consider an alternate method here.
    loop {

        // TODO: Don't leave embedded match statements. I hate that as much as embedded case statements...
        match find_starting_point(&mask) {
            Some(pt) => {
                //vprintln!("Starting point: {}, {}", pt.x, pt.y);
                match infill(&mut working_buffer, &mut mask, &pt) {
                    Ok(_) => (),
                    Err(e) => return Err(e)
                };
            },
            None => break
        };
    }

    let newimage = match vec_to_image(&working_buffer) {
        Ok(i) => i,
        Err(e) => return Err(e)
    };

    Ok(newimage)
}


pub fn apply_inpaint_to_buffer(img:&ImageBuffer, camera:enums::Camera) -> error::Result<ImageBuffer> {

    let mask = match load_mask(camera) {
        Ok(m) => m,
        Err(_) => return Err("Error loading mask")
    };

    apply_inpaint_to_buffer_with_mask(&img, &mask)
}
