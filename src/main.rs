use std::env;
use bmp::Image;
use bmp::Pixel;
use fastrand;

struct Vec2 {
    x : u32,
    y : u32,
}

fn make_vec(i:u32, j:u32) -> Vec2 {
    return Vec2 {x : i, y : j};
}

fn fmap(a_rge: (f32, f32), b_rge: (f32, f32), c: f32) -> f32 {
    //maps C from range A0-A1 to range B0-B1
    b_rge.0 + (c - a_rge.0) * (b_rge.1 - b_rge.0) / (a_rge.1 - a_rge.0)
}

fn extract_seed_from_args(args : Vec<String>) -> u64 {
    for arg in args {
        match arg.parse::<u64>() {
            Ok(seed) => return seed,
            Err(_) => continue, 
        } 
    }
    panic!("gen_me_a_pixamon requires an usigned 64bit int passed in as argument.")
}

fn bmp_resize(bmp : Image, fct : f32) -> Image {
    // upscale image using proximal interpolation.

    // get dimensions
    let bmp_w : f32 = bmp.get_width() as f32;
    let bmp_h : f32 = bmp.get_height() as f32;
    let new_w : f32 = (fct * bmp_w).floor();
    let new_h : f32 = (fct * bmp_h).floor();

    // init a scaled image to output
    let mut img = Image::new(new_w as u32, new_h as u32);

    // assign colours to scaled image by indexing the input image
    let mut j : f32 = 0.0;
    while j < new_h {
        let mut i : f32 = 0.0;
        while i < new_h {
            //find nearest neighbour
            let x : u32 = fmap((0.0,new_w), (0.0,bmp_w), i).floor() as u32;
            let y : u32 = fmap((0.0,new_h), (0.0,bmp_h), j).floor() as u32;

            // take neighbours pixel
            let pix : Pixel = bmp.get_pixel(x, y);
            img.set_pixel(i as u32, j as u32, pix);

            i = i + 1.0;
        }
        j = j + 1.0;
    }

    return img;
}

fn make_monster(seed : u64, size : u32, loc : Vec2) -> Image {
    
    // initialise the rng
    fastrand::seed(seed);

    // initialise the bmp
    let mut monster = Image::new(size, size);

    // recast size to a i32 (we wanted to force a positive number up till now)
    let v : i32 = size as i32;

    // chance to flip drawing axis
    let flip_axis : bool = fastrand::bool();

    // width and height of monster
    let w : i32 = if flip_axis {v - 3} else {(v / 2) - 1};
    let h : i32 = if !flip_axis {v - 3} else {(v / 2) - 1};

    // biased inputs
    let density : f32 = fmap((0.0,1.0), (0.3,0.8), fastrand::f32());
    let y_bias : f32 = fmap((0.0,1.0), (0.2, -0.2), fastrand::f32());
    let colour_rand : f32 = fmap((0.0,1.0), (0.75, 0.95), fastrand::f32());

    //Loop through pixels
    for k in 0..(w*h) {

        // global position based on monster dimensions
        let i : i32 = if flip_axis {k / w} else {k % w};
        let j : i32 = if !flip_axis {k / w} else {k % w};

        // calculate biasses
        let is_hole : bool = fastrand::f32() > density;
        let a_scalar : i32 = fastrand::i32(0..v/2) * fastrand::i32(0..v / 2);
        let y_scalar : i32 = (1.0 - 2.0 * y_bias) as i32;
        let xy_scalar : i32 = (i * i) + (j - y_scalar * (h / 2)).pow(2);
        let bias : bool = a_scalar > xy_scalar;

        if bias && !is_hole {
            // recast x and y to usigned
            let x : u32 = i as u32;
            let y : u32 = j as u32;

            // pick new random colour using weighted by the colour seed
            let r : u8 = (fastrand::f32() * 255.0 * colour_rand) as u8;
            let g : u8 = (fastrand::f32() * 255.0 * colour_rand) as u8;
            let b : u8 = (fastrand::f32() * 255.0 * colour_rand) as u8;
            
            // draw
            let pix = Pixel::new(r, g, b);
            let px_left : u32 = (loc.x + x) % monster.get_width();
            let px_right : u32 = (loc.x - x) % monster.get_width();
            let py : u32 = (loc.y + y) % monster.get_height();
            monster.set_pixel(px_left, py, pix);
            monster.set_pixel(px_right, py, pix);
        }
    }
    return monster;
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // make a monster
    let seed : u64 = extract_seed_from_args(env::args().collect());
    let size : u32 = 32;
    let location : Vec2 = make_vec(size / 2, 2);
    let monster : Image = make_monster(seed, size, location);

    // resize the monster to a desired resolution(img_size)
    let img_size : u32 = 128;
    let factor : f32 = (img_size as f32) / (size as f32);
    let img : Image = bmp_resize(monster, factor);

    // save to disk @ ./monsters/
    let filename : String = format!("monsters/mon{}.bmp", seed);
    img.save(filename).ok();
}