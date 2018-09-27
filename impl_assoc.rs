use std::ops::*;

pub trait Geometry {
    type D: Dimensionality;

    fn min(&self) -> <Self::D as Dimensionality>::Point {
        self.max() - self.dims()
    }

    fn max(&self) -> <Self::D as Dimensionality>::Point;

    fn dims(&self) -> <Self::D as Dimensionality>::Vector;
}

pub trait Dimensionality {
    type Point: EuclideanSpace<Diff=Self::Vector>;
    type Vector;
}

pub trait EuclideanSpace:
    Copy + Clone +
    Sub<<Self as EuclideanSpace>::Diff, Output = Self>
{
    type Diff;
}

fn main() {}
