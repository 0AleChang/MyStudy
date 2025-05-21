use core::fmt;

use rand::Rng;


#[derive(Debug,Clone)]
pub struct Matrix{
    pub rows : usize ,
    pub cols : usize , 
    pub data : Vec<f64>,
}
//Creations
impl Matrix {
    pub fn new(rows: usize , cols : usize , data : Vec<f64>) -> Matrix{
        Matrix { rows, cols, data }
    }
    pub fn new_zeros(rows : usize , cols : usize ) -> Matrix { 
        Matrix { rows, cols, data: vec![0.0; cols * rows] }
    }
    pub fn new_random(rows : usize,cols : usize)-> Matrix { 
        let mut data = Vec::<f64>::with_capacity(rows * cols);

        for _ in 0..data.len(){
            data.push(rand::rng().random_range(0.0..1.0)); 
        } 
        return Matrix{ rows, cols , data};
    }
}
//Simple Operations
impl Matrix {
    pub fn element_multiply(&self , matrix : &Matrix) -> Matrix {
        if self.rows != matrix.rows || self.cols != matrix.cols{
            panic!("Uso incorreto da função multiply")
        }
        let mut result =vec![0.0;self.rows*self.cols];
        for index in 0..result.len(){   
            result[index] = self.data[index] * matrix.data[index]
        }
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: result,
        }
    }
     pub fn element_subtraction(&self , matrix : &Matrix) -> Matrix {
        if self.rows != matrix.rows || self.cols != matrix.cols{
            panic!("Uso incorreto da função multiply")
        }
        let mut result =vec![0.0;self.rows*self.cols];
        for index in 0..result.len(){   
            result[index] = self.data[index] - matrix.data[index]
        }
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: result,
        }
    }
     pub fn element_add(&self , matrix : &Matrix) -> Matrix {
        if self.rows != matrix.rows || self.cols != matrix.cols{
            panic!("Uso incorreto da função multiply")
        }
        let mut result =vec![0.0;self.rows*self.cols];
        for index in 0..result.len(){   
            result[index] = self.data[index] + matrix.data[index]
        }
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: result,
        }
    }


}
// Matrix Operations 
impl Matrix{

    pub fn dot_multiply(&self,pair : Matrix)-> Matrix {
        if self.cols != pair.rows {
            panic!("Mal uso da função dot multiply")
        }
        let mut result = vec![0.0; self.rows * pair.cols];
        
           for i in 0..self.rows {
            for j in 0..pair.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * pair.data[k * pair.cols + j];
                }
                result[i * pair.cols + j] = sum;
            }
        }
           Matrix {
            rows: self.rows,
            cols: pair.cols,
            data: result,
        }
    }
}
//Matrix modification 
impl Matrix {
    pub fn invert(&self)->Matrix{
        let mut result = vec![0.0 ; self.cols * self.rows];
        for i in 0..self.rows{
            for j in 0..self.cols{
                result[j*self.rows + i]  = self.data[i* self.cols + j]; 
            }
        }

        Matrix { rows: self.cols, cols: self.rows, data: result }
    }
    pub fn map(&mut self , function : fn(&f64)->f64) -> Matrix{
        let mut result = Matrix{
            rows: self.rows,
            cols: self.cols,
            data: Vec::<f64>::with_capacity(self.data.len()),
        };

        result.data.extend(self.data.iter().map(|value| function(&value)));
        result

    }
}

//Traits Build

impl From<Vec<f64>> for Matrix {
    fn from(vector: Vec<f64>) -> Self {
        Matrix { 
            rows: vector.len(),
            cols: 1,
            data: vector
        }
    }
}

impl PartialEq for Matrix{
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}

impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rows in 0..self.rows {
            for cols in 0..self.cols{
                write!(f , "{}" , self.data[rows * self.cols + cols])?;
                if cols < self.cols -1 {
                    write!(f , "\t")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}