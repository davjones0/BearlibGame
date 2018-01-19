/************
entity component wip


***********************/
extern crate bear_lib_terminal;
extern crate num_traits;


use bear_lib_terminal::Color;
use bear_lib_terminal::geometry::{Point, Size};
use bear_lib_terminal::terminal::{self, config, Event, KeyCode};
use bear_lib_terminal::terminal::config::{Cellsize, font};

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

mod materials;
mod pathing;
mod rules;
mod actions;

#[derive(Copy, Clone)]
pub enum Materials {
    Steel,
}

#[derive(Copy, Clone)]
pub enum DoorState {
    Open,
    Closed,
}
#[derive(Clone)]
pub enum TileType {
    OpenDoor,
    ClosedDoor,
}
#[derive(Copy, Clone, PartialEq)]
pub enum Control {
    Player,
    AI,
}
const WIDTH: i32 = 20;
const HEIGHT: i32 = 20;

type EntityId = u64;

//entity master list of what currently exists
//also defines components
#[derive(Clone)]
pub struct World {
    //mask: [i32, ENTITY_COUNT],
    //location: [location, ENTITY_COUNT],
    //appearance: [appearance, ENTITY_COUNT],
    //velocity: [velocity, ENTITY_COUNT],
    //input: [input, ENTITY_COUNT],
    //solid: [solid, ENTITY_COUNT],
    position: HashMap<EntityId, (isize, isize)>, //data components
    door_state: HashMap<EntityId, DoorState>,
    tile: HashMap<EntityId, TileType>,
    icon: HashMap<EntityId, String>,
    control: HashMap<EntityId, Control>,
    material: HashMap<EntityId, Materials>,
    pointer: HashMap<EntityId, (isize, isize)>,
    aim: HashMap<EntityId, (isize, isize)>,
    aim_path: HashMap<EntityId, Vec<Point>>,
    solid: HashSet<EntityId>, //flag component
    can_open_doors: HashSet<EntityId>,//flag components
}

impl World {

    fn get_position(&self, id: EntityId) -> Option<(isize,isize)> {
       self.position.get(&id).map(|v| *v)
    }

    fn get_door_state(&self, id: EntityId) -> Option<DoorState> {
        self.door_state.get(&id).map(|z| *z)
    }

    fn get_control(&self, id: EntityId) -> Option<Control> {
        self.control.get(&id).map(|y| *y)
    }

    fn get_material(&self, id: EntityId) -> Option<Materials> {
        self.material.get(&id).map(|y| *y)
    }

    fn get_icon(&self, id: EntityId) -> Option<String> {
        let k = format!("{}", self.icon.get(&id).unwrap());//.map(|y| *y)
        Some(String::from(k))      
    }

    fn get_pointer(&self, id: EntityId) -> Option<(isize,isize)> {
       self.pointer.get(&id).map(|v| *v)
    }

    fn get_aim(&self, id: EntityId) -> Option<(isize,isize)> {
       self.aim.get(&id).map(|v| *v)
    }

    fn get_aimpath(&self, id: EntityId) -> Option<&Vec<Point>> {
        self.aim_path.get(&id).map(|x| x)
    }

    fn contains_solid(&self, id: EntityId) -> bool {
        self.solid.contains(&id)
    }

    fn contains_can_open_doors(&self, id: EntityId) -> bool {
        self.can_open_doors.contains(&id)
    }

    fn contains_pointer(&self, id: EntityId) -> bool {
        self.pointer.contains_key(&id)
    }

    fn contains_aim(&self, id: EntityId) -> bool {
        self.aim.contains_key(&id)
    }

    fn contains_aimpath(&self, id: EntityId) -> bool {
        self.aim_path.contains_key(&id)
    }

    // pub fn createEntity(&self) -> usize {
    //     for entity in 0..ENTITY_COUNT {
    //         if self.mask[entity] == Component::COMPONENT_NONE {
    //             entity
    //         }
    //     }
    //     println!("Error! No more entities left\n");
    //     ENTITY_COUNT;
    // }

    // pub fn destroyEntity(&self, entity: usize) {
    //     self.mask[entity] = Component::COMPONENT_NONE;
    // }
}

