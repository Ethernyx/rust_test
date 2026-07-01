/*
 * File: main.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 10:12:27 am                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Wednesday, July 1st 2026, 4:00:37 pm                         *
 * By: LALIN Romain                                                            *
 * ----------	---	---------------------------------------------------------  *
*/

mod grid;
mod case;

use std::env;
use grid::Grid;
use case::Case;

fn main() {
    let av: Vec<String> = env::args().collect();
    let _ac = av.len();

    let grid: Grid = Grid::new();
    if ac > 0 { for (arg in av) { println!("{}", arg); }}
}
