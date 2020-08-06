use three;
use three::Object;

type Texture = three::Texture<[f32; 4]>;

pub struct SpriteComponent {
    mesh: three::Sprite,
}

impl SpriteComponent {
    pub fn new(win: &mut three::Window, path: &str) -> Self {
        let texture = win.factory.load_texture(path);
        Self {
            mesh: win.factory.sprite(three::material::Sprite { map: texture }),
        }
    }

    pub fn change_texture(&mut self, win: &mut three::Window, path: &str) {
        let texture = win.factory.load_texture(path);
        self.mesh = win.factory.sprite(three::material::Sprite { map: texture });
    }

    pub fn set_scale(&mut self, scale: f32) {
        assert!(scale > 0.0);
        self.mesh.set_scale(scale);
    }

    pub fn scene_add(&self, win: &mut three::Window) {
        win.scene.add(&self.mesh);
    }

    pub fn scene_remove(&self, win: &mut three::Window) {
        win.scene.remove(&self.mesh);
    }

    pub fn update_pos(&mut self, pos: [f32; 3]) {
        self.mesh.set_position(pos);
    }
}


//use std::collections::HashMap;
//use std::rc::Rc;

//type Texture = three::Texture<[f32; 4]>;

//const MAX_TEXTURES: usize = 256;
//pub struct TextureRegistry {
    //textures: Vec<Option<Rc<Texture>>>,
    //file_map: HashMap<String, usize>,
//}

//impl TextureRegistry {
    //pub fn new() -> Self {
        //Self {
            //textures: vec![None; MAX_TEXTURES],
            //file_map: HashMap::new(),
        //}
    //}

    //fn load_texture(&mut self, win: three::Window, path: String) -> Rc<Texture> {
        
    //}

    //pub fn get_texture(&mut self, win: three::Window, path: String) -> Rc<Texture> {
        //match self.file_map.get(&path) {
            //Some(i) => Rc::clone(self.textures[*i]),
            //None => (),
        //}
    //}
//}
