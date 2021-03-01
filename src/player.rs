use super::{
  gamelog::GameLog, CombatStats, Item, Map, Player, Position, RunState, State, Viewshed,
  WantsToMelee, WantsToPickupItem,
};
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
    None => return RunState::AwaitingInput,
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
      VirtualKeyCode::G => get_item(&mut gs.ecs),
      _ => return RunState::AwaitingInput,
    },
  }
  RunState::PlayerTurn
}

fn get_item(ecs: &mut World) {
  let player_pos = ecs.fetch::<Point>();
  let player_entity = ecs.fetch::<Entity>();
  let entities = ecs.entities();
  let items = ecs.read_storage::<Item>();
  let positions = ecs.read_storage::<Position>();
  let mut gamelog = ecs.fetch_mut::<GameLog>();

  let mut target_item: Option<Entity> = None;
  for (item_entity, _item, position) in (&entities, &items, &positions).join() {
    // println!("position.x={}, player_pos.x={}", position.x, player_pos.x);
    // println!("position.y={}, player_pos.y={}", position.y, player_pos.y);
    // プレイヤーと座標が重なる"もの"をアイテムとみなす
    if position.x == player_pos.x && position.y == player_pos.y {
      target_item = Some(item_entity);
    }
  }

  // 上の判定でなにもかからなければ何も拾ってない
  match target_item {
    None => gamelog
      .entries
      .push("There is nothing here to pick up.".to_string()),
    Some(item) => {
      let mut pickup = ecs.write_storage::<WantsToPickupItem>();
      // ここでインサートされたエンティティはItemCollectionSystemで取り出される
      pickup
        .insert(
          *player_entity,
          WantsToPickupItem {
            collected_by: *player_entity,
            item,
          },
        )
        .expect("Unable to insert want to pickup");
    }
  }
}