#[derive(Clone)]
struct RemovedComponents {
    position: HashSet<EntityId>,
    door_state: HashSet<EntityId>,
    tile: HashSet<EntityId>,
    control: HashSet<EntityId>,
    material: HashSet<EntityId>,
    pointer: HashSet<EntityId>,
    aim: HashSet<EntityId>,
    aim_path: HashSet<EntityId>,
    solid: HashSet<EntityId>,
    can_open_doors: HashSet<EntityId>,
    icon: HashSet<EntityId>
}

#[derive(Clone)]
pub struct Action {
    additions: World,
    removals: RemovedComponents,
}

impl Action {
    pub fn remove_position(&mut self, id: EntityId) {
        self.removals.position.insert(id);
        //self.additions.position.remove(&id);
    }

    pub fn remove_control(&mut self, id: EntityId) {
        self.removals.control.insert(id);
    }

    pub fn remove_material(&mut self, id: EntityId) {
        self.removals.material.insert(id);
    }

    pub fn remove_solid(&mut self, id: EntityId) {
        self.removals.solid.insert(id);
    }

    pub fn remove_icon(&mut self, id: EntityId) {
        self.removals.icon.insert(id);
    }

    pub fn remove_pointer(&mut self, id: EntityId) {
        self.removals.pointer.insert(id);
    }

    pub fn remove_aim(&mut self, id: EntityId) {
        self.removals.aim.insert(id);
    }

    pub fn remove_aimpath(&mut self, id: EntityId) {
        self.removals.aim_path.insert(id);
    }

    //more components
    pub fn insert_position(&mut self, id: EntityId, value: (isize, isize)) {
        self.additions.position.insert(id, value);
    }

    pub fn insert_control(&mut self, id: EntityId, value: Control) {
        self.additions.control.insert(id, value);
    }

    pub fn insert_material(&mut self, id: EntityId, value: Materials) {
        self.additions.material.insert(id, value);
    }

    pub fn insert_pointer(&mut self, id: EntityId, value: (isize, isize)) {
        self.additions.pointer.insert(id, value);
    }

    pub fn insert_aim(&mut self, id: EntityId, value: (isize, isize)) {
        self.additions.aim.insert(id, value);
    }

    pub fn insert_aimpath(&mut self, id: EntityId, value: Vec<Point>) {
        self.additions.aim_path.insert(id, value);
    }

    pub fn insert_tile(&mut self, id: EntityId, value: TileType) {
        self.additions.tile.insert(id, value);
    }

    pub fn insert_door_state(&mut self, id: EntityId, value: DoorState) {
        self.additions.door_state.insert(id, value);
    }

    pub fn insert_icon(&mut self, id: EntityId, value: String) {
        self.additions.icon.insert(id, value);
    }

    // more components
    pub fn insert_solid(&mut self, id: EntityId) {
        self.additions.solid.insert(id);
    }
    // more flags

    pub fn instantiate_from(&mut self, action_type: ActionType, world: &World) {
        create_action(action_type, world, self);
    }

    pub fn clear(&mut self) {
        self.additions.position.clear();
        self.additions.door_state.clear();
        self.additions.tile.clear();
        self.additions.control.clear();
        self.additions.material.clear();
        self.additions.solid.clear();
        self.additions.can_open_doors.clear();
        self.additions.icon.clear();
        self.additions.pointer.clear();
        self.additions.aim.clear();
        self.additions.aim_path.clear();

        self.removals.position.clear();
        self.removals.door_state.clear();
        self.removals.tile.clear();
        self.removals.control.clear();
        self.removals.material.clear();
        self.removals.solid.clear();
        self.removals.can_open_doors.clear();
        self.removals.icon.clear();
        self.removals.pointer.clear();
        self.removals.aim.clear();
        self.removals.aim_path.clear();
    }
}


