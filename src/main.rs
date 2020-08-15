use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::*;
mod visibility_system;
use visibility_system::VisibilitySystem;

pub struct State {
  ecs: World,
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut Rltk) {
    ctx.cls();

    player_input(self, ctx);
    self.run_systems();

    draw_map(&self.ecs, ctx);

    let positions = self.ecs.read_storage::<Position>();
    let renderables = self.ecs.read_storage::<Renderable>();
    let map = self.ecs.fetch::<Map>();

    for (pos, render) in (&positions, &renderables).join() {
      let idx = Map::xy_idx(pos.x, pos.y);
      if map.visible_tiles[idx] {
        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
      }
    }
  }
}

impl State {
  fn run_systems(&mut self) {
    let mut vis = VisibilitySystem {};
    vis.run_now(&self.ecs);
    self.ecs.maintain();
  }
}

fn main() -> rltk::BError {
  use rltk::RltkBuilder;
  let context = RltkBuilder::simple80x50()
    .with_title("Roguelike Tutorial")
    .build()?;
  let mut gs = State { ecs: World::new() };
  gs.ecs.register::<Position>();
  gs.ecs.register::<Renderable>();
  gs.ecs.register::<Player>();
  gs.ecs.register::<Viewshed>();

  let map: Map = Map::new_map_rooms_and_corridors();
  let (player_x, player_y) = map.rooms[0].center();

  gs.ecs
    .create_entity()
    .with(Position {
      x: player_x,
      y: player_y,
    })
    .with(Renderable {
      glyph: rltk::to_cp437('@'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::BLACK),
    })
    .with(Player {})
    .with(Viewshed {
      visible_tiles: Vec::new(),
      range: 8,
      dirty: true,
    })
    .build();

  let mut rng = rltk::RandomNumberGenerator::new();
  // skip(1): ゲーム開始時にプレイヤーと同じ部屋に敵を配置させない
  for room in map.rooms.iter().skip(1) {
    let (x, y) = room.center();
    let glyph;
    let roll = rng.roll_dice(1, 2);
    match roll {
      1 => glyph = rltk::to_cp437('g'),
      _ => glyph = rltk::to_cp437('o'),
    }

    gs.ecs
      .create_entity()
      .with(Position { x, y })
      .with(Renderable {
        glyph: glyph,
        fg: RGB::named(rltk::RED),
        bg: RGB::named(rltk::BLACK),
      })
      .with(Viewshed {
        visible_tiles: Vec::new(),
        range: 8,
        dirty: true,
      })
      .build();
  }

  gs.ecs.insert(map);

  rltk::main_loop(context, gs)
}
