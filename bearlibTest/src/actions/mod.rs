use World;
use Action;
use EntityId;
use Direction;
use Control;
use DoorState;
use pathing;
use TileType;
use ActionType;
use ProjectileType;
use Materials;
use Game_State;
use bear_lib_terminal::geometry::Point;
use bear_lib_terminal::terminal::{self, config, Event, KeyCode};


pub fn start_pointer(character_id: EntityId, world: &World, action: &mut Action) {
    let start_position = world.get_position(character_id);
    action.insert_pointer(character_id, start_position.unwrap());
}

pub fn start_aim(character_id: EntityId, world: &World, action: &mut Action) {
    let start_position = world.get_position(character_id);
    action.insert_aim(character_id, start_position.unwrap());
    //print!("startaim");
}




pub fn move_pointer(character_id: EntityId, direction: Direction, world: &World, action: &mut Action) {
    let current_position = world.get_pointer(character_id)
        .expect("Attempt to pointer entity with no position");
    let (x, y) = current_position;
    let (dx, dy) = direction.unit_vector();
    let new_position = (x+dx, y+dy);

    action.insert_pointer(character_id, new_position);
}

pub fn move_aim(character_id: EntityId, direction: Direction, world: &World, action: &mut Action) {
    let current_position = world.get_pointer(character_id)
        .expect("Attempt to move aim with no position");
    let (x, y) = current_position;
    let (dx, dy) = direction.unit_vector();
    let new_position = (x+dx, y+dy);

    action.insert_aim(character_id, new_position);
}


pub fn move_character(character_id: EntityId, direction: Direction, world: &World, action: &mut Action) {
    let current_position = world.get_position(character_id)
        .expect("Attempt to move entity with no position");
    let (x, y) = current_position;
    let (dx, dy) = direction.unit_vector();
    let new_position = (x+dx, y+dy);

    action.remove_position(character_id);
    action.insert_position(character_id, new_position);
}

pub fn fire_projectile(character_id: EntityId, world: &World, action: &mut Action) {
    let projectile_id = spawn_entity(world);
    let mut path: &Vec<Point> = &Vec::new();
    for key in world.aim_path.keys() {
        path = world.get_aimpath(*key).unwrap();
        let (x, y) = world.get_position(*key)
                .expect("Attempt to get entity with no position");
        action.insert_position(projectile_id, (x,y));
    }
    
    let mut conv_path: Vec<Direction> = Vec::new();
    let mut conv = path.to_vec();
    for index in 0..conv.len() {
        if index + 1 >= conv.len() {
            break;
        } else {
            let mut workingC = conv[index];
            let mut workingF = conv[index + 1];

            let mut resultsX = workingF.x - workingC.x;
            let mut resultsY = workingF.y - workingC.y;

            let mut dir = Direction { x: resultsX as isize, y: resultsY as isize};
            conv_path.push(dir);
        }
    } 

    action.insert_life_time(projectile_id, conv.len() as i32);
    action.insert_velocity(projectile_id, conv_path);
    action.insert_aimpath(projectile_id, path.to_vec());
    action.insert_icon(projectile_id, String::from("*"));
    action.insert_projectile(projectile_id, ProjectileType::Bullet);
    
    action.insert_material(projectile_id, Materials::Steel);
     //action.insert_aimpath(projectile_id, );
     //action.insert_position(projectile_id, );
}

pub fn open_door(door_id: EntityId, action: &mut Action) {
    action.remove_solid(door_id);
    action.insert_tile(door_id, TileType::OpenDoor);//add this
    action.insert_door_state(door_id, DoorState::Open);//add this
}

pub fn close_door(door_id: EntityId, action: &mut Action) {
    action.insert_solid(door_id);
    action.insert_tile(door_id, TileType::ClosedDoor);//add this
    action.insert_door_state(door_id, DoorState::Closed);//add this
}

pub fn player_control(player_id: EntityId, action: &mut Action) {
    action.insert_control(player_id, Control::Player);
}

