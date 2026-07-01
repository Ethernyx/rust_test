/*
 * File: main.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 10:12:27 am                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Wednesday, July 1st 2026, 6:37:38 pm                         *
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

    let grid: Grid = Grid::new(vec![2,1,0,0,8,0,0,6,4,5,3,7,4,6,0,8,0,9,0,0,6,0,3,1,0,0,0,0,0,4,0,0,0,1,0,2,0,5,2,0,0,0,9,3,0,3,0,1,0,0,0,7,0,0,0,0,0,3,5,0,6,0,0,1,0,5,0,2,9,4,7,3,9,7,0,0,1,0,0,2,8]);

    grid.set_blocked(false);
    while !grid.is_complete || grid.is_blocked {
        grid.set_blocked(true);
        let case:Case;
        let possibility:Vec<u32>;
        for i in 0..81 {
            if i == 0 case = Case::new(&grid, 0);
            else case.change_case(&grid, i);
            if case.value == 0 {
                possibility.clear();
                for j in 1..9 {
                    if !case.get_all_carre().contains(j) && !case.get_all_line_h().contains(j) && !case.get_all_line_v().contains(j) {
                        possibility.push(j);
                    }
                }
                if possibility.len() == 1 {
                    grid.set_case(i, possibility[0]);
                    grid.set_blocked(false);
                    grid.check_complete();
                }
            }
            if (grid.is_complete) break;
        }
    }
    
    for case in &grid.get_all_case { println!("{}", case); }
}
