use bevy::{ecs::system::Resource, utils::HashMap};

use super::Element;

#[derive(Resource)]
pub struct FallingSandRules {
    rules: HashMap<u32, u32>,
}

impl Default for FallingSandRules {
    fn default() -> Self {
        // Pre-computed air-sand rules
        Self {
            rules: HashMap::from([
                gen_rule(
                    (Element::Air, Element::Sand, Element::Air, Element::Air),
                    (Element::Air, Element::Air, Element::Air, Element::Sand),
                ),
                gen_rule(
                    (Element::Air, Element::Sand, Element::Air, Element::Sand),
                    (Element::Air, Element::Air, Element::Sand, Element::Sand),
                ),
                gen_rule(
                    (Element::Air, Element::Sand, Element::Sand, Element::Air),
                    (Element::Air, Element::Air, Element::Sand, Element::Sand),
                ),
                gen_rule(
                    (Element::Sand, Element::Air, Element::Air, Element::Air),
                    (Element::Air, Element::Air, Element::Sand, Element::Air),
                ),
                gen_rule(
                    (Element::Sand, Element::Air, Element::Air, Element::Sand),
                    (Element::Air, Element::Air, Element::Sand, Element::Sand),
                ),
                gen_rule(
                    (Element::Sand, Element::Air, Element::Sand, Element::Air),
                    (Element::Air, Element::Air, Element::Sand, Element::Sand),
                ),
                gen_rule(
                    (Element::Sand, Element::Sand, Element::Air, Element::Air),
                    (Element::Air, Element::Air, Element::Sand, Element::Sand),
                ),
                gen_rule(
                    (Element::Sand, Element::Sand, Element::Air, Element::Sand),
                    (Element::Air, Element::Sand, Element::Sand, Element::Sand),
                ),
                gen_rule(
                    (Element::Sand, Element::Sand, Element::Sand, Element::Air),
                    (Element::Sand, Element::Air, Element::Sand, Element::Sand),
                ),
            ]),
        }
    }
}

impl FallingSandRules {
    pub fn get_result(&self, input_rule: u32) -> u32 {
        match self.rules.get(&input_rule) {
            Some(&result) => result,
            None => input_rule,
        }
    }
}

pub fn to_rule_state(input: (Element, Element, Element, Element)) -> u32 {
    ((input.0 as u32) << 24) + ((input.1 as u32) << 16) + ((input.2 as u32) << 8) + input.3 as u32
}

fn gen_rule(
    input: (Element, Element, Element, Element),
    output: (Element, Element, Element, Element),
) -> (u32, u32) {
    (to_rule_state(input), to_rule_state(output))
}
