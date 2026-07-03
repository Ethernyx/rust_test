/*
 * File: main.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 10:12:27 am                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Friday, July 3rd 2026, 5:04:11 pm                            *
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

    let mut grid: Grid = Grid::new(vec![2,1,0,0,8,0,0,6,4,5,3,7,4,6,0,8,0,9,0,0,6,0,3,1,0,0,0,0,0,4,0,0,0,1,0,2,0,5,2,0,0,0,9,3,0,3,0,1,0,0,0,7,0,0,0,0,0,3,5,0,6,0,0,1,0,5,0,2,9,4,7,3,9,7,0,0,1,0,0,2,8]);
   // let mut case:Case = Case::new(0);
    let mut possibility:Vec<u32> = Vec::new();

    grid.set_blocked(false);
    while !grid.is_complete() || grid.is_blocked() {
        grid.set_blocked(true);
        for i in 0..81 {
            if grid.get_case(i).get_value() == 0 {
                possibility.clear();
                for j in 1..=9 {
                    if !grid.get_case(i).get_all_carre().contains(&j) 
                    && !grid.get_case(i).get_all_line_h().contains(&j) 
                    && !grid.get_case(i).get_all_line_v().contains(&j) {
                        grid.get_case(i).get_all_possibility().push(j);
                    }
                }
                if grid.get_case(i).get_all_possibility().len() == 1 {
                    grid.get_case(i as usize).change_case(grid, possibility[0]);
                    grid.set_blocked(false);
                    grid.check_complete();
                }
            }
            if grid.is_complete() { break; }
        }
    }
    let mut code = "";
    for (id, case) in grid.get_all_case().iter().enumerate() {
        if id % 9 == 0 && (id / 9) % 3 == 0 && id != 0 { print!("|\n-------------------------------\n"); }
        else if id % 9 == 0 && (id / 9) % 3 == 0 { print!("\n-------------------------------\n"); }
        else if id % 9 == 0 { print!("|\n"); }
        code = if *case == 0 { "\x1b[90m" } else { "\x1b[32m" };
        if id % 3 == 0 { print!("|"); } 
        print!("{} {} \x1b[0m", code, case);
    }
 print!("|\n-------------------------------\n");
}
