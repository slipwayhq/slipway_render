use std::path::PathBuf;

use serde_json::{Map, Value};

pub fn process_schema_path(schema_path: PathBuf, output_schema_path: PathBuf) {
    let json_schema_str = std::fs::read_to_string(schema_path).unwrap();

    let result = process_schema_string(&json_schema_str);

    let result_str = serde_json::to_string_pretty(&result).unwrap();
    std::fs::write(output_schema_path, result_str).unwrap();
}

pub fn process_schema_string(json_schema_str: &str) -> Value {
    let mut schema: Value = serde_json::from_str(json_schema_str).unwrap();
    process_schema_json(&mut schema);
    schema
}

fn process_schema_json(schema: &mut Value) {
    let definitions = schema["definitions"].clone();
    let host_config = schema.as_object_mut().unwrap();

    apply_defaults(host_config, &definitions);
}

fn apply_defaults(obj: &mut Map<String, Value>, defs: &Value) {
    if let Some(Value::String(ref_type)) = obj.get("$ref") {
        if ref_type.starts_with("#/definitions/") {
            if let Some(def_name) = ref_type.strip_prefix("#/definitions/") {
                if let Some(Value::Object(definition)) = defs.get(def_name) {
                    // Handle properties directly in the definition (outside of allOf)
                    if let Some(Value::Object(properties)) = definition.get("properties") {
                        let default_obj = create_default_obj_from_properties(properties, defs);

                        // Only insert the combined default if it’s not empty and doesn’t already exist
                        if !default_obj.is_empty() {
                            obj.entry("default".to_string())
                                .and_modify(|e| {
                                    // Merge the existing default with the new default.
                                    if let Value::Object(existing_defaults) = e {
                                        for (key, value) in default_obj.clone() {
                                            existing_defaults.entry(key).or_insert(value);
                                        }
                                    }
                                })
                                .or_insert(Value::Object(default_obj));
                        }
                    }
                }
            }
        }
    }

    // Recursively apply defaults to nested objects
    for (_key, value) in obj.iter_mut() {
        if let Value::Object(ref mut map) = value {
            apply_defaults(map, defs);
        }
        if let Value::Array(ref mut arr) = value {
            recurse_on_array(arr, defs);
        }
    }
}

fn create_default_obj_from_properties(
    properties: &Map<String, Value>,
    defs: &Value,
) -> Map<String, Value> {
    let mut default_obj = Map::new();
    for (prop_key, prop_val) in properties {
        if let Some(Value::String(sub_ref)) = prop_val.get("$ref") {
            // If there is a $ref then we apply_defaults even if a "default" is present
            // at the same level in the tree, because "apply_defaults" will merge them.
            let mut sub_obj = serde_json::Map::new();
            sub_obj.insert("$ref".to_string(), Value::String(sub_ref.clone()));

            // Include any existing defaults in our sub-object so they can be merged.
            if let Some(default_val) = prop_val.get("default") {
                sub_obj.insert("default".to_string(), default_val.clone());
            }

            apply_defaults(&mut sub_obj, defs);
            if let Some(Value::Object(inner_default)) = sub_obj.get("default") {
                default_obj.insert(prop_key.to_string(), Value::Object(inner_default.clone()));
            }
        } else if let Some(default_val) = prop_val.get("default") {
            // If there is no $ref we can just use the default value as-is.
            default_obj.insert(prop_key.to_string(), default_val.clone());
        } else if let Some(Value::Array(all_of)) = prop_val.get("allOf") {
            // Handle allOf, combining defaults from each part
            let mut combined_defaults = Map::new();
            for schema_part in all_of {
                // Merge properties from $ref in allOf
                if let Some(Value::String(inner_ref)) = schema_part.get("$ref") {
                    let mut sub_obj = serde_json::Map::new();
                    sub_obj.insert("$ref".to_string(), Value::String(inner_ref.clone()));
                    apply_defaults(&mut sub_obj, defs);
                    if let Some(Value::Object(inner_default)) = sub_obj.get("default") {
                        for (key, val) in inner_default {
                            combined_defaults.insert(key.to_string(), val.clone());
                        }
                    }
                }

                // Merge direct properties in allOf
                if let Some(Value::Object(properties)) = schema_part.get("properties") {
                    for (prop_key, prop_val) in properties {
                        if let Some(default_val) = prop_val.get("default") {
                            combined_defaults.insert(prop_key.to_string(), default_val.clone());
                        }
                    }
                }
            }
            default_obj.insert(prop_key.to_string(), Value::Object(combined_defaults));
        } else if let Some(Value::Object(inner_properties)) = prop_val.get("properties") {
            let inner_default = create_default_obj_from_properties(inner_properties, defs);
            default_obj.insert(prop_key.to_string(), Value::Object(inner_default.clone()));
        }
    }

    default_obj
}