// applies 'action' to 'state', clearing 'action' in the process
fn commit_action(world: &mut World, action: &mut Action) {

    // removals
    for id in action.removals.position.drain() {
        world.position.remove(&id);
    }

    for id in action.removals.control.drain() {
        world.control.remove(&id);
    }

    for id in action.removals.tile.drain() {
        world.tile.remove(&id);
    }

    for id in action.removals.pointer.drain() {
        world.pointer.remove(&id);
    }

    for id in action.removals.material.drain() {
        world.material.remove(&id);
    }

    for id in action.removals.aim.drain() {
        world.aim.remove(&id);
    }

    for id in action.removals.aim_path.drain() {
        world.aim_path.remove(&id);
    }

    //data insertions
    for (id, value) in action.additions.position.drain() {
        world.position.insert(id, value);
    }

    for (id, value) in action.additions.control.drain() {
        world.control.insert(id, value);
    }

    for (id, value) in action.additions.tile.drain() {
        world.tile.insert(id, value);
    }

    for (id, value) in action.additions.pointer.drain() {
        world.pointer.insert(id, value);
    }

    for (id, value) in action.additions.aim.drain() {
        world.aim.insert(id, value);
    }

    for (id, value) in action.additions.aim_path.drain() {
        world.aim_path.insert(id, value);
    }

    for (id, value) in action.additions.material.drain() {
        world.material.insert(id, value);
    }

    // flag insertions
    for id in action.additions.solid.drain() {
        world.solid.insert(id);
    }

    for (id, value) in action.additions.icon.drain() {
        world.icon.insert(id, value);
    }
}

#[derive(Debug)]
pub struct Direction {
    x: isize,
    y: isize,
}

impl Direction {

    pub fn unit_vector(&self) -> (isize, isize) {
        return (self.x, self.y);
    }
}


#[derive(Debug)]
pub enum ActionType {
    MoveCharacter(EntityId, Direction),
    StartPointer(EntityId),
    MovePointer(EntityId, Direction),
    PointerControl(EntityId),
    StartAim(EntityId),
    MoveAim(EntityId, Direction),
    AimControl(EntityId),
    OpenDoor(EntityId),
    CloseDoor(EntityId),
    PlayerControl(EntityId),
    AIControl(EntityId),
    Exit,
}

fn create_action(action_type: ActionType, world: &World, action: &mut Action) {
    // action is assumed to be initially empty

    match action_type {
        ActionType::MoveCharacter(entity_id, direction) => {
            actions::move_character(entity_id, direction, world, action);
        }
        ActionType::StartPointer(entity_id) => {
            actions::start_pointer(entity_id, world, action);
        }
        ActionType::StartAim(entity_id) => {
            actions::start_aim(entity_id, world, action);
        }
        ActionType::MovePointer(entity_id, direction) => {
            actions::move_pointer(entity_id, direction, world, action);
        }
        ActionType::MoveAim(entity_id, direction) => {
            actions::move_aim(entity_id, direction, world, action);
        }
        ActionType::OpenDoor(entity_id) => {
            actions::open_door(entity_id, action);
        }
        ActionType::CloseDoor(entity_id) => {
            actions::close_door(entity_id, action);
        }
        ActionType::PlayerControl(entity_id) => {
            actions::player_control(entity_id, action);
        }
        ActionType::PointerControl(entity_id) => {
            actions::pointer_control(entity_id, world, action);
        }
        ActionType::AimControl(entity_id) => {
            actions::aim_control(entity_id, world, action);
        }
        ActionType::AIControl(entity_id) => {
            actions::ai_control(entity_id, action);
        }
        ActionType::Exit => {
            actions::exit_game();
        }

    }
}

#[derive(PartialEq)]
pub enum ActionStatus {
    Accept,
    Reject,
}

#[derive(PartialEq)]
pub enum RuleStatus {
    KeepChecking,
    StopChecking,
}

// fn aiming() -> (ActionStatus, RuleStatus) {
//     let future_state = EntityStoreAfterAction {
//         entity_store: world,
//         action: action,
//     };

//     for (id,
// }


#[derive(Debug)]
pub struct SpatialHashTable {
    data: Vec<SpatialHashCell>//HashMap<(isize, isize), SpatialHashCell>
}

impl SpatialHashTable {
    fn build(&mut self, world: &World, id: EntityId) {
        let (x,y) = world.get_position(id).unwrap();
        let k = x as i32 + WIDTH * y as i32;

        let data = &mut self.data[k as usize];
        data.entities.insert(id);

        if world.contains_solid(id) {
            data.solid += 1;
        }
        if world.get_door_state(id).is_some() {
            data.door_state.insert(id);
        }
    }

