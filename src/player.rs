use super::{CombatStats, Map, Player, Position, RunState, State, Viewshed, WantsToMelee};
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut positions = ecs.write_storage::<Position>();
  let mut players = ecs.write_storage::<Player>();
  let mut viewsheds = ecs.write_storage::<Viewshed>();
  let combat_stats = ecs.read_storage::<CombatStats>();
  let map = ecs.fetch::<Map>();
  let entities = ecs.entities();
  let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

  for (entity, _player, pos, viewsheds) in
    (&entities, &mut players, &mut positions, &mut viewsheds).join()
  {
    if pos.x + delta_x < 1
      || pos.x + delta_x > map.width - 1
      || pos.y + delta_y < 1
      || pos.y + delta_y > map.height - 1
    {
      return;
    }

    let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

    for potential_target in map.tile_content[destination_idx].iter() {
      let target = combat_stats.get(*potential_target);
      if let Some(_target) = target {
        println!("entity id={}", entity.id());
        wants_to_melee
          .insert(
            entity,
            WantsToMelee {
              target: *potential_target,
            },
          )
          .expect("Add target failed");
        return;
      }
    }

    if !map.blocked[destination_idx] {
      pos.x = min(79, max(0, pos.x + delta_x));
      pos.y = min(49, max(0, pos.y + delta_y));

      let mut ppos = ecs.write_resource::<Point>();
      ppos.x = pos.x;
      ppos.y = pos.y;

      viewsheds.dirty = true;
    }
  }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
  match ctx.key {
    None => return RunState::Paused,
    Some(key) => match key {
      VirtualKeyCode::Left | VirtualKeyCode::H | VirtualKeyCode::A => {
        try_move_player(-1, 0, &mut gs.ecs)
      }
      VirtualKeyCode::Right | VirtualKeyCode::L | VirtualKeyCode::D => {
        try_move_player(1, 0, &mut gs.ecs)
      }
      VirtualKeyCode::Up | VirtualKeyCode::K | VirtualKeyCode::W => {
        try_move_player(0, -1, &mut gs.ecs)
      }
      VirtualKeyCode::Down | VirtualKeyCode::J | VirtualKeyCode::S => {
        try_move_player(0, 1, &mut gs.ecs)
      }
      VirtualKeyCode::Y | VirtualKeyCode::E => try_move_player(1, -1, &mut gs.ecs), // 右上
      VirtualKeyCode::U | VirtualKeyCode::Q => try_move_player(-1, -1, &mut gs.ecs), // 左上
      VirtualKeyCode::N | VirtualKeyCode::C => try_move_player(1, 1, &mut gs.ecs),  // 右下
      VirtualKeyCode::B | VirtualKeyCode::Z => try_move_player(-1, 1, &mut gs.ecs), // 左下
      _ => return RunState::Paused,
    },
  }
  RunState::Running
}
