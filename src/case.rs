/*
 * File: case.rs                                                               *
 * Project: src                                                                *
 * Created Date: Fr Jun 2026, 11:21:07 pm                                      *
 * Author: LALIN Romain                                                        *
 * -----                                                                       *
 * Last Modified: Wednesday, July 1st 2026, 5:13:24 pm                         *
 * By: LALIN Romain                                                            *
 * ----------	---	---------------------------------------------------------  *
*/

pub struct Case {
    carre: Vec<u32>,
    line_h: Vec<u32>,
    line_v: Vec<u32>,
    value: u32,
}

impl Case {
    pub fn new(_value: u32) -> Self {
        Self {
            carre: vec![0, 0, 0, 0, 0, 0, 0, 0],
            line_h: vec![0, 0, 0, 0, 0, 0, 0, 0],
            line_v: vec![0, 0, 0, 0, 0, 0, 0, 0],
            value: _value,
        }
    }

    pub fn get_carre(&self) -> Vec<u32> {
        self.carre
    }

    pub fn et_line_h(&self) -> Vec<u32> {
        self.line_h
    }

    pub fn get_line_v(&self) -> Vec<u32> {
        self.line_v
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn fill(&mut self, grid: Grid, id_case: u32) {
        self.value = grid[id_case];
        self.fill_col(grid, id_case);
        self.fill_line(grid, id_case);
        self.fill_carre(grid, id_case);
    }

    fn fill_col(&mut self, grid: Grid, id_case: u32) {
        let mut col:u32 = id_case % 9;

        // je récupère toute les valeurs de la même colonne
        for i in 0..9 {
            if id_case != i * 9 + col {
                self.line_v.push(grid.get_case(i * 9 + col));
            }
        }
    }

    fn fill_line(&mut self, grid: Grid, id_case: u32) {
        let mut line:u32 = id_case / 9;

        // je récupère toute les valeurs de la même ligne 
        for i in 0..9 {
            if id_case != line * 9 + i {
                self.line_h.push(grid.get_case(line * 9 + i));
            }
        }
    }

    fn fill_carre(&mut self, grid: Grid, id_case: u32) {
        let mut col:u32 = id_case % 9;
        let mut line:u32 = id_case / 9;

        // je récupère toute les valeurs du même carré 
        for i in 0..81 {
            if (i / 9) / 3 == line / 3 && (i % 9) / 3 == col / 3 && i != id_case {
                self.carre.push(grid.get_case(i));
            }
        }
    } 
}