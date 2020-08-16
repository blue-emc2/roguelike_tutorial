use super::{Monster, Position, Viewshed};
use rltk::Point;
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = (
    ReadExpect<'a, Point>,
    ReadStorage<'a, Viewshed>,
    ReadStorage<'a, Monster>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (player_pos, viewshed, monster) = data;

    for (viewshed, _monter) in (&viewshed, &monster).join() {
      if viewshed.visible_tiles.contains(&*player_pos) {
        println!("モンスターは侮辱を叫ぶ")
      }
    }
  }
}
