use core::f32;

use crate::engine::Ray;
use crate::utils::{Matrix3, Vec3};

use super::{Material, Shape};

#[derive(Clone, PartialEq)]
pub struct Cilinder {
    pub r: f32, pub h: f32,
    pub cb: Vec3, pub ct: Vec3,
    pub dc: Vec3,
    pub material: Material
}

impl Cilinder {
    #[inline]
    #[must_use]
    pub fn new(r: f32, h: f32, cb: Vec3, mut dc: Vec3, material: Material) -> Shape {
        dc = dc.normalize();
        Shape::Cilinder( Cilinder {r, h, cb, dc, ct:cb + h*dc, material} )
    }

    #[must_use]
    pub fn intersects(&self, r: &Ray) -> (f32, Vec3) {
        let mut t = f32::INFINITY;
        let mut n = Vec3::NULL;

        // Check superfície cilíndrica
        let q = self.dc.projection_matrix();
        let m = Matrix3::I - q;
        let s = r.origin - self.cb;
        
        let mdr = m*r.dr;
        let ms = m*s;

        let a = mdr.dot(mdr);
        let b = 2.0 * mdr.dot(ms);
        let c = ms.dot(ms) - self.r*self.r;
        let delta = b*b - 4.0*a*c;

        if delta > 0.0 {
            let t1 = (-b + delta.sqrt()) / (2.0*a);
            let t2 = (-b - delta.sqrt()) / (2.0*a);

            if t1 > 0.0 && t1 < t {
                let cbp = r.at(t1) - self.cb;
                let cbe = q*cbp;

                if cbe.dot(self.dc) > 0.0
                && cbe.length() < self.h {
                    t = t1;
                    n = (m*cbp).normalize();
                }
            }

            if t2 > 0.0 && t2 < t {
                let cbp = r.at(t2) - self.cb;
                let cbe = q*cbp;

                if cbe.dot(self.dc) > 0.0
                && cbe.length() < self.h {
                    t = t2;
                    n = (m*cbp).normalize();
                }
            }
        }
        
        // Check plano do topo do cilindro
        let bottom = r.dr.dot(self.dc);
        if bottom != 0.0 {
            let t_tampa = -(r.origin - self.ct).dot(self.dc) / bottom;
            
            if t_tampa >= 0.0
            && t_tampa < t
            && (r.at(t_tampa) - self.ct).length() <= self.r {
                t = t_tampa;
                n = self.dc * -self.dc.dot(r.dr).signum();
            }
        }

        // Check plano da base do cilindro
        let bottom = r.dr.dot(-self.dc);
        if bottom != 0.0 {
            let t_base = -(r.origin - self.cb).dot(-self.dc) / bottom;
            
            if t_base >= 0.0
            && t_base < t
            && (r.at(t_base) - self.cb).length() <= self.r {
                t = t_base;
                n = self.dc * -self.dc.dot(r.dr).signum();
            }
        }
        
        if t == f32::INFINITY { t = -t }
        (t, n)
    }
}