fn recurse_on_array(arr: &mut [Value], defs: &Value) {
    for item in arr {
        if let Value::Object(ref mut map) = item {
            apply_defaults(map, defs);
        }
        if let Value::Array(ref mut arr) = item {
            recurse_on_array(arr, defs);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_add_defaults_to_schema() {
        let schema_str = r##"
        {
	"definitions": {
		"AdaptiveCardConfig": {
			"type": "object",
			"description": "Toplevel options for `AdaptiveCards`",
			"additionalProperties": false,
			"properties": {
				"allowCustomStyle": {
					"type": "boolean",
					"description": "Controls whether custom styling is allowed",
					"default": true
				}
			}
		},
		"TextStyleConfig": {
			"type": "object",
			"description": "Sets default properties for text of a given style",
			"additionalProperties": false,
			"properties": {
				"size": {
					"type": "string",
					"description": "Default font size for text of this style",
					"enum": [
						"default",
						"small",
						"medium",
						"large",
						"extraLarge"
					],
					"default": "default"
				},
				"weight": {
					"type": "string",
					"description": "Default font weight for text of this style",
					"enum": [
						"normal",
						"lighter",
						"bolder"
					],
					"default": "normal"
				},
				"color": {
					"type": "string",
					"description": "Default font color for text of this style",
					"enum": [
						"default",
						"dark",
						"light",
						"accent",
						"good",
						"warning",
						"attention"
					],
					"default": "default"
				},
				"fontType": {
					"type": "string",
					"description": "Default font type for text of this style",
					"enum": [
						"default",
						"monospace"
					],
					"default": "default"
				},
				"isSubtle": {
					"type": "boolean",
					"description": "Whether text of this style should be subtle by default",
					"default": false
				}
			}
		},
		"TextStylesConfig": {
			"type": "object",
			"description": "Sets default properties for text of a given style",
			"additionalProperties": false,
			"properties": {
				"heading": {
					"$ref": "#/definitions/TextStyleConfig",
					"default": {
						"weight": "bolder",
						"size": "large",
						"color": "default",
						"fontType": "default",
						"isSubtle": false
					}
				},
				"columnHeader": {
					"$ref": "#/definitions/TextStyleConfig"
				}
			}
		},
		"TextBlockConfig": {
			"type": "object",
			"description": "Configuration settings for TextBlocks",
			"additionalProperties": false,
			"properties": {
				"headingLevel": {
					"type": "integer",
					"description": "When displaying a `TextBlock` element with the `heading` style, this is the heading level exposed to accessibility tools.",
					"default": 2
				}
			}
		},
		"FontTypeConfig": {
			"type": "object",
			"description": "Controls font styles",
			"properties": {
				"fontSizes": {
					"type": "object",
					"additionalProperties": false,
					"properties": {
						"small": {
							"type": "integer",
							"default": 12
						},
						"default": {
							"type": "integer",
							"default": 14
						},
						"medium": {
							"type": "integer",
							"default": 17
						},
						"large": {
							"type": "integer",
							"default": 21
						},
						"extraLarge": {
							"type": "integer",
							"default": 26
						}
					}
				},
				"fontWeights": {
					"type": "object",
					"additionalProperties": false,
					"properties": {
						"lighter": {
							"type": "integer",
							"default": 200
						},
						"default": {
							"type": "integer",
							"default": 400
						},
						"bolder": {
							"type": "integer",
							"default": 600
						}
					}
				}
			}
		},
		"FontTypesConfig": {
			"type": "object",
			"description": "Controls font styles",
			"additionalProperties": false,
			"properties": {
				"default": {
					"description": "Default font type",
					"unevaluatedProperties": false,
					"allOf": [
						{
							"$ref": "#/definitions/FontTypeConfig"
						},
						{
							"properties": {
								"fontFamily": {
									"type": "string",
									"default": "sans-serif"
								}
							}
						}
					]
				},
				"monospace": {
					"description": "Monospace font type",
					"unevaluatedProperties": false,
					"allOf": [
						{
							"$ref": "#/definitions/FontTypeConfig"
						},
						{
							"properties": {
								"fontFamily": {
									"type": "string",
									"default": "monospace"
								}
							}
						}
					]
				}
			}
		},
		"HostConfig": {
			"type": "object",
			"description": "Contains host-configurable settings",
			"additionalProperties": false,
			"properties": {
				"supportsInteractivity": {
					"type": "boolean",
					"description": "Control whether interactive `Action`s are allowed to be invoked",
					"default": true
				},
				"imageBaseUrl": {
					"type": "string",
					"format": "uri",
					"description": "Base URL to be used when loading resources"
				},
				"fontFamily": {
					"type": "string",
					"description": "Font face to use when rendering text",
					"default": "Calibri"
				},
				"choiceSetInputValueSeparator": {
					"type": "string",
					"description": "Separator to use when displaying multiple values for a `ChoiceSet`",
					"default": ", "
				},
				"textBlock": {
					"$ref": "#/definitions/TextBlockConfig"
				},
				"textStyles": {
					"$ref": "#/definitions/TextStylesConfig"
				},
				"fontTypes": {
					"$ref": "#/definitions/FontTypesConfig"
				},
				"$schema": {
					"type": "string",
					"description": "The Host Config schema."
				}
			}
		}
	},
	"$ref": "#/definitions/HostConfig"
}
    "##;

        let mut schema: Value = serde_json::from_str(schema_str).unwrap();

        process_schema_json(&mut schema);

        insta::assert_json_snapshot!(schema, @r###"
        {
          "$ref": "#/definitions/HostConfig",
          "default": {
            "choiceSetInputValueSeparator": ", ",
            "fontFamily": "Calibri",
            "fontTypes": {
              "default": {
                "fontFamily": "sans-serif",
                "fontSizes": {
                  "default": 14,
                  "extraLarge": 26,
                  "large": 21,
                  "medium": 17,
                  "small": 12
                },
                "fontWeights": {
                  "bolder": 600,
                  "default": 400,
                  "lighter": 200
                }
              },
              "monospace": {
                "fontFamily": "monospace",
                "fontSizes": {
                  "default": 14,
                  "extraLarge": 26,
                  "large": 21,
                  "medium": 17,
                  "small": 12
                },
                "fontWeights": {
                  "bolder": 600,
                  "default": 400,
                  "lighter": 200
                }
              }
            },
            "supportsInteractivity": true,
            "textBlock": {
              "headingLevel": 2
            },
            "textStyles": {
              "columnHeader": {
                "color": "default",
                "fontType": "default",
                "isSubtle": false,
                "size": "default",
                "weight": "normal"
              },
              "heading": {
                "color": "default",
                "fontType": "default",
                "isSubtle": false,
                "size": "large",
                "weight": "bolder"
              }
            }
          },
          "definitions": {
            "AdaptiveCardConfig": {
              "additionalProperties": false,
              "description": "Toplevel options for `AdaptiveCards`",
              "properties": {
                "allowCustomStyle": {
                  "default": true,
                  "description": "Controls whether custom styling is allowed",
                  "type": "boolean"
                }
              },
              "type": "object"
            },
            "FontTypeConfig": {
              "description": "Controls font styles",
              "properties": {
                "fontSizes": {
                  "additionalProperties": false,
                  "properties": {
                    "default": {
                      "default": 14,
                      "type": "integer"
                    },
                    "extraLarge": {
                      "default": 26,
                      "type": "integer"
                    },
                    "large": {
                      "default": 21,
                      "type": "integer"
                    },
                    "medium": {
                      "default": 17,
                      "type": "integer"
                    },
                    "small": {
                      "default": 12,
                      "type": "integer"
                    }
                  },
                  "type": "object"
                },
                "fontWeights": {
                  "additionalProperties": false,
                  "properties": {
                    "bolder": {
                      "default": 600,
                      "type": "integer"
                    },
                    "default": {
                      "default": 400,
                      "type": "integer"
                    },
                    "lighter": {
                      "default": 200,
                      "type": "integer"
                    }
                  },
                  "type": "object"
                }
              },
              "type": "object"
            },
            "FontTypesConfig": {
              "additionalProperties": false,
              "description": "Controls font styles",
              "properties": {
                "default": {
                  "allOf": [
                    {
                      "$ref": "#/definitions/FontTypeConfig",
                      "default": {
                        "fontSizes": {
                          "default": 14,
                          "extraLarge": 26,
                          "large": 21,
                          "medium": 17,
                          "small": 12
                        },
                        "fontWeights": {
                          "bolder": 600,
                          "default": 400,
                          "lighter": 200
                        }
                      }
                    },
                    {
                      "properties": {
                        "fontFamily": {
                          "default": "sans-serif",
                          "type": "string"
                        }
                      }
                    }
                  ],
                  "description": "Default font type",
                  "unevaluatedProperties": false
                },
                "monospace": {
                  "allOf": [
                    {
                      "$ref": "#/definitions/FontTypeConfig",
                      "default": {
                        "fontSizes": {
                          "default": 14,
                          "extraLarge": 26,
                          "large": 21,
                          "medium": 17,
                          "small": 12
                        },
                        "fontWeights": {
                          "bolder": 600,
                          "default": 400,
                          "lighter": 200
                        }
                      }
                    },
                    {
                      "properties": {
                        "fontFamily": {
                          "default": "monospace",
                          "type": "string"
                        }
                      }
                    }
                  ],
                  "description": "Monospace font type",
                  "unevaluatedProperties": false
                }
              },
              "type": "object"
            },
            "HostConfig": {
              "additionalProperties": false,
              "description": "Contains host-configurable settings",
              "properties": {
                "$schema": {
                  "description": "The Host Config schema.",
                  "type": "string"
                },
                "choiceSetInputValueSeparator": {
                  "default": ", ",
                  "description": "Separator to use when displaying multiple values for a `ChoiceSet`",
                  "type": "string"
                },
                "fontFamily": {
                  "default": "Calibri",
                  "description": "Font face to use when rendering text",
                  "type": "string"
                },
                "fontTypes": {
                  "$ref": "#/definitions/FontTypesConfig",
                  "default": {
                    "default": {
                      "fontFamily": "sans-serif",
                      "fontSizes": {
                        "default": 14,
                        "extraLarge": 26,
                        "large": 21,
                        "medium": 17,
                        "small": 12
                      },
                      "fontWeights": {
                        "bolder": 600,
                        "default": 400,
                        "lighter": 200
                      }
                    },
                    "monospace": {
                      "fontFamily": "monospace",
                      "fontSizes": {
                        "default": 14,
                        "extraLarge": 26,
                        "large": 21,
                        "medium": 17,
                        "small": 12
                      },
                      "fontWeights": {
                        "bolder": 600,
                        "default": 400,
                        "lighter": 200
                      }
                    }
                  }
                },
                "imageBaseUrl": {
                  "description": "Base URL to be used when loading resources",
                  "format": "uri",
                  "type": "string"
                },
                "supportsInteractivity": {
                  "default": true,
                  "description": "Control whether interactive `Action`s are allowed to be invoked",
                  "type": "boolean"
                },
                "textBlock": {
                  "$ref": "#/definitions/TextBlockConfig",
                  "default": {
                    "headingLevel": 2
                  }
                },
                "textStyles": {
                  "$ref": "#/definitions/TextStylesConfig",
                  "default": {
                    "columnHeader": {
                      "color": "default",
                      "fontType": "default",
                      "isSubtle": false,
                      "size": "default",
                      "weight": "normal"
                    },
                    "heading": {
                      "color": "default",
                      "fontType": "default",
                      "isSubtle": false,
                      "size": "large",
                      "weight": "bolder"
                    }
                  }
                }
              },
              "type": "object"
            },
            "TextBlockConfig": {
              "additionalProperties": false,
              "description": "Configuration settings for TextBlocks",
              "properties": {
                "headingLevel": {
                  "default": 2,
                  "description": "When displaying a `TextBlock` element with the `heading` style, this is the heading level exposed to accessibility tools.",
                  "type": "integer"
                }
              },
              "type": "object"
            },
            "TextStyleConfig": {
              "additionalProperties": false,
              "description": "Sets default properties for text of a given style",
              "properties": {
                "color": {
                  "default": "default",
                  "description": "Default font color for text of this style",
                  "enum": [
                    "default",
                    "dark",
                    "light",
                    "accent",
                    "good",
                    "warning",
                    "attention"
                  ],
                  "type": "string"
                },
                "fontType": {
                  "default": "default",
                  "description": "Default font type for text of this style",
                  "enum": [
                    "default",
                    "monospace"
                  ],
                  "type": "string"
                },
                "isSubtle": {
                  "default": false,
                  "description": "Whether text of this style should be subtle by default",
                  "type": "boolean"
                },
                "size": {
                  "default": "default",
                  "description": "Default font size for text of this style",
                  "enum": [
                    "default",
                    "small",
                    "medium",
                    "large",
                    "extraLarge"
                  ],
                  "type": "string"
                },
                "weight": {
                  "default": "normal",
                  "description": "Default font weight for text of this style",
                  "enum": [
                    "normal",
                    "lighter",
                    "bolder"
                  ],
                  "type": "string"
                }
              },
              "type": "object"
            },
            "TextStylesConfig": {
              "additionalProperties": false,
              "description": "Sets default properties for text of a given style",
              "properties": {
                "columnHeader": {
                  "$ref": "#/definitions/TextStyleConfig",
                  "default": {
                    "color": "default",
                    "fontType": "default",
                    "isSubtle": false,
                    "size": "default",
                    "weight": "normal"
                  }
                },
                "heading": {
                  "$ref": "#/definitions/TextStyleConfig",
                  "default": {
                    "color": "default",
                    "fontType": "default",
                    "isSubtle": false,
                    "size": "large",
                    "weight": "bolder"
                  }
                }
              },
              "type": "object"
            }
          }
        }
        "###);
    }
}
