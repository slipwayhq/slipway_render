use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{load::Loaded, typed_schema_types::Class};

pub(super) struct Relationships<'c> {
    pub classes: &'c HashMap<String, Loaded<Class>>,
    pub ancestors: HashMap<String, Vec<&'c Loaded<Class>>>,
    pub descendants: HashMap<String, Vec<&'c Loaded<Class>>>,
}

pub(super) fn get_relationships(classes: &HashMap<String, Loaded<Class>>) -> Relationships<'_> {
    let parents = get_class_parents_map(classes);
    let children = get_class_children_map(&parents, classes);

    let ancestors = flatten_class_map(&parents);
    let descendants = flatten_class_map(&children);

    Relationships {
        classes,
        ancestors,
        descendants,
    }
}

fn get_class_parents_map(
    classes_by_id: &HashMap<String, Loaded<Class>>,
) -> HashMap<String, HashSet<&Loaded<Class>>> {
    let mut class_inherits_from = HashMap::new();
    for (id, loaded_class) in classes_by_id.iter() {
        // Ensure that every class ID is in the map.
        let entry = class_inherits_from
            .entry(id.clone())
            .or_insert_with(HashSet::new);
        if let Some(extends_str) = loaded_class.value.extends.as_ref() {
            let extends_list = extends_str
                .split(',')
                .map(|s| s.trim())
                .collect::<HashSet<_>>();
            for extends in extends_list {
                let parent_class = get_or_panic(classes_by_id, extends);
                entry.insert(parent_class);
            }
        }
    }
    class_inherits_from
}

fn get_class_children_map<'c>(
    class_parents: &HashMap<String, HashSet<&'c Loaded<Class>>>,
    classes_by_id: &'c HashMap<String, Loaded<Class>>,
) -> HashMap<String, HashSet<&'c Loaded<Class>>> {
    let mut class_inheritors = HashMap::new();
    for (id, dependencies) in class_parents.iter() {
        // Ensure that every class ID is in the map.
        class_inheritors
            .entry(id.clone())
            .or_insert_with(HashSet::new);
        let parent_class = get_or_panic(classes_by_id, id);
        for dependency in dependencies {
            class_inheritors
                .entry(dependency.id.clone())
                .or_insert_with(HashSet::new)
                .insert(parent_class);
        }
    }
    class_inheritors
}

fn get_or_panic<'a>(
    classes_by_id: &'a HashMap<String, Loaded<Class>>,
    id: &str,
) -> &'a Loaded<Class> {
    let parent_class = classes_by_id
        .get(id)
        .unwrap_or_else(|| panic!("Failed to find class {}", id));
    parent_class
}

fn flatten_class_map<'c>(
    map: &HashMap<String, HashSet<&'c Loaded<Class>>>,
) -> HashMap<String, Vec<&'c Loaded<Class>>> {
    let mut result = HashMap::new();
    for id in map.keys() {
        let flattened = flatten_map_for(id, map);
        result.insert(id.clone(), flattened);
    }
    result
}

fn flatten_map_for<'m, 'c>(
    class_id: &'m str,
    map: &'m HashMap<String, HashSet<&'c Loaded<Class>>>,
) -> Vec<&'c Loaded<Class>> {
    let mut result = Vec::new();
    flatten_map_for_inner(class_id, map, &mut result);
    result
}

fn flatten_map_for_inner<'m, 'c>(
    class_id: &'m str,
    map: &'m HashMap<String, HashSet<&'c Loaded<Class>>>,
    result: &mut Vec<&'c Loaded<Class>>,
) {
    if let Some(parents) = map.get(class_id) {
        for parent in parents.iter().sorted_by_key(|p| p.id.as_str()) {
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

        fn extends(extends: &str) -> Class {
            Class::builder()
                .extends(Some(extends.to_string()))
                .try_into()
                .unwrap()
        }

        let b = Loaded {
            value: Class::builder().try_into().unwrap(),
            file_name: "b.json".to_string(),
            type_name: "B".to_string(),
            id: "b".to_string(),
        };
        let c = Loaded {
            value: extends("b, interface"),
            file_name: "c.json".to_string(),
            type_name: "C".to_string(),
            id: "c".to_string(),
        };
        let a = Loaded {
            value: extends("c"),
            file_name: "a.json".to_string(),
            type_name: "A".to_string(),
            id: "a".to_string(),
        };
        let d = Loaded {
            value: extends("a"),
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

        let relationships = get_relationships(&classes_by_id);

        assert_eq!(
            relationships
                .descendants
                .get("b")
                .unwrap()
                .iter()
                .map(|c| c.id.clone())
                .collect::<Vec<_>>(),
            vec!["c", "a", "d"]
        );

        assert_eq!(
            relationships
                .ancestors
                .get("d")
                .unwrap()
                .iter()
                .map(|c| c.id.clone())
                .collect::<Vec<_>>(),
            vec!["a", "c", "b", "interface"]
        );

        assert_eq!(
            relationships
                .descendants
                .get("interface")
                .unwrap()
                .iter()
                .map(|c| c.id.clone())
                .collect::<Vec<_>>(),
            vec!["c", "a", "d"]
        );

        assert!(relationships
            .ancestors
            .get("interface")
            .unwrap()
            .iter()
            .map(|c| c.id.clone())
            .collect::<Vec<_>>()
            .is_empty());
    }
}
