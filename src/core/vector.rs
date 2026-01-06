use std::fmt;
use std::ops::{Add, Sub};
use std::string;

pub struct Vector<T> {
    data: Vec<T>,
    shape: Vec<usize>
}


impl<T: Copy> Vector<T> {
    pub fn new(s: Vec<T>) -> Self {
        let shape: Vec<usize> = vec!(s.len());
        println!("Initialize a vector with shape {}", shape[0]);
        return Self { data: s, shape: shape };
    }
}

impl<T: Clone + Copy> Clone for Vector<T> {
    fn clone(&self) -> Self {
        let mut data: Vec<T> = Vec::new();
        let shape = vec!((*self).shape[0]);
        for i in 0..shape[0] {
            data.push((*self).data[i]);
        }
        return Self {data:data, shape:shape}
    }
}

impl<'a,'b, T: Add<Output = T> + Copy> std::ops::Add<&'b Vector<T>> for  &'a Vector<T> {
    type Output = Vector<T>;
    fn add(self, other: &'b Vector<T>) -> Vector<T> {
        if self.shape[0] != other.shape[0] {
            panic!("The shape {} is incompatible with the second vector, which shape is {}", 
                    self.shape[0], other.shape[0]
                );            
        }
        let mut res: Vec<T> = Vec::with_capacity(self.shape[0]);
        for i in 0..self.shape[0] {
            res.push(self.data[i] + other.data[i]);
        }
        return Vector {data: res, shape: vec!(self.shape[0])};
    }
}

impl<'a, 'b, T: Sub<Output = T> + Copy> std::ops::Sub<&'b Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;
    fn sub(self, other: &'b Vector<T>) -> Vector<T> {
        if self.shape[0] != other.shape[0] {
            panic!("The shape {} is incompatible with the second vector, which shape is {}", 
                    self.shape[0], other.shape[0]
                );            
        }
        let mut res: Vec<T> = Vec::with_capacity(self.shape[0]);
        for i in 0..self.shape[0] {
            res.push(self.data[i] - other.data[i]);
        }
        return Vector {data: res, shape: vec!(self.shape[0])};

    }
}

impl<T: string::ToString + fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print!("[");
        for i in 0..self.shape[0] {
            if i >= 5usize {break;}
            if i == 0 {print!("{}", self.data[i].to_string());}
            else {print!(" {}", self.data[i].to_string());}
        }
        if self.shape[0] <= 5usize {
            println!("]), shape = {}", self.shape[0]);
        }
        else {
            println!(" ... {}]), shape = {}", self.data[self.shape[0] - 1], self.shape[0]);
        }
        Ok(())
    }    
} 

