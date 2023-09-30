![banner](assets/gen_me_a_pixamon_logo.jpg)

## Generate Random Pixel Creatures

Simple code that generate PNG Pixel Art of Little Monsters.
Perfect for :
- Table-Top RPGs :game_die:
- Video Games :video_game:
- Cryptography :key:
- Database Visualisation :card_index_dividers:

**Note :** *I used this to assign each [EAN Barcode](https://en.wikipedia.org/wiki/International_Article_Number) in a database manager a cool low-rez monster.*

### Dependencies

### How to use

1. [Download Rust](https://www.rust-lang.org/tools/install)
2. Choose a number between 0 and 2^64 - 1
3. Generate your monster making sure to pass your chosen number in as a commmand line argument.
```bash
# Where "seed" is an unsigned 64 bit integer
cargo run seed
```

```bash
# make an .exe
cargo build --release
# run executable and create the 0th monster as png
target/release/gen_me_a_pixamon 0

if test -f monsters/mon0.bmp; then
  echo "Hurray! You made your first Pixamon!"
fi
```

### Customise

#### Monster location
You can change the location of the creature relative to the canvas.
```rust
/* make_monster() takes the Vec2 struct as an argument.
Passing in Vec2{x:16, y:2} would position the monster
16 pixels right and 2 pixels down from the top right corner of the PNG */ 
let location : Vec2 = make_vec(16, 8);
let _monster = make_monster(0, 8, location);
```

#### Monster visual complexity
A pixamon's algorithm uses a pixel-by-pixel process.   
Passing a larger bitmap into make_monster() gives the alorithm more opportunities for complexity.   
Note: this is not the dimensions of the final PNG file. We can modify the resolution of the final PNG later.
```rust
// Modify this value here to increase the number of pixels that make up a monster.
let size : u32 = 32;

// Make a monster with 32*32 bitmap.
let monster = make_monster(0, size, make_vec(0,0);
assert_eq!(monster.get_width(), size);
assert_eq!(monster.get_height(), size);
```

#### Change the size of the output PNG
For larger images, *gen' me a PIXAMON* provides a resize function.
The function uses [proximal interpolation](https://en.wikipedia.org/wiki/Nearest-neighbor_interpolation) (recommended for pixel art).
```rust
use fastrand;

// make a monster with a bitmap of unknown dimensions.
let any_size_will_do : i32 = fastrand::i32(0..64);
let monster = make_monster(0, any_size_will_do, make_vec(0,0);

// Pick a size
let desired_img_size : u32 = 128;

// calculate and apply a scaling factor
let factor : f32 = (desired_img_size as f32) / (size as f32);
let img : Image = bmp_resize(monster, factor);
assert_eq!(img.get_width(), desired_img_size);
assert_eq!(img.get_height(), desired_img_size);
```

### Examples

### Credits
*In loving memory of [PixelEncounter](https://pixelencounter.com/) by [Josh Gomez](https://github.com/XzaR90/PixelEncounter)*

- Based on the work of [KilledByAPixel](https://github.com/KilledByAPixel/ZzSprite/blob/master/ZzSprite.js) 
- Original alogrithm by [FireFly](https://www.dwitter.net/d/3078)
- Thanks to [Andras](https://github.com/fordosa90) for helping me track down and thank Josh.

#### A quick story
When I was just starting out programming professionally, my work required a barcode management system.  
Remembering a weird toy from the 2000s called [Skannerz](https://en.wikipedia.org/wiki/Skannerz), I thought it would be a cute UI feature to give each EAN barcode a unique monster.
An excellent use of company recources!  
I found an online API called "Pixel Encounter" and wrote some Visual Basic code to utilise it.  
A critical warehousing system I was responsible for now depended on someone else's fun website for D&D players. :man_facepalming:  
In 2023 the maker of "Pixel Encounter" moved on to new projects. (Thankfully, I had wrapped the function in a try block)  
However, by this point the staff were attached to their barcode creatures and making my own was going to become a weekend project. 

Pixel Encounter was my first experience with online APIs. It was easy to use and a lot of fun.
Thanks Josh.
