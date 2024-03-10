use crate::mesh::{mesh_notex, Mesh};
use crate::camera::{ Camera, camera };
use crate::shader::{shader, Shader};

pub struct Level {
    objects: Vec<Mesh>,
    light: Mesh,
    camera: Camera,
    projection: nalgebra_glm::Mat4,
    shader: Shader,
    light_shader: Shader
}

impl Level {
    pub fn draw(&mut self) {
        self.shader.set_uniform_mat4("view", &self.camera.view());
        self.light_shader.set_uniform_mat4("view", &self.camera.view());

        self.shader.set_uniform_vec3("viewPos", &self.camera.get_position());

        self.set_light();

        self.shader.set_uniform_float("material.shininess", 4.0);
        self.shader.set_uniform_int("material.diffuse", 0);
        self.shader.set_uniform_int("material.specular", 0);
        for obj in &self.objects {
            obj.draw(&self.shader);
        }
    }

    pub fn add_obj(&mut self, obj: Mesh) {
        self.objects.push(obj);
    }

    pub fn camera(&self) -> &Camera { &self.camera }
    pub fn camera_mut(&mut self) -> &mut Camera { &mut self.camera }
    pub fn shader(&self) -> &Shader { &self.shader }

    pub fn reload(&mut self, objs: Vec<Mesh>) { self.objects = objs; }

    fn set_light(&mut self) {
        self.shader.set_uniform_vec3("light.position", self.light.get_pos());
        self.shader.set_uniform_vec3("light.ambient", &nalgebra_glm::vec3(0.1, 0.1, 0.1));
        self.shader.set_uniform_vec3("light.diffuse", &nalgebra_glm::vec3(0.5, 0.5, 0.5));
        self.shader.set_uniform_vec3("light.specular", &nalgebra_glm::vec3(1.0, 1.0, 1.0));
        self.light.draw(&self.light_shader);
    }
    pub fn move_lpos(&mut self, pos: &nalgebra_glm::Vec3) { self.light.move_pos(pos) }
}
pub fn level(objs: Vec<Mesh>, pos: nalgebra_glm::Vec3, scr_width: f32, scr_height: f32) -> Level {
    let projection = nalgebra_glm::perspective(110.0 / (180.0/3.1415926), scr_width/scr_height, 0.1, 150.0);
    let oshader = shader("./shaders/shader.vert", "./shaders/shader.frag");
    let lshader = shader("./shaders/light_cube.vert", "./shaders/light_cube.frag");
    oshader.set_uniform_mat4("projection", &projection);
    lshader.set_uniform_mat4("projection", &projection);

    oshader.set_uniform_float("light.constant", 1.0);
    oshader.set_uniform_float("light.linear", 0.022);
    oshader.set_uniform_float("light.quadratic", 0.0019);
    oshader.set_uniform_bool("fullBright", false);

    Level { objects: objs, light: mesh_notex(nalgebra_glm::vec3(20.0, 9.0, 5.0), 0.5, 0.5, 0.5, nalgebra_glm::vec4(1.0, 1.0, 1.0, 1.0)), camera: camera(pos), projection: projection, shader: oshader, light_shader: lshader }
}