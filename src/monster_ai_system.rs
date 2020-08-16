use super::{Monster, Name, Viewshed};
use rltk::Point;
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = (
    ReadExpect<'a, Point>,
    ReadStorage<'a, Viewshed>,
    ReadStorage<'a, Monster>,
    ReadStorage<'a, Name>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (player_pos, viewshed, monster, name) = data;

    for (viewshed, _monter, name) in (&viewshed, &monster, &name).join() {
      if viewshed.visible_tiles.contains(&*player_pos) {
        println!("{} モンスターは侮辱を叫ぶ", name.name)
      }
    }
  }
}
