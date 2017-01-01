use shapes::Sphere;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    pub objects: Vec<Sphere>
}
