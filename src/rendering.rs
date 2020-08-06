use three;
use three::Object;

// TODO: MeshRegistry, load all entities with instanced rendering for extra speed

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
