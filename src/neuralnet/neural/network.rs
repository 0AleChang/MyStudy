
use crate::neuralnet::matrix::matrix::{Matrix};

use super::activation::{ Activation};
pub struct NetWork{
    layer : Vec<usize>,
    weight: Vec<Matrix>,
    biases: Vec<Matrix>,
    data  : Vec<Matrix>,
    activation : Activation,
    learning_rate: f64,
}

impl NetWork {

    pub fn new(layer : Vec<usize>, activation: Activation, learning_rate : f64)-> NetWork{
        let mut weight : Vec<Matrix> = vec![];
        let mut weight : Vec<Matrix> = vec![];


        
        NetWork { layer, weight: (), biases: (), data: (), activation, learning_rate }
    }

}