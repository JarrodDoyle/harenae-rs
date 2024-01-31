use bevy::{ecs::system::Resource, utils::HashMap};

use super::Element;

struct RuleConfig {
    element_groups: Vec<Vec<Element>>,
    rules: Vec<([usize; 4], [usize; 4])>,
}

impl RuleConfig {
    fn compile(&self) -> HashMap<u32, u32> {
        let mut rules = HashMap::new();
        for i in 0..self.rules.len() {
            let rule = self.rules[i];

            let i0 = &self.element_groups[rule.0[0]];
            let i1 = &self.element_groups[rule.0[1]];
            let i2 = &self.element_groups[rule.0[2]];
            let i3 = &self.element_groups[rule.0[3]];

            let o0 = &self.element_groups[rule.1[0]];
            let o1 = &self.element_groups[rule.1[1]];
            let o2 = &self.element_groups[rule.1[2]];
            let o3 = &self.element_groups[rule.1[3]];

            // !HACK: Assume element groups are length 1 for now!
            let input = to_rule_state((i0[0], i1[0], i2[0], i3[0]));
            let output = to_rule_state((o0[0], o1[0], o2[0], o3[0]));
            rules.insert(input, output);
        }

        rules
    }
}

#[derive(Resource)]
pub struct FallingSandRules {
    rules: HashMap<u32, u32>,
}

impl Default for FallingSandRules {
    fn default() -> Self {
        // Pre-computed rules
        // I should really start building a tool for this instead of manually calculating it huh
        let rule_config = RuleConfig {
            element_groups: vec![
                vec![Element::Air],
                vec![Element::Sand],
                vec![Element::Water],
            ],
            rules: vec![
                // Air/Sand
                ([0, 1, 0, 0], [0, 0, 0, 1]),
                ([0, 1, 0, 1], [0, 0, 1, 1]),
                ([0, 1, 1, 0], [0, 0, 1, 1]),
                ([1, 0, 0, 0], [0, 0, 1, 0]),
                ([1, 0, 0, 1], [0, 0, 1, 1]),
                ([1, 0, 1, 0], [0, 0, 1, 1]),
                ([1, 1, 0, 0], [0, 0, 1, 1]),
                ([1, 1, 0, 1], [0, 1, 1, 1]),
                ([1, 1, 1, 0], [1, 0, 1, 1]),
                // Air/Water
                ([0, 2, 0, 0], [0, 0, 0, 2]),
                ([0, 2, 0, 2], [0, 0, 2, 2]),
                ([0, 2, 2, 0], [0, 0, 2, 2]),
                ([2, 0, 0, 0], [0, 0, 2, 0]),
                ([2, 0, 0, 2], [0, 0, 2, 2]),
                ([2, 0, 2, 0], [0, 0, 2, 2]),
                ([2, 2, 0, 0], [0, 0, 2, 2]),
                ([2, 2, 0, 2], [0, 2, 2, 2]),
                ([2, 2, 2, 0], [2, 0, 2, 2]),
                ([0, 2, 2, 2], [2, 0, 2, 2]),
                ([2, 0, 2, 2], [0, 2, 2, 2]),
                // Air/Sand/Water
                ([0, 1, 0, 2], [0, 0, 2, 1]),
                ([0, 1, 1, 2], [0, 2, 1, 1]),
                ([0, 1, 2, 0], [0, 0, 2, 1]),
                ([0, 1, 2, 1], [2, 0, 1, 1]),
                ([0, 1, 2, 2], [0, 2, 2, 1]),
                ([0, 2, 0, 1], [0, 0, 2, 1]),
                ([0, 2, 1, 0], [0, 0, 1, 2]),
                ([0, 2, 1, 1], [2, 0, 1, 1]),
                ([0, 2, 1, 2], [2, 0, 1, 2]),
                ([0, 2, 2, 1], [2, 0, 2, 1]),
                ([1, 0, 0, 2], [0, 0, 1, 2]),
                ([1, 0, 1, 2], [0, 2, 1, 1]),
                ([1, 0, 2, 0], [0, 0, 1, 2]),
                ([1, 0, 2, 1], [2, 0, 1, 1]),
                ([1, 0, 2, 2], [2, 0, 1, 2]),
                ([1, 1, 0, 2], [0, 2, 1, 1]),
                ([1, 1, 1, 2], [1, 2, 1, 1]),
                ([1, 1, 2, 0], [2, 0, 1, 1]),
                ([1, 1, 2, 1], [2, 1, 1, 1]),
                ([1, 1, 2, 2], [2, 2, 1, 1]),
                ([1, 2, 0, 0], [0, 0, 1, 2]),
                ([1, 2, 0, 1], [0, 2, 1, 1]),
                ([1, 2, 0, 2], [0, 2, 1, 2]),
                ([1, 2, 1, 0], [0, 2, 1, 1]),
                ([1, 2, 1, 2], [2, 2, 1, 1]),
                ([1, 2, 2, 0], [0, 2, 1, 2]),
                ([1, 2, 2, 1], [2, 2, 1, 1]),
                ([1, 2, 2, 2], [2, 2, 1, 2]),
                ([2, 0, 0, 1], [0, 0, 2, 1]),
                ([2, 0, 1, 0], [0, 0, 1, 2]),
                ([2, 0, 1, 1], [0, 2, 1, 1]),
                ([2, 0, 1, 2], [0, 2, 1, 2]),
                ([2, 0, 2, 1], [0, 2, 2, 1]),
                ([2, 1, 0, 0], [0, 0, 2, 1]),
                ([2, 1, 0, 1], [2, 0, 1, 1]),
                ([2, 1, 0, 2], [2, 0, 2, 1]),
                ([2, 1, 1, 0], [2, 0, 1, 1]),
                ([2, 1, 1, 2], [2, 2, 1, 1]),
                ([2, 1, 2, 0], [2, 0, 2, 1]),
                ([2, 1, 2, 1], [2, 2, 1, 1]),
                ([2, 1, 2, 2], [2, 2, 2, 1]),
                ([2, 2, 0, 1], [0, 2, 2, 1]),
                ([2, 2, 1, 0], [2, 0, 1, 2]),
            ],
        };
        let rules = rule_config.compile();

        Self { rules }
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
