/*
 * File: case.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 11:21:07 pm                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Wednesday, July 1st 2026, 3:44:36 pm                         *
 * By: LALIN Romain                                                            *
 * ----------	---	---------------------------------------------------------  *
*/

pub struct Case {
    carre: [u32; 8],
    line_h: [u32; 8],
    line_v: [u32; 8],
    value: u32,
}

impl Case {
    pub fn new(_value: u32) -> Self {
        Self {
            carre: [0; 8],
            line_h: [0; 8],
            line_v: [0; 8],
            value: _value,
        }
    }
}