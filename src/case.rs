/*
 * File: case.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 11:21:07 pm                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Friday, July 3rd 2026, 5:07:20 pm                            *
 * By: LALIN Romain                                                            *
 * ----------	---	---------------------------------------------------------  *
*/

use crate::grid::Grid;

pub struct Case {
    carre: Vec<u32>,
    line_h: Vec<u32>,
    line_v: Vec<u32>,
    possibility: Vec<u32>,
    value: u32,
}

impl Case {
    pub fn new(value:u32) -> Self {
        Self {
            carre: Vec::new(),
            line_h: Vec::new(),
            line_v: Vec::new(),
            possibility: Vec::new(),
            value: value,
        };
    }

    pub fn change_case(&mut self, grid: &Grid, id_case:u32) {
        self.clear();
        self.fill(grid, id_case);
    }

    pub fn get_all_carre(&self) -> Vec<u32> {
        self.carre.clone()
    }

    pub fn get_all_possibility(&self) -> Vec<u32> {
        self.possibility.clone()
    }

    pub fn get_all_line_h(&self) -> Vec<u32> {
        self.line_h.clone()
    }

    pub fn get_all_line_v(&self) -> Vec<u32> {
        self.line_v.clone()
    }

     pub fn get_carre(&self, id:usize) -> u32 {
        self.carre[id]
    }

    pub fn get_line_h(&self, id:usize) -> u32 {
        self.line_h[id]
    }

    pub fn get_line_v(&self, id:usize) -> u32 {
        self.line_v[id]
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    fn fill(&mut self, grid: &Grid, id_case: u32) {
        self.value = grid.get_case(id_case as usize).get_value();
        self.fill_col(grid, id_case);
        self.fill_line(grid, id_case);
        self.fill_carre(grid, id_case);
    }

    fn fill_col(&mut self, grid: &Grid, id_case: u32) {
        let col:u32 = id_case % 9;

        // je récupère toute les valeurs de la même colonne
        for i in 0..9 {
            if id_case != i * 9 + col {
                self.line_v.push(grid.get_case((i * 9 + col) as usize).get_value());
            }
        }
    }

    fn fill_line(&mut self, grid: &Grid, id_case: u32) {
        let line:u32 = id_case / 9;

        // je récupère toute les valeurs de la même ligne 
        for i in 0..9 {
            if id_case != line * 9 + i {
                self.line_h.push(grid.get_case((line * 9 + i) as usize).get_value());
            }
        }
    }

    fn fill_carre(&mut self, grid: &Grid, id_case: u32) {
        let col:u32 = id_case % 9;
        let line:u32 = id_case / 9;

        // je récupère toute les valeurs du même carré 
        for i in 0..81 {
            if (i / 9) / 3 == line / 3 && (i % 9) / 3 == col / 3 && i != id_case {
                self.carre.push(grid.get_case(i as usize).get_value());
            }
        }
    }

    fn clear(&mut self) {
        self.carre.clear();
        self.line_h.clear();
        self.line_v.clear();
    }
}