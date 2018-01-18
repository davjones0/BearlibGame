use EntityStoreAfterAction;
use RuleStatus;
use ActionStatus;
use ActionType;
use std::collections::VecDeque;
use World;
use SpatialHashTable;
use Action;

pub fn aim(action: &Action, world: &World, spatial_hash: &SpatialHashTable, reactions: &mut VecDeque<ActionType>) -> (ActionStatus, RuleStatus) {
    let future_state = EntityStoreAfterAction {
        entity_store: world,
        action: action,
    };

    for (id, position) in action.additions.aim.iter() {
        reactions.push_front(ActionType::AimControl(*id));
    }
    
    return (ActionStatus::Accept, RuleStatus::KeepChecking);

}

pub fn look(action: &Action, world: &World, spatial_hash: &SpatialHashTable, reactions: &mut VecDeque<ActionType>) -> (ActionStatus, RuleStatus) {
    let future_state = EntityStoreAfterAction {
        entity_store: world,
        action: action,
    };

    for (id, position) in action.additions.pointer.iter() {
        reactions.push_front(ActionType::PointerControl(*id));
        // only proceed if this entity can actually open doors
        // if !future_state.contains_can_open_doors(&id) {  //add that contains to world
        //     continue;
        // }    for event in terminal::events() {
      
    }

    // no doors were bumped, so check other rules
    return (ActionStatus::Accept, RuleStatus::KeepChecking);

}

pub fn bump_open_doors(action: &Action, world: &World, spatial_hash: &SpatialHashTable, reactions: &mut VecDeque<ActionType>) -> (ActionStatus, RuleStatus) {
    // new
    let future_state = EntityStoreAfterAction {
        entity_store: world,
        action: action,
    };
    //println!("hit1");
    // loop through all positions set by the action
    for (id, position) in action.additions.position.iter() {

        // only proceed if this entity can actually open doors
        if !future_state.contains_can_open_doors(&id) {  //add that contains to world
            continue;
        }

        if let Some(door_id) = spatial_hash.get(position).any_door_state() {
            // if the entity would move into a cell with a door...

            //...open the door...
            reactions.push_front(ActionType::OpenDoor(door_id));

            // ...and prevent the move from occuring.
            return (ActionStatus::Reject, RuleStatus::StopChecking);
        }
    }

    // no doors were bumped, so check other rules
    return (ActionStatus::Accept, RuleStatus::KeepChecking);
}


pub fn collision(action: &Action, state: &World, spatial_hash: &SpatialHashTable, reactions: &mut VecDeque<ActionType>) -> (ActionStatus, RuleStatus) {
    //println!("hit2");
    let future_state = EntityStoreAfterAction {
        entity_store: state,
        action: action,
    };

    for (id, position) in action.additions.position.iter() {
        if !future_state.contains_solid(&id) {
            continue;
        }
        //println!("{}..{:?}",spatial_hash.get(position).is_solid(), position);
        if spatial_hash.get(position).is_solid() {
            return (ActionStatus::Reject, RuleStatus::StopChecking);
        }
    }

    return (ActionStatus::Accept, RuleStatus::KeepChecking);
}