pub fn ai_control(player_id: EntityId, action: &mut Action) {
    action.insert_control(player_id, Control::AI);
}

pub fn add_turn(player_id: EntityId, action: &mut Action) {
    action.remove_given_turn(player_id);
}

pub fn exit_game() {
    terminal::close();
}

pub fn spawn_entity(world: &World) -> EntityId {
    let mut keyNum = world.position.len();
    keyNum += 1;
    return keyNum as u64;
}

pub fn despawn_entity(character_id: EntityId, action: &mut Action) {
    action.clear(character_id);
}

pub fn decrease_life_time(character_id: EntityId, world: &World, action: &mut Action) {
    let time = world.get_life_time(character_id).unwrap() - 1;
    action.remove_life_time(character_id);
    action.insert_life_time(character_id, time);
}

pub fn open_status_menu(character_id: EntityId, action: &mut Action) {
    let state = Game_State::StatusMenu;
    action.remove_game_state(character_id);
    action.insert_game_state(character_id, state);
}

pub fn close_menu(character_id: EntityId, action: &mut Action) {
    let state = Game_State::GameOn;
    action.remove_game_state(character_id);
    action.insert_game_state(character_id, state);
}


// Controls
pub fn aim_control(character_id: EntityId, world: &World, action: &mut Action) {
    //print!("control aim");
    println!("aim control start {:?}", world.get_aim(character_id));

    terminal::read_event();
    for event in terminal::events() {
        match event {
            Event::KeyPressed{ key: KeyCode::Escape, ctrl: _, shift: _ } => {
                action.remove_aim(character_id);
                action.remove_aimpath(character_id);
                action.insert_given_turn(character_id);
                break;
            }, // exit game
            Event::KeyPressed{ key: KeyCode::Up, ctrl: _, shift: _ } => {
                let mut d = Direction { x: 0, y: -1};

                let (x, y) = world.get_aim(character_id)
                        .expect("Attempt to move entity with no position");
                let (px, py) = world.get_position(character_id)
                        .expect("Attempt to move entity with no position");

                action.insert_aim(character_id, (x, y-1));
                let path = supercover_path_maker(px as i32, py as i32, x as i32, (y-1) as i32);

                action.insert_aimpath(character_id, path);
                
                break;

            },
            Event::KeyPressed{ key: KeyCode::Down, ctrl: _, shift: _ } => {
                let mut d = Direction { x: 0, y: 1 };

                let (x, y) = world.get_aim(character_id)
                        .expect("Attempt to move entity with no position");
                let (px, py) = world.get_position(character_id)
                        .expect("Attempt to move entity with no position");

                let path = supercover_path_maker(px as i32, py as i32, x as i32, (y+1) as i32);

                action.insert_aimpath(character_id, path);
                     
                action.insert_aim(character_id, (x, y+1));
                break;

            },
            Event::KeyPressed{ key: KeyCode::Left, ctrl: _, shift: _ } => {
                let mut d = Direction { x: -1, y : 0 };

                let (x, y) = world.get_aim(character_id)
                        .expect("Attempt to move entity with no position");
                let (px, py) = world.get_position(character_id)
                        .expect("Attempt to move entity with no position");

                let path = supercover_path_maker(px as i32, py as i32, (x-1) as i32, y as i32);

                action.insert_aimpath(character_id, path);
                action.insert_aim(character_id, (x - 1, y));
                break;
            },
            Event::KeyPressed{ key: KeyCode::Right, ctrl: _, shift: _ } => {
                let mut d = Direction { x: 1, y: 0 };

                let (x, y) = world.get_aim(character_id)
                        .expect("Attempt to move entity with no position");
                let (px, py) = world.get_position(character_id)
                        .expect("Attempt to move entity with no position");

                
                let path = supercover_path_maker(px as i32, py as i32, (x+1) as i32, y as i32);
                action.insert_aimpath(character_id, path);           
                action.insert_aim(character_id, (x + 1, y));
                break;
                
            },
            Event::KeyPressed{ key: KeyCode::Enter, ctrl: _, shift: _ } => {
                let path = world.get_aimpath(character_id).unwrap();
                
                fire_projectile(character_id, world, action);
                action.remove_aim(character_id);
                action.remove_aimpath(character_id);

                break;

            }
            
            _ => (),
        }
    }
    terminal::read_event();
}

