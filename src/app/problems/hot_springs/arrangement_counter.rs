use std::cell::RefCell;
use std::collections::HashMap;
use super::spring_condition::SpringCondition;

pub struct ArrangementCounter {
    count_cache: RefCell<HashMap<Vec<Option<SpringCondition>>, HashMap<Vec<usize>, usize>>>,
}

impl ArrangementCounter {
    pub fn new() -> Self {
        Self {
            count_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn count_arrangements(&self, spring_conditions: &[Option<SpringCondition>], damaged_group_sizes: &[usize]) -> usize {
        if let Some(map) = self.count_cache.borrow().get(spring_conditions) {
            if let Some(matches) = map.get(damaged_group_sizes) {
                return *matches;
            }
        }

        let result = if spring_conditions.is_empty() {
            if damaged_group_sizes.is_empty() {
                1
            } else {
                0
            }
        } else {
            match spring_conditions[0] {
                Some(SpringCondition::Operational) => self.count_arrangements(&spring_conditions[1..], &damaged_group_sizes),
                spring_condition => {
                    let none_result = if spring_condition == None {
                        self.count_arrangements(&spring_conditions[1..], &damaged_group_sizes)
                    } else {
                        0
                    };

                    let damaged_result = if damaged_group_sizes.is_empty() {
                        0
                    } else {
                        let damaged_group_size = damaged_group_sizes[0];
                        let checked_length = if spring_conditions.len() > damaged_group_size {
                            damaged_group_size + 1
                        } else {
                            damaged_group_size
                        };

                        if spring_conditions.len() >= damaged_group_size &&
                            (1..damaged_group_size).all(|i| spring_conditions[i] != Some(SpringCondition::Operational)) &&
                            (spring_conditions.len() == damaged_group_size || spring_conditions[damaged_group_size] != Some(SpringCondition::Damaged)) {
                            self.count_arrangements(&spring_conditions[checked_length..], &damaged_group_sizes[1..])
                        } else {
                            0
                        }
                    };

                    none_result + damaged_result
                }
            }
        };

        let mut count_cache = self.count_cache.borrow_mut();
        if let Some(map) = count_cache.get_mut(spring_conditions) {
            map.insert(damaged_group_sizes.to_vec(), result);
        } else {
            let mut map = HashMap::new();
            map.insert(damaged_group_sizes.to_vec(), result);
            count_cache.insert(spring_conditions.to_vec(), map);
        }

        result
    }
}
