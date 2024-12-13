use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{load::Loaded, typed_schema_types::Class};

const IMAGE_TYPE: &str = "Image";
const LAYOUTABLE_TYPES: [&str; 7] = [
    "Item",
    "AdaptiveCard",
    // We include this because it is an Action but does not extend Item.
    "ISelectAction",
    "TableCell",
    "TableRow",
    "Inline",
    // This contains an action, and so requires the TLayoutType generic parameter.
    "Refresh",
];
const ELEMENT_TYPE: &str = "Element";
const TOGGLEABLE_ITEM_TYPE: &str = "ToggleableItem";
const STACKABLE_TYPES: [&str; 2] = ["Element", "Column"];

pub(super) struct Relationships<'c> {
    pub classes: &'c HashMap<String, Loaded<Class>>,
    pub ancestors: HashMap<String, Vec<&'c Loaded<Class>>>,
    pub descendants: HashMap<String, Vec<&'c Loaded<Class>>>,
    pub metadata: HashMap<String, ClassMetadata>,
    pub type_name_to_id: HashMap<String, String>,
}

pub(super) struct ClassRelationships<'c> {
    pub ancestors: Vec<&'c Loaded<Class>>,
    pub descendants: Vec<&'c Loaded<Class>>,
    pub metadata: ClassMetadata,
}

#[derive(Debug, Copy, Clone)]
pub(super) struct ClassMetadata {
    pub is_abstract: bool,

    // Private because we should use the Relationships::is_layoutable method.
    is_layoutable: bool,

    pub is_image: bool,
    pub is_element: bool,
    pub is_toggleable: bool,
    pub is_stackable: bool,
}

impl<'c> Relationships<'c> {
    pub fn get_class_data(&self, id: &str) -> ClassRelationships<'c> {
        let ancestors = self
            .ancestors
            .get(id)
            .unwrap_or_else(|| panic!("No ancestors for {}", id));

        let descendants = self
            .descendants
            .get(id)
            .unwrap_or_else(|| panic!("No descendants for {}", id));

        let metadata = *self
            .metadata
            .get(id)
            .unwrap_or_else(|| panic!("No metadata for {}", id));

        ClassRelationships {
            ancestors: ancestors.clone(),
            descendants: descendants.clone(),
            metadata,
        }
    }

    pub fn is_layoutable(&self, id: &str) -> bool {
        self.metadata
            .get(id)
            .map(|m| m.is_layoutable)
            .unwrap_or_else(|| {
                // Try as if the ID passed in was a type name.
                // e.g. map ActionExecute to Action.Execute
                self.type_name_to_id
                    .get(id)
                    .map(|id| {
                        self.metadata
                            .get(id)
                            .map(|m| m.is_layoutable)
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
    }
}

pub(super) fn get_relationships(classes: &HashMap<String, Loaded<Class>>) -> Relationships<'_> {
    let parents = get_class_parents_map(classes);
    let children = get_class_children_map(&parents, classes);

    let ancestors = flatten_class_map(&parents);
    let descendants = flatten_class_map(&children);

    let metadata = classes
        .iter()
        .map(|(id, class)| {
            let ancestors = ancestors
                .get(id)
                .unwrap_or_else(|| panic!("No ancestors for {}", class.id));

            let is_abstract = class.value.is_abstract.unwrap_or(false);

            // Does this type need to have layout data added.
            let is_layoutable = LAYOUTABLE_TYPES.contains(&class.type_name.as_str())
                || ancestors
                    .iter()
                    .any(|a| LAYOUTABLE_TYPES.contains(&a.type_name.as_str()));

            let is_image = IMAGE_TYPE == class.type_name
                || ancestors.iter().any(|a| IMAGE_TYPE == a.type_name);

            let is_element = ELEMENT_TYPE == class.type_name
                || ancestors.iter().any(|a| ELEMENT_TYPE == a.type_name);

            let is_toggleable = TOGGLEABLE_ITEM_TYPE == class.type_name
                || ancestors
                    .iter()
                    .any(|a| TOGGLEABLE_ITEM_TYPE == a.type_name);

            let is_stackable = STACKABLE_TYPES.contains(&class.type_name.as_str())
                || ancestors
                    .iter()
                    .any(|a| STACKABLE_TYPES.contains(&a.type_name.as_str()));

            (
                id.clone(),
                ClassMetadata {
                    is_abstract,
                    is_layoutable,
                    is_image,
                    is_element,
                    is_toggleable,
                    is_stackable,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let type_name_to_id = classes
        .iter()
        .map(|(id, class)| (class.type_name.clone(), id.clone()))
        .collect::<HashMap<_, _>>();

    Relationships {
        classes,
        ancestors,
        descendants,
        metadata,
        type_name_to_id,
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
