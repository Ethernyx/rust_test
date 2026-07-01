/*
 * File: case.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 11:21:07 pm                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Friday, June 26th 2026, 11:21:22 pm                          *
 * By: LALIN Romain                                                            *
 * ----------	---	---------------------------------------------------------  *
*/

struct Case {
    carre: [u32; 8],
    line_h: [u32; 8],
    line_v: [u32; 8],
    value: u32,
}

impl Case {
    fn new(_value: u32) -> Self {
        Self {
            carre: [0; 8],
            line_h: [0; 8],
            line_v: [0; 8],
            value: _value,
        }
    }
}