use super::{Monster, Position, Viewshed};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = (
    ReadStorage<'a, Viewshed>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Monster>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (viewshed, pos, monster) = data;

    for (viewshed, pos, _monter) in (&viewshed, &pos, &monster).join() {
      println!("モンスターは自分の存在を考える");
    }
  }
}
