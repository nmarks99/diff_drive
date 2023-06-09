/// rigid2D: 2D rigid body motion library

use num_traits::Float;
use std::fmt::Display;
use std::ops;

/// a 2-dimensional vector
#[derive(Debug, Clone, Copy)]
pub struct Vector2D<T: Float> {
    /// x coordinate
    pub x: T,

    /// y coordinate
    pub y: T
}

/// Implements the operation: Vector2D + Vector2D
impl<T: Float> ops::Add<Vector2D<T>> for Vector2D<T> {
    type Output = Vector2D<T>;
    fn add(self, _rhs: Vector2D<T>) -> Vector2D<T> {
        Vector2D::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

/// Implements the Displayy trait for Vector2D
impl<T:Float + Display> Display for Vector2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T: Float> Vector2D<T> {
    
    /// constructs a Vector2D from floats (x,y)
    pub fn new(x: T, y: T) -> Self {
        Vector2D {x, y}
    }

    /// Creates a Vector2D from polar coordinates
    pub fn from_polar(r: T, phi: T) -> Self {
        Vector2D { x: r*phi.cos() , y: r*phi.sin() }
    }
    
    /// computes the L2 norm of the 2D vector
    pub fn magnitude(&self) -> T {
        let x_sqr = self.x.powf(T::from(2.0).unwrap());
        let y_sqr = self.y.powf(T::from(2.0).unwrap());
        (x_sqr + y_sqr).sqrt()
    }
    
    /// computes the dot product of the vector with another vector
    pub fn dot(&self, rhs: &Vector2D<T>) -> T {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
    
    /// computes the angle between two vectors
    pub fn angle(&self, rhs: &Vector2D<T>) -> T {
        let dot_prod = self.dot(rhs);
        let prod_of_msg = self.magnitude() * rhs.magnitude();
        return (dot_prod / prod_of_msg).acos();
    }
    
    /// normalizes the vector
    pub fn normalize(&self) -> Self {
        let v_norm: Self;
        if self.x.is_zero() || self.y.is_zero() {
            v_norm = Vector2D::new(T::zero(), T::zero());
        }
        else {
            let mag = self.magnitude();
            v_norm = Vector2D::new(self.x/mag, self.y/mag);
        }
        return v_norm;
    }
}


/// A 2-dimensional pose
#[derive(Debug)]
pub struct Pose2D<T: Float> {
    /// Rotation in radians
    pub theta: T,

    /// x position
    pub x: T,

    /// y position
    pub y: T
}

impl<T:Float> Pose2D<T> {

    /// constructs a new Pose2D object from (theta,x,y) which are floats
    pub fn new(theta: T, x: T, y: T)  -> Self {
        Pose2D {theta, x, y}
    }
}

/// Implments the Display trait
impl<T:Float + Display> Display for Pose2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "deg:{} x:{} y:{}", self.theta, self.x, self.y)
    }
}


/// A 2-dimensional Twist
#[derive(Debug)]
pub struct Twist2D<T: Float> {
    /// angular velocity
    pub thetadot: T,

    /// x velocity
    pub xdot: T,

    /// y 
    pub ydot: T
}

impl<T: Float> Twist2D<T> {

    /// constructs as new Twist2D object from theta, x, and y velocities
    pub fn new(thetadot: T, xdot: T, ydot: T) -> Self {
        Twist2D {thetadot, xdot, ydot}
    }
}

/// Implements the Display trait
impl<T:Float + Display> Display for Twist2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "[{}, {}, {}]", self.thetadot, self.xdot, self.ydot)
    }
}

/// A 2-dimensional rigid body transformation
pub struct Transform2D<T: Float> {

    /// rotational component of the transform in radians
    angle: T,

    /// translational component of the transform
    p_vec: Vector2D<T>
}

impl<T: Float> Transform2D<T> {
    
    /// contructs a new Transform2D from a translation and rotation
    pub fn new(trans: Vector2D<T>, angle: T) -> Self {
        Transform2D {p_vec: trans, angle} 
    }
    
    /// returns the rotatational component of the transform
    pub fn rotation(&self) -> T {
        self.angle
    }

    /// returns the translational component of the transform
    pub fn translation(&self) -> Vector2D<T> {
        self.p_vec
    }
}



