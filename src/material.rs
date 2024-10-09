use std::rc::Rc;

use na::Point3;

use crate::random_double;
use crate::vec3::{random_unit_vector, reflect, refract, Vector3Ext};
use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub enum DefaultPalette {
    DefaultMaterial,
    Lambertian,
    Metal,
    Dielectric,
    DiffuseLight,
}

pub fn create_material(
    palette: DefaultPalette,
    color: Color,
    fuzz: f64,
    refraction_index: f64,
    emit: Color,
) -> Rc<dyn Material> {
    match palette {
        DefaultPalette::DefaultMaterial => Rc::new(DefaultMaterial::new()),
        DefaultPalette::Lambertian => Rc::new(Lambertian::new(color)),
        DefaultPalette::Metal => Rc::new(Metal::new(color, fuzz)),
        DefaultPalette::Dielectric => Rc::new(Dielectric::new(refraction_index)),
        DefaultPalette::DiffuseLight => Rc::new(DiffuseLight::new(emit)),
    }
}

pub enum StandardColor {
    Red,
    Green,
    Blue,
    White,
    Black,
    Grey,
    Yellow,
    Purple,
}

pub fn create_standard_material(palette: StandardColor) -> Rc<dyn Material> {
    match palette {
        StandardColor::Red => Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05))),
        StandardColor::Green => Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15))),
        StandardColor::Blue => Rc::new(Lambertian::new(Color::new(0.1, 0.1, 0.7))),
        StandardColor::White => Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73))),
        StandardColor::Black => Rc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
        StandardColor::Grey => Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
        StandardColor::Yellow => Rc::new(Lambertian::new(Color::new(0.9, 0.9, 0.0))),
        StandardColor::Purple => Rc::new(Lambertian::new(Color::new(0.5, 0.0, 0.5))),
    }
}

pub enum StandardMetal {
    Gold,
    Silver,
    Copper,
    Aluminium,
}

pub fn create_standard_metal(palette: StandardMetal, fuzz: f64) -> Rc<dyn Material> {
    match palette {
        StandardMetal::Gold => Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), fuzz)),
        StandardMetal::Silver => Rc::new(Metal::new(Color::new(0.9, 0.9, 0.9), fuzz)),
        StandardMetal::Copper => Rc::new(Metal::new(Color::new(0.72, 0.45, 0.2), fuzz)),
        StandardMetal::Aluminium => Rc::new(Metal::new(Color::new(0.8, 0.85, 0.88), fuzz)),
    }
}

pub enum StandardGlasses {
    Water,
    Glass,
    Diamond,
}

pub fn create_standard_glass(palette: StandardGlasses) -> Rc<dyn Material> {
    match palette {
        StandardGlasses::Water => Rc::new(Dielectric::new(1.333)),
        StandardGlasses::Glass => Rc::new(Dielectric::new(1.5)),
        StandardGlasses::Diamond => Rc::new(Dielectric::new(2.42)),
    }
}

pub enum LightColor {
    White,
    Yellow,
    Red,
    Green,
    Blue,
    Purple,
}

pub fn create_light_material(palette: LightColor) -> Rc<dyn Material> {
    match palette {
        LightColor::White => Rc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0))),
        LightColor::Yellow => Rc::new(DiffuseLight::new(Color::new(1.0, 1.0, 0.0))),
        LightColor::Red => Rc::new(DiffuseLight::new(Color::new(1.0, 0.0, 0.0))),
        LightColor::Green => Rc::new(DiffuseLight::new(Color::new(0.0, 1.0, 0.0))),
        LightColor::Blue => Rc::new(DiffuseLight::new(Color::new(0.0, 0.0, 1.0))),
        LightColor::Purple => Rc::new(DiffuseLight::new(Color::new(1.0, 0.0, 1.0))),
    }
}

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3<f64>) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct DefaultMaterial;

impl DefaultMaterial {
    pub fn new() -> DefaultMaterial {
        DefaultMaterial
    }
}

impl Material for DefaultMaterial {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(*r_in.direction(), rec.normal);
        reflected = reflected.normalize() + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    // Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().normalize();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_double() {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, ri)
            };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}

pub struct DiffuseLight {
    emit: Color,
}

impl DiffuseLight {
    pub fn new(emit: Color) -> DiffuseLight {
        DiffuseLight { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3<f64>) -> Color {
        self.emit
    }
}
