use std::collections::{HashMap, HashSet};

use crate::{load::Loaded, typed_schema_types::Class};

pub(super) fn flatten_class_map<'c>(
    map: &HashMap<String, HashSet<&'c Loaded<Class>>>,
) -> HashMap<String, Vec<&'c Loaded<Class>>> {
    let mut result = HashMap::new();
    for id in map.keys() {
        let flattened = flatten_map_for(id, map);
        result.insert(id.clone(), flattened);
    }
    result
}

fn flatten_map_for<'c>(
    class_id: &str,
    map: &HashMap<String, HashSet<&'c Loaded<Class>>>,
) -> Vec<&'c Loaded<Class>> {
    let mut result = Vec::new();
    flatten_map_for_inner(class_id, map, &mut result);
    result
}

fn flatten_map_for_inner<'c>(
    class_id: &str,
    map: &HashMap<String, HashSet<&'c Loaded<Class>>>,
    result: &mut Vec<&'c Loaded<Class>>,
) {
    if let Some(parents) = map.get(class_id) {
        for parent in parents {
            result.push(parent);
            flatten_map_for_inner(parent.id.as_str(), map, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn it_should_flatten_map_ordering_vectors_by_distance_from_class() {
        // b  interface
        // | /
        // c
        // |
        // a
        // |
        // d

        let b = Loaded {
            value: Class::builder().try_into().unwrap(),
            file_name: "b.json".to_string(),
            type_name: "B".to_string(),
            id: "b".to_string(),
        };
        let c = Loaded {
            value: Class::builder()
                .extends(Some("b, interface".to_string()))
                .try_into()
                .unwrap(),
            file_name: "c.json".to_string(),
            type_name: "C".to_string(),
            id: "c".to_string(),
        };
        let a = Loaded {
            value: Class::builder()
                .extends(Some("c".to_string()))
                .try_into()
                .unwrap(),
            file_name: "a.json".to_string(),
            type_name: "A".to_string(),
            id: "a".to_string(),
        };
        let d = Loaded {
            value: Class::builder()
                .extends(Some("a".to_string()))
                .try_into()
                .unwrap(),
            file_name: "d.json".to_string(),
            type_name: "D".to_string(),
            id: "d".to_string(),
        };

        let interface = Loaded::<Class> {
            value: Class::builder()
                .extends(None)
                .is_abstract(Some(true))
                .try_into()
                .unwrap(),
            file_name: "interface.json".to_string(),
            type_name: "Interface".to_string(),
            id: "interface".to_string(),
        };

        let classes_by_id = vec![a, b, c, d, interface]
            .into_iter()
            .map(|c| (c.id.clone(), c))
            .collect::<HashMap<_, _>>();

        let class_parents = super::super::get_class_parents_map(&classes_by_id);
        let class_children = super::super::get_class_children_map(&class_parents, &classes_by_id);

        let flattened_parents = flatten_class_map(&class_parents);
        let flattened_children = flatten_class_map(&class_children);

        assert_eq!(
            flattened_children
                .get("b")
                .unwrap()
                .iter()
                .map(|c| c.id.clone())
                .collect::<Vec<_>>(),
            vec!["c", "a", "d"]
        );

        assert_eq!(
            flattened_parents
                .get("d")
                .unwrap()
                .iter()
                .map(|c| c.id.clone())
                .collect::<Vec<_>>(),
            vec!["a", "c", "b", "interface"]
        );
    }
}
