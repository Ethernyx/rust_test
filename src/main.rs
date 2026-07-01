/*
 * File: main.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 10:12:27 am                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Friday, June 26th 2026, 11:24:10 pm                          *
 * By: LALIN Romain                                                            *
 * ----------	---	---------------------------------------------------------  *
*/

use std::env;
use Grid;
use Case;

fn main() {
    let av: Vec<String> = env::args().collect();
    let _ac = av.len();

    let grid: Grid = Grid::new();
   // if (ac > 0) for (arg in av) println!("{}", arg);
}
