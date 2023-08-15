use std::ops;

#[derive(Clone)]
struct Quaternion {
    a: f64, // scalar
    b: f64, // vector i
    c: f64, // vector j
    d: f64, // vector k
}

impl Quaternion {
    fn new(    
        a: f64, // scalar
        b: f64, // vector i
        c: f64, // vector j
        d: f64, // vector k
    ) -> Self {
        Quaternion{a,b,c,d}
    }

    fn sum(q: Quaternion, p: Quaternion) -> Quaternion {
        Quaternion::new(
            q.a + p.a,
            q.b + p.b, 
            q.c + p.c,
             q.d + p.d
        )
    }

    fn product(q: Quaternion, p: Quaternion) -> Quaternion {
        Quaternion::new(
            (q.a * p.a) - (q.b * p.b) - (q.c * p.c) - (q.d * p.d),
            (q.a * p.b) + (q.b * p.a) + (q.c * p.d) - (q.d * p.c), 
            (q.a * p.c) - (q.b * p.d) + (q.c * p.a) + (q.d * p.b), 
            (q.a * p.d) + (q.b * p.c) - (q.c * p.b) - (q.d * p.a)
        )
    }

    fn conjugate(&self) -> Quaternion {
        Quaternion::new(self.a, -self.b, -self.c, -self.d)
    }

    fn normalize(&self) -> f64 {
        ((self.a*self.a) + (self.b*self.b) + (self.c*self.c) + (self.d*self.d)).sqrt()
    }

    fn unitize(&self) -> Quaternion {
        let l = self.normalize();
        Quaternion::new(self.a/l, self.b/l, self.c/l, self.d/l)
    }

    fn inverse(&self) -> Quaternion {
        let qi = self.conjugate();
        let l = self.normalize();
        let l2 = l*l;
        Quaternion::new(qi.a/l2, qi.b/l2, qi.c/l2, qi.d/l2)
    }

    fn rotate_point<Q>(&self, a: Q) -> (f64,f64,f64) 
    where Q: Into<Quaternion> {
        let u = self.unitize();
        let v = a.into();
        let ui = u.inverse();
        let lv = u*v*ui;
        lv.into()
    }

}

impl ops::Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Self::Output {
        Quaternion::sum(self, rhs)
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion::product(self,rhs)        
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