/*
 * File: grid.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 11:21:32 pm                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Friday, June 26th 2026, 11:21:50 pm                          *
 * By: LALIN Romain                                                            *
 * ----------	---	---------------------------------------------------------  *
*/

struct Grid {
    case: [u32; 81],
    is_complete: bool,
    is_blocked: bool,
}

impl Grid {
    fn new() -> Self {
        Self {
            case: [0; 81],
            is_complete: false,
            is_blocked: true,
        }
    }
}