    fn update(&mut self, action: &Action) {
        let solidCount = 0;
        let r: u64 = 1;

        for key in action.additions.position.keys() {
            let (x, y) = action.additions.get_position(*key).unwrap();
            let index = x as i32 + WIDTH * y as i32;
            let mut o = &mut self.data[index as usize];
            o.entities.insert(*key);
            if action.additions.contains_solid(*key) {
                o.solid += 1;
            } else {
                o.solid = 0;
            }

            if action.additions.get_door_state(*key).is_some() {
                o.door_state.insert(*key);
            }
            //println!("ss{}", key);
        }
        //println!("ss{:?}", action.additions.position.get(&r));
        /*for (id, position) in action.additions.position.iter() {
            let (x, y) = *position;
            let k = x as i32 + WIDTH * y as i32;
            println!("{}", k);
            let mut o = &mut self.data[k as usize];//.map(|v| *v);
            o.entities.insert(*id);
            if action.additions.contains_solid(*id) {
                o.solid += 1;
            }

            if action.additions.get_door_state(*id).is_some() {
                o.door_state.insert(*id);
            }

        }*/
    }

    fn get(&self, pos: &(isize, isize)) -> &SpatialHashCell {
        let (x, y) = *pos;
        let k =  x as i32 + WIDTH * y as i32;
        &self.data[k as usize]
        //get((x,y)).map(|v| *v)
    }
}

#[derive(Clone, Debug)]
struct SpatialHashCell {
    // all the entities in this cell
    entities: HashSet<EntityId>,

    // keep track of the number of solid entities int this cell
    solid: usize,

    // maintain a set of entities with the 'door-state' component in this cell
    door_state: HashSet<EntityId>,
}

impl SpatialHashCell {
    // returns true if there is at least one solid entity in this cell
    fn is_solid(&self) -> bool {
        self.solid > 0
    }

    // returns the id of an arbitrarily chosen entity in this cell with the 'door_state' component
    fn any_door_state(&self) -> Option<EntityId> {
        self.door_state.iter().next().map(|s| *s)
    }
}

struct EntityStoreAfterAction<'a> {
    entity_store: &'a World,
    action: &'a Action,
}

// the same getters as an EntityStore
impl<'a> EntityStoreAfterAction<'a> {
    fn get_position(&self, id: EntityId) -> Option<(isize, isize)> {

        // if the component is being inserted, return it
        if let Some(value) = self.action.additions.get_position(id) {
            return Some(value);
        }

        // if the component is being removed, prevent the original value from being returned
        if self.action.removals.position.contains(&id) {
            return None;
        }

        // return the original value
        return self.entity_store.get_position(id);
    }

    
    fn contains_can_open_doors(&self, id: &EntityId) -> bool {
        self.entity_store.can_open_doors.contains(id)
    }

    fn contains_solid(&self, id: &EntityId) -> bool {
        self.entity_store.solid.contains(id)
    }
}



// the type of a rule function (e.g. collision)
type RuleFn = fn(&Action, &World, &SpatialHashTable, &mut VecDeque<ActionType>) -> (ActionStatus, RuleStatus);  // can prolly figure this out look at type example online

// knows which entity's turn it is
struct TurnSchedule {
    Turn: VecDeque<EntityId>,
 }

impl TurnSchedule {
    fn next_turn(&mut self) -> Option<EntityId> {
        self.Turn.pop_front()
    }

    fn insert(&mut self, x: EntityId) {
        self.Turn.push_back(x);
    }
}

struct Game {
    // All entities and components in the game world.
    state: World,

    // List of rules in the order they will be checked.
    rules: Vec<RuleFn>,

    // Used to determine whose turn it is
    schedule: TurnSchedule,

    // It turns out you only need to have a single action
    // instantiated at a time. Store this as part of the
    // game to remove the overhead of creating a new
    // action each time we need one.
    action: Action,
    map: SpatialHashTable,
    height: usize,
    width: usize,
    // A queue of actions waiting to be processed in the
    // current turn.
    pending_actions: VecDeque<ActionType>,

