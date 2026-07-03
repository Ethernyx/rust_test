/*
 * File: grid.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 11:21:32 pm                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Friday, July 3rd 2026, 5:06:54 pm                            *
 * By: LALIN Romain                                                            *
 * ----------	---	---------------------------------------------------------  *
*/

use crate::case::Case;

pub struct Grid {
    case: Vec<Case>,
    is_complete: bool,
    is_blocked: bool,
}

impl Grid {
    pub fn new(grid: Vec<u32>) -> Self {
        let my_grid = Self {
            case: Vec::new(),
            is_complete: false,
            is_blocked: true,
        };
        for value in grid {
            my_grid.case.push(Case::new(value));
        }
        for (id, case) in my_grid.get_all_case().iter().enumerate() {
            case.change_case(my_grid.case, id)
        }
        my_grid
    }

    pub fn get_case(&self, id: usize) -> u32 {
        self.case[id]
    }

    pub fn get_all_case(&self) -> Vec<u32> {
        self.case.clone()
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn is_blocked(&self) -> bool {
        self.is_blocked
    }

    pub fn set_blocked(&mut self, _is_blocked:bool) {
        self.is_blocked = _is_blocked;
    }

    pub fn set_complete(&mut self, _is_complete:bool) {
        self.is_complete = _is_complete;
    }

    pub fn check_complete(&mut self) -> bool {
        for c in self.case.clone() {
            if c.get_value() == 0 {
                self.set_complete(false);
                return false;
            }
        }
        self.set_complete(true);
        true
    }
}