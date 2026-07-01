/*
 * File: grid.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 11:21:32 pm                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Wednesday, July 1st 2026, 6:38:03 pm                         *
 * By: LALIN Romain                                                            *
 * ----------	---	---------------------------------------------------------  *
*/

pub struct Grid {
    case: Vec[u32],
    is_complete: bool,
    is_blocked: bool,
}

impl Grid {
    pub fn new(grid: &Vec<u32>) -> Self {
        Self {
            case: grid,
            is_complete: false,
            is_blocked: true,
        }
    }

    pub fn get_case(&self, id: usize) -> u32 {
        self.case[id]
    }

    pub fn get_all_case(&self) -> [u32; 81] {
        self.case.clone()
    }

    pub fn set_case(&self, id_case:usize, value:u32) {
        self.case[id_case] = value;
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn is_blocked(&self) -> bool {
        self.is_blocked
    }

    pub fn set_blocked(&self, _is_blocked:bool) {
        self.is_blocked = _is_blocked;
    }

    pub fn set_complete(&self, _is_complete:bool) {
        self.is_complete = _is_complete;
    }

    pub fn check_complete(&self) -> bool {
        for c in self.case {
            if c == 0 {
                return false;
            }
        }
        true
    }
}