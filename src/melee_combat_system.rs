use super::{CombatStats, Name, SufferDamage, WantsToMelee};
use rltk::console;
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, WantsToMelee>,
    ReadStorage<'a, Name>,
    ReadStorage<'a, CombatStats>,
    WriteStorage<'a, SufferDamage>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (entities, mut wants_melee, names, combat_stats, mut inflict_damage) = data;

    for (_entity, wants_melee, name, stats) in
      (&entities, &wants_melee, &names, &combat_stats).join()
    {
      // Player(stats)が生存している事をチェックする
      if stats.hp > 0 {
        println!("target id={}", wants_melee.target.id());
        let target_stats = combat_stats.get(wants_melee.target).unwrap();
        // 敵(target_stats)が生存している事をチェックする
        if target_stats.hp > 0 {
          let target_name = names.get(wants_melee.target).unwrap();

          // Player攻撃力 - 敵の防御
          let damage = i32::max(0, stats.power - target_stats.defense);

          if damage == 0 {
            console::log(&format!(
              "{} is unable to hurt {}",
              &name.name, &target_name.name
            ));
          } else {
            console::log(&format!(
              "{} hits {}, for {} hp.",
              &name.name, &target_name.name, damage
            ));
            SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
          }
        }
      }
    }

    wants_melee.clear();
  }
}