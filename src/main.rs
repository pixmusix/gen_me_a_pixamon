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

// fn bmp_upscale(bmp : Image, a_sz : u32, b_sz : u32) -> Image {
    
// }

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
    let colour_rand : f32 = fmap((0.0,1.0), (0.3, 0.9), fastrand::f32());

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
            monster.set_pixel(loc.x + x, loc.y + y, pix);
            monster.set_pixel(loc.x - x, loc.y + y, pix);
        }
    }
    return monster;
}

fn main() {
    let seed : u64 = 0;
    let size : u32 = 21;
    let location : Vec2 = make_vec(size / 2, size / 4);
    let img : Image = make_monster(seed, size, location);
    let filename : String = format!("monsters/mon{}.bmp", seed);
    img.save(filename).ok();
}