    // Rules have the ability to enqueue follow-on actions,
    // which will also be processed by rules. The follow-on
    // actions enqueued by a rule as it checks an action
    // are only added to pending_actions if the action being
    // checked gets accepted. Follow-on actions are
    // temporarily stored here, and added to pending_actions
    // if the current action is accepted.
    //
    // There is a separate queue for actions enqueued by
    // accepting rules and rejecting rules.  This allows
    // accepting rules to enqueue actions that will only
    // occur if the action ends up getting accepted.
    follow_on_accepted: VecDeque<ActionType>,
    follow_on_rejected: VecDeque<ActionType>,
    follow_on_current: VecDeque<ActionType>,
}

fn handle_keys(world: &World, entity_id: EntityId) -> ActionType {
    //let key = terminal::wait_event();

    for event in terminal::events() {
        match event {
            Event::KeyPressed{ key: KeyCode::Escape, ctrl: _, shift: _ } => return ActionType::Exit, // exit game
            Event::KeyPressed{ key: KeyCode::Up, ctrl: _, shift: _ } => {
                let mut d = Direction { x: 0, y: -1};
                //d.x = 0;
                //d.y = -1;
                return ActionType::MoveCharacter(entity_id, d);
                //player.move_by(0, -1, map);

                //return false
            },
            Event::KeyPressed{ key: KeyCode::Down, ctrl: _, shift: _ } => {
                let mut d = Direction { x: 0, y: 1 };
                //d.x = 0;
                //d.y = 1;
                return ActionType::MoveCharacter(entity_id, d);
                //player.move_by(0, 1, map);
                //return false;
            },
            Event::KeyPressed{ key: KeyCode::Left, ctrl: _, shift: _ } => {
                //player.move_by(-1, 0, map);
                let mut d = Direction { x: -1, y : 0 };
                //d.x = -1;
                //d.y = 0;
                return ActionType::MoveCharacter(entity_id, d);
                //return false;
            },
            Event::KeyPressed{ key: KeyCode::Right, ctrl: _, shift: _ } => {
                let mut d = Direction { x: 1, y: 0 };
                //d.x = 1;
                //d.y = 0;
                return ActionType::MoveCharacter(entity_id, d);
                //player.move_by(1, 0, map);
                //return false;
            },
            Event::KeyPressed{ key: KeyCode::K, ctrl: _, shift: _ } => {
                return ActionType::StartPointer(entity_id);
            },
            Event::KeyPressed{ key: KeyCode::F, ctrl: _, shift: _ } => {
                return ActionType::StartAim(entity_id);
            }
            // Event::KeyPressed{ key: KeyCode::F, ctrl: _, shift: _ } => {
            //     let mut e =;
            //     return ActionType::Fire(entity_id);
            // }

            _ => (),
        }
    }

    ActionType::Exit
}

fn CHOOSE_ACTION(world: &World, entity_id: EntityId) -> ActionType {
    let controlType = world.get_control(entity_id);
    if controlType == Some(Control::Player) {
        handle_keys(world, entity_id)
    } else {
        ActionType::AIControl(entity_id)
    }
}

impl Game {
    fn game_loop(&mut self) {
        loop {
            // Figure out whose turn it is.
            let entity_id: EntityId = self.schedule.next_turn().unwrap_or_default();
            // The current entity decides an action.
            // This waits for player input if it's
            // the player's turn, and invokes the AI
            // if it's an NPC's turn.
            // The details of choosing an action are
            // out of scope.
            
            //eat extra inputs
            terminal::read_event();

            let action_type: ActionType = CHOOSE_ACTION(&self.state, entity_id); // need to figure this one out done
            println!("{:?}", action_type);

            //eat extra inputs
            terminal::read_event();

            // Equeue the action for processing

            self.pending_actions.push_back(action_type);
            //println!("an event{:?}",terminal::read_event());

            // Check rules, and handle any follow-on
            // actions.
            self.process_actions();

            // Allow the entity to take another turn
            // at some point in the future.
            self.schedule.insert(entity_id);
            //terminal::refresh();

            //terminal::has_input();
        }
    }

