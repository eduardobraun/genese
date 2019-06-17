use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 190;
const SCREEN_HEIGHT: i32 = 100;

const LIMIT_FPS: i32 = 20;

const COLOR_WATER: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LAND: Color = Color { r: 0, g: 100, b: 0 };

#[derive(Clone)]
struct CellInfo {}

#[derive(Clone)]
enum MapCell {
    Land(CellInfo),
    Ocean(CellInfo),
}

pub fn main() {
    let mut map_info: Vec<Vec<MapCell>> = vec![vec![MapCell::Ocean(CellInfo {}); 640]; 200];

    for row in map_info.iter_mut() {
        for cell in row.iter_mut() {
            *cell = MapCell::Land(CellInfo {});
        }
    }

    let mut con = Offscreen::new(640, 200);

    let mut root = Root::initializer()
        .font("Potash_10x10.png", FontLayout::AsciiInRow)
        .font_type(FontType::Default)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    tcod::system::set_fps(LIMIT_FPS);

    while !root.window_closed() {
        con.set_default_foreground(WHITE);
        con.clear();

        // for (i, row) in map_info.iter().enumerate() {
        //     for (j, cell) in row.iter().enumerate() {
        //         let x: i32 = j as i32;
        //         let y: i32 = i as i32;

        //         match cell {
        //             MapCell::Ocean(_) => {
        //                 con.set_char_background(x, y, COLOR_WATER, BackgroundFlag::Set);
        //                 con.put_char(x, y, '~', BackgroundFlag::None);
        //             }
        //             MapCell::Land(_) => {
        //                 con.set_char_background(x, y, COLOR_LAND, BackgroundFlag::Set);
        //                 con.put_char(x, y, '`', BackgroundFlag::None);
        //             }
        //         };
        //     }
        // }

        for i in 0..16 {
            for j in 0..16 {
                let c: char = (j + i * 16) as u8 as char;
                con.put_char(j, i, c, BackgroundFlag::None);
            }
        }

        blit(
            &mut con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut root,
            (0, 0),
            1.0,
            1.0,
        );

        root.flush();
        root.wait_for_keypress(true);
    }

    println!("Hello mapviewer");
}
