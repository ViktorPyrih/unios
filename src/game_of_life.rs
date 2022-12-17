use crate::vga_buffer::{Screen};

const MAP: [&str; 25] = [
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                    x                                           ",
    "                                  x x                                           ",
    "                        xx      xx            xx                                ",
    "                       x   x    xx            xx                                ",
    "            xx        x     x   xx                                              ",
    "            xx        x   x xx    x x                                           ",
    "                      x     x       x                                           ",
    "                       x   x                                                    ",
    "                        xx                                                      ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                "
];

const LIVE_CELL: u8 = b'x';
const DEAD_CELL: u8 = b' ';

pub fn game_of_life(screen: &mut Screen) -> ! {
    let mut current_gen: [[u8; 80]; 25] = [[0; 80]; 25];
    for i in 0..MAP.len() {
        for (j, byte) in MAP[i].bytes().enumerate() {
            current_gen[i][j] = byte;
        }
    }

    screen.write_arr(&current_gen);

    loop {
        current_gen = get_next_gen(&current_gen);
        screen.write_arr(&current_gen);
        wait();
    }
}

fn get_next_gen(gen: &[[u8; 80]; 25]) -> [[u8; 80]; 25] {

    let mut next_gen: [[u8; 80]; 25] = [[0; 80]; 25];

    for i in 0..25 {
        for j in 0..80 {
            let neighbours = get_neighbours_count(&gen, &i, &j);
            let cell_type = gen[i as usize][j as usize];

            if cell_type == LIVE_CELL && (neighbours == 2 || neighbours == 3){
                next_gen[i as usize][j as usize] = LIVE_CELL;
            }
            else if cell_type == DEAD_CELL && neighbours == 3{
                next_gen[i as usize][j as usize] = LIVE_CELL;
            }
            else{
                next_gen[i as usize][j as usize] = DEAD_CELL;
            }
        }
    }

    return next_gen;
}

fn get_neighbours_count(gen: &[[u8; 80]; 25], i: &usize, j: &usize) -> i32 {
    let mut count : i32 = 0;

    if i > &0 && gen[i - 1][*j] == LIVE_CELL{
        count += 1;
    }
    if i > &0 && j > &0 && gen[i - 1][j - 1] == LIVE_CELL{
        count += 1;
    }
    if i > &0 && j + 1 < 80 && gen[i - 1][j + 1] == LIVE_CELL{
        count += 1;
    }
    if i + 1 < 25 && gen[i + 1][*j] == LIVE_CELL{
        count += 1;
    }
    if i + 1 < 25 && j > &0 && gen[i + 1][j - 1] == LIVE_CELL{
        count += 1;
    }
    if i + 1 < 25 && j + 1 < 80 && gen[i + 1][j + 1] == LIVE_CELL{
        count += 1;
    }
    if j > &0 && gen[*i][j - 1] == LIVE_CELL{
        count += 1;
    }
    if j + 1 < 80 && gen[*i][j + 1] == LIVE_CELL{
        count += 1;
    }

    return count;
}

fn wait() {
    for _ in 0..100000 {}
}