    fn process_actions(&mut self) {

        // Repeat until there are no pending actions.
        while let Some(action_type) = self.pending_actions.pop_front() {
            // Populate self.action based on the
            // value of action_type.
            self.action.instantiate_from(action_type, &self.state); // need to figure this one out

            let mut accepted = true;

            // For each rule
            for rule in self.rules.iter() {

                // Check the rule
                let (action_status, rule_status) = rule(&self.action, &self.state, &self.map, &mut self.follow_on_current); //figure this out

                // If a single rule rejects an action,
                // the action is rejected.
                if action_status == ActionStatus::Reject {
                    accepted = false;

                    // Drain follow-on actions into
                    // rejected queue/
                    for a in self.follow_on_current.drain(..) {
                        self.follow_on_rejected.push_back(a)
                    }
                } else {
                    // Drain follow-on actions into
                    // accepted queue.
                    for a in self.follow_on_current.drain(..) { // look into the drain(..)
                        self.follow_on_accepted.push_back(a);
                    }
                }

                // Stop checking rules if the rule say so.
                if rule_status == RuleStatus::StopChecking {
                    break;
                }
            }

            if accepted {

                // Apply the action, clearing the action in the
                // process.
                self.map.update(&self.action);
                commit_action(&mut self.state, &mut self.action);
                //println!("{:?}", self.map.data[(5 + WIDTH * 14) as usize]);
                // It's only necessary to re-draw the scene after
                // something has changed.
                // The details of rendering are out of scope.
                RENDER(&self.state, self.width, self.height); // I can handle this with bearlib: render need position, and what to draw

                // Enqueue all the follow-on actions.
                for a in self.follow_on_accepted.drain(..) {
                    self.pending_actions.push_back(a);
                }

            } else {
                // The action was rejected.
                // Clear the action
                self.action.clear();

                // Enqueue all the follow-on actions.
                for a in self.follow_on_rejected.drain(..) {
                    self.pending_actions.push_back(a);
                }
            }
        }
    }
}

fn RENDER(world: &World, width: usize, height: usize) {
    terminal::clear(None);
    if world.contains_pointer(0) {
            let (px, py) = world.get_pointer(0).unwrap();
            terminal::print_xy(px as i32, py as i32, "X");
            //terminal::refresh();
    }

    if world.contains_aim(0) {
        let (px, py) = world.get_aim(0).unwrap();
        terminal::print_xy(px as i32, py as i32, "X");
    }

    if world.contains_aim(0) {
        let path =  world.get_aimpath(0);
        if path.is_some() {
            for point in path.unwrap().iter() {
                terminal::print_xy(point.x as i32, point.y as i32, "-");
            } 
        }
    }

    for key in world.position.keys() {
        let (x, y) = world.get_position(*key).unwrap();
        let icon = world.get_icon(*key);
        //println!("{},{}", x, y);
        //terminal::with_foreground(Color::from_rgb(100,100,100), || terminal::put_xy(x as i32, y as i32, icon.unwrap()));
        let st = icon.unwrap();//format!("{}", icon.unwrap();
        //let st = format!("[font=huge][U+2588][/font]");
        // full block [U+2588]
        
        terminal::print_xy(x as i32, y as i32, &st);
    }
            terminal::refresh();


}

