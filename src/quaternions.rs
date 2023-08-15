use std::ops;

#[derive(Clone, Debug)]
pub struct Quaternion {
    a: f64, // scalar
    b: f64, // vector i
    c: f64, // vector j
    d: f64, // vector k
}

impl Quaternion {
    pub fn new(    
        a: f64, // scalar
        b: f64, // vector i
        c: f64, // vector j
        d: f64, // vector k
    ) -> Self {
        Quaternion{a,b,c,d}
    }

    pub fn sum(q: Quaternion, p: Quaternion) -> Quaternion {
        Quaternion::new(
            q.a + p.a,
            q.b + p.b, 
            q.c + p.c,
             q.d + p.d
        )
    }

    pub fn product(q: Quaternion, p: Quaternion) -> Quaternion {
        Quaternion::new(
            (q.a * p.a) - (q.b * p.b) - (q.c * p.c) - (q.d * p.d),
            (q.a * p.b) + (q.b * p.a) + (q.c * p.d) - (q.d * p.c), 
            (q.a * p.c) - (q.b * p.d) + (q.c * p.a) + (q.d * p.b), 
            (q.a * p.d) + (q.b * p.c) - (q.c * p.b) - (q.d * p.a)
        )
    }

    pub fn scale(q: Quaternion, alpha: f64) -> Quaternion {
        Quaternion::new(q.a*alpha, q.b*alpha, q.c*alpha, q.d*alpha)
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion::new(self.a, -self.b, -self.c, -self.d)
    }

    pub fn normalize(&self) -> f64 {
        ((self.a*self.a) + (self.b*self.b) + (self.c*self.c) + (self.d*self.d)).sqrt()
    }

    pub fn unitize(&self) -> Quaternion {
        let l = self.normalize();
        self.clone() * (1.0/l)
    }

    pub fn inverse(&self) -> Quaternion {
        let qi = self.conjugate();
        let l = self.normalize();
        let l2 = l*l;
        qi*(1.0/l2)
    }
    pub fn rotatation(&self, theta: f64) -> Quaternion {
        let c = (theta/2.0).cos();
        let s =  (theta/2.0).sin();
        let u = self.unitize();
        (u * s) + c
    }

    pub fn rotate_point<Q>(&self, a: Q, theta: f64) -> (f64,f64,f64) 
    where Q: Into<Quaternion> {
        let p = a.into();
        let q = self.rotatation(theta);
        let qi = q.inverse();
        let lp = q*p*qi;
        lp.into()
    }


}

impl ops::Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Self::Output {
        Quaternion::sum(self, rhs)
    }
}

impl ops::Add<f64> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: f64) -> Self::Output {
        let mut q = self.clone();
        q.a += rhs;
        q
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion::product(self,rhs)        
    }
}

impl ops::Mul<f64> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: f64) -> Self::Output {
        Quaternion::scale(self, rhs)        
    }
}

impl From<(f64,f64,f64)> for Quaternion {
    fn from(value: (f64,f64,f64)) -> Self {
        Quaternion::new(0.0, value.0, value.1, value.2)
    }
}

impl Into<(f64,f64,f64)> for Quaternion {
    fn into(self) -> (f64,f64,f64) {
        (self.b, self.c, self.d)
    }
}