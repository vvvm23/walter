use three;

type Texture = three::Texture<[f32; 4]>;

pub struct SpriteComponent {
    texture: Texture,
    scale: f32,
}

impl SpriteComponent {
    pub fn new(win: &mut three::Window, path: &str) -> Self {
        Self {
            texture: win.factory.load_texture(path),
            scale: 1.0
        }
    }

    pub fn change_texture(&mut self, win: &mut three::Window, path: &str) {
        self.texture = win.factory.load_texture(path);
    }

    pub fn set_scale(&mut self, scale: f32) {
        assert!(scale > 0.0);
        self.scale = scale;
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
