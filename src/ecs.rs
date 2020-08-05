/// Walter Engine ECS System 
/// 
/// First we define a Component trait. All components must implement this trait.
/// This lets us store references to components in an Entity together in one HashMap
/// We can also store the actual components in one vector in the main state.
/// This will let us define any number of Components without having to alter
/// the main state.
/// 
/// However, we still wish to have some way to iterate over all components of one 
/// type. We can instead keep a HashMap<ComponentType, Vec<Rc<Component>>> in the
/// main state. We can achieve the same effect as one vector if we find a way to
/// iterate over the enum ComponentType.
///
/// Entity will contain data that applies to all entities (such as position, moving 
/// out of the previous PositionComponent). It will also contain a lookup to 
/// components associated with that Entity. With this, we can lookup the components
/// of an entity from the Entity struct itself or from one of its components.
///

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use log::{error, debug};

use strum_macros::EnumIter;
use strum::IntoEnumIterator;

pub trait Component {
    fn get_owner(&self) -> Rc<RefCell<Entity>>;
    fn update(&mut self) {}
}
impl std::fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum ComponentType {
    Null,
    Position,
    Fighter,
    Inventory,
    Playable,
}

struct NullComponent {
    owner: Rc<RefCell<Entity>>,
}

impl NullComponent {
    fn new(owner: Rc<RefCell<Entity>>) -> Self {
        NullComponent {owner: owner}
    }
}

impl Component for NullComponent {
    fn get_owner(&self) -> Rc<RefCell<Entity>> {
        Rc::clone(&self.owner)
    }

    fn update(&mut self) {
        debug!("This component does nothing.")
    }
}

#[derive(Debug)]
pub struct Entity {
    pub id: u16,
    components: HashMap<ComponentType, Rc<RefCell<dyn Component>>>,
}

impl Entity {
    fn new(id: u16) -> Self {
        Entity {
            id: id,
            components: HashMap::new()
        }
    }
    
    fn get_component(&self, c: ComponentType) -> Option<Rc<RefCell<dyn Component>>> {
        match self.components.get(&c) {
            Some(a) => Some(Rc::clone(a)),
            None => None,
        }
    }

    fn add_component(&mut self, ct: ComponentType, c: &Rc<RefCell<dyn Component>>) {
        self.components.insert(ct, Rc::clone(c));
    }

}

const MAX_ENTITIES: usize = 256;
pub struct State {
    //pub entities: Vec<Rc<RefCell<Entity>>>,
    pub entities: Vec<Option<Rc<RefCell<Entity>>>>,
    pub components: HashMap<ComponentType, Vec<Rc<RefCell<dyn Component>>>>,
    next_free: u16,
}

impl State {
    pub fn new() -> Self {
        let mut x = Self {
            entities: vec![None; MAX_ENTITIES],
            components: HashMap::new(),
            next_free: 0
        };

        for ct in ComponentType::iter() {
            x.components.insert(ct, Vec::new());
        }
        x
    }

    pub fn new_entity(&mut self) -> Rc<RefCell<Entity>> {
        let mut n_id = self.next_free;
        if let Some(_) = self.entities[n_id as usize] {
            for c_id in self.next_free..(self.next_free+MAX_ENTITIES as u16) {
                n_id = c_id % MAX_ENTITIES as u16;
                if let None = self.entities[n_id as usize] {
                    break;
                }
            }

            error!("Maximum Entity count reached!");
            panic!("");
        }

        let e = Entity::new(n_id);
        let e = Rc::new(RefCell::new(e));
        self.entities[n_id as usize] = Some(Rc::clone(&e));

        self.next_free = (n_id + 1) % MAX_ENTITIES as u16;
        e
    }

    // TODO: Add entity / component

    // TODO: Delete entity / component
    // TODO: Check Rc Count to ensure they are truly deleted

}
