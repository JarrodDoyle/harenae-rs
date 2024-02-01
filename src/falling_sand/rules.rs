use bevy::{ecs::system::Resource, utils::HashMap};

use super::{
    element::{update_sand, update_water},
    Element,
};

pub struct RuleBuilder {
    elements: [Element; 4],
    processed: [bool; 4],
    position: i8,
}

impl RuleBuilder {
    pub fn build(input: (Element, Element, Element, Element)) -> u32 {
        let mut block = RuleBuilder {
            elements: [input.0, input.1, input.2, input.3],
            processed: [false; 4],
            position: 0,
        };

        for i in 0..4 {
            if block.processed[i] {
                continue;
            }

            block.position = i as i8;
            let element = block.elements[i];
            match element {
                Element::Sand => update_sand(&mut block, element),
                Element::Water => update_water(&mut block, element),
                _ => {}
            }
        }

        to_rule_state((
            block.elements[0],
            block.elements[1],
            block.elements[2],
            block.elements[3],
        ))
    }

    pub fn get(&self, x: i8, y: i8) -> Element {
        let x = x + (self.position % 2);
        let y = (self.position / 2) - y;
        if !(0..2).contains(&x) || !(0..2).contains(&y) {
            return Element::None;
        }

        let idx = (x + y * 2) as usize;
        self.elements[idx]
    }

    pub fn set(&mut self, x: i8, y: i8, element: Element) {
        let x = x + (self.position % 2);
        let y = (self.position / 2) - y;
        if (0..2).contains(&x) && (0..2).contains(&y) {
            let idx = (x + y * 2) as usize;
            self.elements[idx] = element;
            self.processed[idx] = true;
        }
    }
}

#[derive(Resource)]
pub struct FallingSandRules {
    rules: HashMap<u32, u32>,
}

impl Default for FallingSandRules {
    fn default() -> Self {
        // Build a list of elements
        // We do it this way so it automatically handles adding new elements
        let mut elements = vec![];
        for i in 0..(Element::ElementCount as u32) {
            elements.push(Element::from(i));
        }

        // Attempt to compute a rule for every possible element block permutation
        // Only bother keeping the rule if the state actually changes
        // TODO: See if there's a better way to build the permutations than nesting loops
        let mut rules = HashMap::new();
        for a in 0..elements.len() {
            for b in 0..elements.len() {
                for c in 0..elements.len() {
                    for d in 0..elements.len() {
                        let input = (elements[a], elements[b], elements[c], elements[d]);
                        let in_rule = to_rule_state(input);
                        let out_rule = RuleBuilder::build(input);
                        if in_rule != out_rule {
                            rules.insert(in_rule, out_rule);
                        }
                    }
                }
            }
        }

        Self { rules }
    }
}

impl FallingSandRules {
    pub fn get_result(
        &self,
        input: (Element, Element, Element, Element),
    ) -> (Element, Element, Element, Element) {
        let input_rule = to_rule_state(input);
        let output_rule = match self.rules.get(&input_rule) {
            Some(&result) => result,
            None => input_rule,
        };
        from_rule_state(output_rule)
    }
}

fn to_rule_state(input: (Element, Element, Element, Element)) -> u32 {
    ((input.0 as u32) << 24) + ((input.1 as u32) << 16) + ((input.2 as u32) << 8) + input.3 as u32
}

fn from_rule_state(input: u32) -> (Element, Element, Element, Element) {
    (
        Element::from((input >> 24) & 0xFF),
        Element::from((input >> 16) & 0xFF),
        Element::from((input >> 8) & 0xFF),
        Element::from(input & 0xFF),
    )
}
