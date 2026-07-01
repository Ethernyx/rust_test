/*
 * File: grid.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 11:21:32 pm                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Wednesday, July 1st 2026, 5:10:34 pm                         *
 * By: LALIN Romain                                                            *
 * ----------	---	---------------------------------------------------------  *
*/

pub struct Grid {
    case: [u32; 81],
    is_complete: bool,
    is_blocked: bool,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            case: [0; 81],
            is_complete: false,
            is_blocked: true,
        }
    }

    pub fn get_case(&self, id: u32) -> u32 {
        self.case[id]
    }

    pub fn get_case(&self) -> [u32; 81] {
        self.case
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn is_blocked(&self) -> bool {
        self.is_blocked
    }
}