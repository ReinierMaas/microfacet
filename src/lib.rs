extern crate cgmath;
extern crate rand;

use cgmath::InnerSpace;
use cgmath::Vector3;

use self::rand::Closed01;

#[inline]
pub fn microfacet_sample(normal: &Vector3<f32>
                       , view: &Vector3<f32>
                       , alpha: f32) -> Vector3<f32> {
    let Closed01(r0) = rand::random::<Closed01<f32>>();
    let Closed01(r1) = rand::random::<Closed01<f32>>();

    let t = r0.powf(2.0 / (alpha + 1.0));
    let phi = 2. * std::f32::consts::PI * r1;
    let sqrt_1_min_t = (1.0 - t).sqrt();

    let x = phi.cos() * sqrt_1_min_t;
    let y = phi.sin() * sqrt_1_min_t;
    let z = t.sqrt();

    let halfway = from_tangent_to_local(normal, &Vector3::new(x, y, z));
    reflect(view, &halfway)
}
#[inline]
fn from_tangent_to_local(normal: &Vector3<f32>, tangent: &Vector3<f32>) -> Vector3<f32> {
    let t = (normal.cross(if normal.x.abs() > 0.99 { Vector3::new(0.0,1.0,0.0) } else { Vector3::new(1.0,0.0,0.0) })).normalize();
    let b = normal.cross(t);
    tangent.x * t + tangent.y * b + tangent.z * normal
}
#[inline]
fn reflect(view: &Vector3<f32>, halfway: &Vector3<f32>) -> Vector3<f32> {
    2.0 * view.dot(*halfway) * halfway - view
}

#[inline]
pub fn microfacet(normal: &Vector3<f32>
                , view_direction: &Vector3<f32>
                , light_direction: &Vector3<f32>
                , specular_color: &Vector3<f32>
                , alpha: f32) -> Vector3<f32> {
    let halfway = (view_direction + light_direction).normalize();

    let ndotv = normal.dot(*view_direction);
    let ndotl = normal.dot(*light_direction);
    let ndoth = normal.dot(halfway);
    let hdotv = halfway.dot(*view_direction);
    let hdotl = halfway.dot(*light_direction);

    let normal_distribution = normal_distribution(ndoth, alpha);
    let geometric_term = geometric_term(ndotv, ndotl, ndoth, hdotv);
    let fresnel_term = fresnel_term(hdotl, specular_color);
    let normalization = 1.0 / (4.0 * ndotl * ndotv);
    normal_distribution * geometric_term * normalization * fresnel_term
}
#[inline]
fn normal_distribution(ndoth: f32
                         , alpha: f32) -> f32 {
    const FRAC_1_2PI: f32 = 1.0 / (2.0 * std::f32::consts::PI);
    // Blinn-Phong distribution
    (alpha + 2.0) * FRAC_1_2PI * ndoth.powf(alpha)
}
#[inline]
fn geometric_term(ndotv: f32
                    , ndotl: f32
                    , ndoth: f32
                    , hdotv: f32) -> f32 {
    // Physically Based Rendring, page 455
    let frac_2ndoth_hdotv = (2.0 * ndoth) / hdotv;
    (1.0_f32).min((frac_2ndoth_hdotv * ndotv).min(frac_2ndoth_hdotv * ndotl))
}
#[inline]
fn fresnel_term(hdotl: f32
                  , specular_color: &Vector3<f32>) -> Vector3<f32> {
    const ONES: Vector3<f32> = Vector3::<f32>{x: 1.0, y: 1.0, z: 1.0};
    // Schlick's approximation
    let pow_5_1_min_hdotl = (1.0 - hdotl).powi(5);
    specular_color + pow_5_1_min_hdotl * (ONES - specular_color)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