pub fn pointer_control(character_id: EntityId, world: &World, action: &mut Action) {
    terminal::read_event();
    for event in terminal::events() {
        match event {
            Event::KeyPressed{ key: KeyCode::Escape, ctrl: _, shift: _ } => {
                action.remove_pointer(character_id);
                action.insert_given_turn(character_id);
                break;
            }, // exit game
            Event::KeyPressed{ key: KeyCode::Up, ctrl: _, shift: _ } => {
                let mut d = Direction { x: 0, y: -1};
                //d.x = 0;
                //d.y = -1;
                let (x, y) = world.get_pointer(character_id)
                        .expect("Attempt to move entity with no position");

                action.remove_pointer(character_id);
                action.insert_pointer(character_id, (x, y-1));
                break;
                //player.move_by(0, -1, map);

                //return false
            },
            Event::KeyPressed{ key: KeyCode::Down, ctrl: _, shift: _ } => {
                let mut d = Direction { x: 0, y: 1 };
                //d.x = 0;
                //d.y = 1;
                let (x, y) = world.get_pointer(character_id)
                        .expect("Attempt to move entity with no position");

                action.remove_pointer(character_id);
                action.insert_pointer(character_id, (x, y+1));
                break;
                //player.move_by(0, 1, map);
                //return false;
            },
            Event::KeyPressed{ key: KeyCode::Left, ctrl: _, shift: _ } => {
                //player.move_by(-1, 0, map);
                let mut d = Direction { x: -1, y : 0 };
                //d.x = -1;
                //d.y = 0;
                let (x, y) = world.get_pointer(character_id)
                        .expect("Attempt to move entity with no position");

                action.remove_pointer(character_id);
                action.insert_pointer(character_id, (x - 1, y));
                                //return false;
                break;
            },
            Event::KeyPressed{ key: KeyCode::Right, ctrl: _, shift: _ } => {
                let mut d = Direction { x: 1, y: 0 };
                //d.x = 1;
                //d.y = 0;
                let (x, y) = world.get_pointer(character_id)
                        .expect("Attempt to pointer control entity with no position");

                action.remove_pointer(character_id);
                action.insert_pointer(character_id, (x + 1, y));
                break;
                //player.move_by(1, 0, map);
                //return false;
            }
            // Event::KeyPressed{ key: KeyCode::F, ctrl: _, shift: _ } => {
            //     let mut e =;
            //     return ActionType::Fire(entity_id);
            // }

            _ => (),
        }
    }
    terminal::read_event();
}

pub fn status_control(character_id: EntityId, world: &World, action: &mut Action) {
    terminal::read_event();
    for event in terminal::events() {
        match event {
            Event::KeyPressed{ key: KeyCode::Escape, ctrl: _, shift: _ } => {
                action.remove_game_state(character_id);
                action.insert_game_state(character_id, Game_State::GameOn);
                action.insert_given_turn(character_id);
                break;
            }, // exit game
            

            _ => (),
        }
    }
    terminal::read_event();
}




// helper functions
fn supercover_path_maker(startx: i32, starty: i32, endx: i32, endy: i32) -> Vec<Point> {
    let mut path: Vec<Point> = vec![];
    //println!("Start: {},{}  $$  End: {},{}", startx, starty, endx, endy);
    for point in pathing::supercover::Supercover::new(Point::new(startx, starty), Point::new(endx, endy)) {
        //println!("{:?}", point);
        path.push(Point::new(point.x, point.y));
    }
    path
}