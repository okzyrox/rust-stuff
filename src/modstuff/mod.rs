pub mod testmod {
    pub fn pub_test() {
        println!("Testing module stuff");
    }

    fn priv_test() {
        println!("Testing private stuff");
    }

    pub fn all_test() {
        pub_test();
        priv_test();
    }
}

pub mod mathstuff {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn mul(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn div(a: i32, b: i32) -> i32 {
        a / b
    }

    pub fn rem(a: i32, b: i32) -> i32 {
        a % b
    }

    pub fn pow(a: i32, b: u32) -> i32 {
        a.pow(b)
    }
}

pub mod entity {
    pub struct Entity {
        pub id: i32,
        pub name: String,
        pub x: i32,
        pub y: i32,
    }

    pub struct EntityManager {
        pub entities: Vec<Entity>,
    }

    impl EntityManager {
        pub fn new() -> EntityManager {
            EntityManager { entities: Vec::new() }
        }

        pub fn list_entities(&self) {
            println!("List of entities: ");
            for entity in &self.entities {
                entity.print();
            }
        }

        pub fn new_entity(&mut self, id: i32, name: String, x: i32, y: i32) -> Option<&Entity> {
            for entity in &self.entities {
                if entity.id == id {
                    println!("Entity with id {} already exists", id);
                    return None;
                }
            }
            let entity = Entity::new(id, name, x, y);
            self.entities.push(entity);
            return self.entities.last();
        }

        pub fn get_entity(&self, id: i32) -> Option<&Entity> {
            for entity in &self.entities {
                if entity.id == id {
                    return Some(&entity);
                }
            }
            return None;
        }
    }

    impl Entity {
        pub fn new(id: i32, name: String, x: i32, y: i32) -> Entity {
            Entity { id, name, x, y }
        }

        pub fn print(&self) {
            println!("<Entity> id: {}, name: {}, x: {}, y: {}", self.id, self.name, self.x, self.y);
        }
    }
}