fn main() {
	terminal::open("Simple example", 21, 21);
    let code = String::from("437");
	terminal::set(config::Window::empty().resizeable(true).cellsize(Cellsize::Sized(Size::new(8,8))));
    terminal::set(font::bitmap(font::Origin::Root, "Andux_cp866ish.png").codepage(code).size(Size::new(8, 12)).font_name(String::from("huge")));
	terminal::composition(true);
    terminal::refresh();
    //framerate

    terminal::delay(1000/29);


    let thisone = Point::new(0,0);
    let thatone = Point::new(6,3);
    //println!("{:?}", pathing::supercover::supercover_line(thisone, thatone));

    type RuleFn = fn(&Action, &World, &SpatialHashTable, &mut VecDeque<ActionType>) -> (ActionStatus, RuleStatus);  // can prolly figure this out look at type example online

    let mut ruleContainer: Vec<RuleFn> = Vec::new();
    ruleContainer.push(rules::aim);
    ruleContainer.push(rules::look);
    ruleContainer.push(rules::bump_open_doors);
    ruleContainer.push(rules::collision);

    let mut pending: VecDeque<ActionType> = VecDeque::new();
    let mut accept: VecDeque<ActionType> = VecDeque::new();
    let mut reject: VecDeque<ActionType> = VecDeque::new();
    let mut current: VecDeque<ActionType> = VecDeque::new();

    let mut turn: VecDeque<EntityId> = VecDeque::new();

    let mut pos1: HashMap<EntityId, (isize, isize)> = HashMap::new();
    let mut pos2: HashMap<EntityId, (isize, isize)> = HashMap::new();

    let mut doorst1: HashMap<EntityId, DoorState> = HashMap::new();
    let mut doorst2: HashMap<EntityId, DoorState> = HashMap::new();

    let mut til1: HashMap<EntityId, TileType> = HashMap::new();
    let mut til2: HashMap<EntityId, TileType> = HashMap::new();

    let mut ico1: HashMap<EntityId, String> = HashMap::new();
    let mut ico2: HashMap<EntityId, String> = HashMap::new();

    let mut cont1: HashMap<EntityId, Control> = HashMap::new();
    let mut cont2: HashMap<EntityId, Control> = HashMap::new();
    let mut sol1: HashSet<EntityId> = HashSet::new();
    let mut sol2: HashSet<EntityId> = HashSet::new();
    let mut cod1: HashSet<EntityId> = HashSet::new();
    let mut cod2: HashSet<EntityId> = HashSet::new();
    
    pos1.insert(0, (10,10));
    ico1.insert(0, String::from("@"));
    cont1.insert(0, Control::Player);

    pos1.insert(1, (5,15));
    ico1.insert(1, String::from("[font=huge][U+2588][/font]"));

    pos1.insert(2, (6,15));
    ico1.insert(2, String::from("[font=huge][U+2588][/font]"));

    pos1.insert(3, (5,14));
    ico1.insert(3, String::from("[font=huge][U+2588][/font]"));
    sol1.insert(1);    
    sol1.insert(0);
    sol1.insert(2);
    sol1.insert(3);

    //let mut cell: Vec<SpatialHashCell> = vec![SpatialHashCell{}]
    let mut game = Game {
        state: World {
            position: pos1, //data components
            door_state: doorst1,
            tile: til1,
            icon: ico1,
            control: cont1,
            pointer: HashMap::new(),
            material: HashMap::new(),
            aim: HashMap::new(),
            aim_path: HashMap::new(),
            solid: sol1, //flag component
            can_open_doors: cod1,
        },
        rules: ruleContainer,
        schedule: TurnSchedule {
            Turn: turn
        },
        action: Action {
            additions: World {
                position: pos2, //data components
                door_state: doorst2,
                tile: til2,
                icon: ico2,
                control: cont2,
                pointer: HashMap::new(),
                material: HashMap::new(),
                aim: HashMap::new(),
                aim_path: HashMap::new(),
                solid: sol2, //flag component
                can_open_doors: cod2,
            },
            removals: RemovedComponents {
                position: HashSet::new(),
                door_state: HashSet::new(),
                tile: HashSet::new(),
                control: HashSet::new(),
                pointer: HashSet::new(),
                material: HashSet::new(),
                solid: HashSet::new(),
                aim: HashSet::new(),
                aim_path: HashSet::new(),
                can_open_doors: HashSet::new(),
                icon: HashSet::new()
            }
        },
        map: SpatialHashTable {
            data: vec![SpatialHashCell{
                entities: HashSet::new(),
                solid: 0,
                door_state: HashSet::new(),
            }; (WIDTH*HEIGHT) as usize]
        },
        height: HEIGHT as usize,
        width: WIDTH as usize,
        pending_actions: pending,
        follow_on_accepted: accept,
        follow_on_rejected: reject,
        follow_on_current: current,
    };
    //game.action.insert_icon(0, '@');
    //game.action.insert_position(0, (10,10));
    //game.action.insert_solid(0);
    //game.action.insert_control(0,Control::Player);
    //println!("{:?}", game.state.get_icon(0));
    //game.map.build(&game.state, 0);
    game.map.build(&game.state, 1);
    game.map.build(&game.state, 2);
    game.map.build(&game.state, 3);
    game.game_loop();
}


/********************/
