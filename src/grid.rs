/*
 * File: grid.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 11:21:32 pm                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Monday, July 6th 2026, 10:28:14 am                           *
 * By: LALIN Romain                                                            *
 * ----------	---	---------------------------------------------------------  *
*/

use crate::case::Case;

pub struct Grid {
    values: Vec<u32>,
    case: Vec<Case>,
    is_complete: bool,
    is_blocked: bool,
}

impl Grid {
    pub fn new(grid: Vec<u32>) -> Self {
        let mut my_grid = Self {
            values: grid,
            case: Vec::new(),
            is_complete: false,
            is_blocked: true,
        };
        let snapshot = my_grid.values.clone();
        for id in 0..my_grid.case.len() {
            my_grid.case[id].change_case(&snapshot, id as u32);
        }

        my_grid
    }

    pub fn get_case(&self, id: usize) -> &Case {
        &self.case[id]
    }

    pub fn get_all_case(&self) -> Vec<Case> {
        self.case.clone()
    }

    pub fn get_all_values(&self) -> Vec<u32> {
        self.values.clone()
    }

    pub fn get_value(&self, id:usize) -> u32 {
        self.values[id]
    }

    pub fn set_value(&mut self, id: usize, value:u32) {
        self.values[id] = value;
        self.case[id].set_value(value);

         // recalcule uniquement les cases impactées (même ligne, colonne ou carré)
        let snapshot = self.values.clone();
        for other_id in 0..self.case.len() {
            if Self::affects(id, other_id) {
                self.case[other_id].change_case(&snapshot, other_id as u32);
            }
        }
    }

    fn affects(changed_id: usize, other_id: usize) -> bool {
        let (l1, c1) = (changed_id / 9, changed_id % 9);
        let (l2, c2) = (other_id / 9, other_id % 9);
        l1 == l2 || c1 == c2 || (l1 / 3 == l2 / 3 && c1 / 3 == c2 / 3)
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
        let complete = self.values.iter().all(|&v| v != 0);
        self.set_complete(complete);
        complete
    }
}