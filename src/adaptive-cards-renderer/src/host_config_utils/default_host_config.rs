use adaptive_cards_host_config::HostConfig;

pub fn default_host_config() -> HostConfig {
    adaptive_cards_host_config::builder::HostConfig::default()
        .try_into()
        .expect("Default host config should be valid")
    // serde_json::from_str::<HostConfig>(DEFAULT_TEAMS_LIGHT_HOST_CONFIG_JSON).unwrap()
}

const DEFAULT_TEAMS_LIGHT_HOST_CONFIG_JSON: &str = r##"{
    "choiceSetInputValueSeparator": ",",
    "supportsInteractivity": true,
    "spacing": {
        "small": 8,
        "default": 12,
        "medium": 16,
        "large": 20,
        "extraLarge": 24,
        "padding": 16
    },
    "separator": {
        "lineThickness": 1,
        "lineColor": "#EEEEEE"
    },
    "imageSizes": {
        "small": 32,
        "medium": 52,
        "large": 100
    },
    "fontTypes": {
        "default": {
            "fontFamily": "'Segoe UI', Tahoma, Geneva, Verdana, sans-serif",
            "fontSizes": {
                "small": 12,
                "default": 14,
                "medium": 14,
                "large": 18,
                "extraLarge": 24
            },
            "fontWeights": {
                "lighter": 200,
                "default": 400,
                "bolder": 600
            }
        },
        "monospace": {
            "fontFamily": "'Courier New', Courier, monospace",
            "fontSizes": {
                "small": 12,
                "default": 14,
                "medium": 14,
                "large": 18,
                "extraLarge": 24
            },
            "fontWeights": {
                "lighter": 200,
                "default": 400,
                "bolder": 600
            }
        }
    },
    "textStyles": {
        "heading": {
            "fontType": "default",
            "size": "large",
            "weight": "bolder",
            "color": "default",
            "isSubtle": false
        }
    },
    "textBlock": {
        "headingLevel": 2
    },
    "containerStyles": {
        "default": {
            "foregroundColors": {
                "default": {
                    "default": "#ff252424",
                    "subtle": "#bf252424"
                },
                "dark": {
                    "default": "#252424",
                    "subtle": "#bf252424"
                },
                "light": {
                    "default": "#ffffff",
                    "subtle": "#fff3f2f1"
                },
                "accent": {
                    "default": "#6264a7",
                    "subtle": "#8b8cc7"
                },
                "good": {
                    "default": "#92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#f8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#c4314b",
                    "subtle": "#e5c4314b"
                }
            },
            "borderColor": "#CCCCCC",
            "backgroundColor": "#ffffff"
        },
        "emphasis": {
            "foregroundColors": {
                "default": {
                    "default": "#ff252424",
                    "subtle": "#bf252424"
                },
                "dark": {
                    "default": "#252424",
                    "subtle": "#bf252424"
                },
                "light": {
                    "default": "#ffffff",
                    "subtle": "#fff3f2f1"
                },
                "accent": {
                    "default": "#6264a7",
                    "subtle": "#8b8cc7"
                },
                "good": {
                    "default": "#92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#f8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#c4314b",
                    "subtle": "#e5c4314b"
                }
            },
            "borderColor": "#666666",
            "backgroundColor": "#fff9f8f7"
        },
        "accent": {
            "borderColor": "#62A8F7",
            "backgroundColor": "#C7DEF9",
            "foregroundColors": {
                "default": {
                    "default": "#ff252424",
                    "subtle": "#bf252424"
                },
                "dark": {
                    "default": "#252424",
                    "subtle": "#bf252424"
                },
                "light": {
                    "default": "#ffffff",
                    "subtle": "#fff3f2f1"
                },
                "accent": {
                    "default": "#6264a7",
                    "subtle": "#8b8cc7"
                },
                "good": {
                    "default": "#92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#f8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#c4314b",
                    "subtle": "#e5c4314b"
                }
            }
        },
        "good": {
            "borderColor": "#69E569",
            "backgroundColor": "#CCFFCC",
            "foregroundColors": {
                "default": {
                    "default": "#ff252424",
                    "subtle": "#bf252424"
                },
                "dark": {
                    "default": "#252424",
                    "subtle": "#bf252424"
                },
                "light": {
                    "default": "#ffffff",
                    "subtle": "#fff3f2f1"
                },
                "accent": {
                    "default": "#6264a7",
                    "subtle": "#8b8cc7"
                },
                "good": {
                    "default": "#92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#f8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#c4314b",
                    "subtle": "#e5c4314b"
                }
            }
        },
        "attention": {
            "borderColor": "#FF764C",
            "backgroundColor": "#FFC5B2",
            "foregroundColors": {
                "default": {
                    "default": "#ff252424",
                    "subtle": "#bf252424"
                },
                "dark": {
                    "default": "#252424",
                    "subtle": "#bf252424"
                },
                "light": {
                    "default": "#ffffff",
                    "subtle": "#fff3f2f1"
                },
                "accent": {
                    "default": "#6264a7",
                    "subtle": "#8b8cc7"
                },
                "good": {
                    "default": "#92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#f8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#c4314b",
                    "subtle": "#e5c4314b"
                }
            }
        },
        "warning": {
            "borderColor": "#FFBC51",
            "backgroundColor": "#FFE2B2",
            "foregroundColors": {
                "default": {
                    "default": "#ff252424",
                    "subtle": "#bf252424"
                },
                "dark": {
                    "default": "#252424",
                    "subtle": "#bf252424"
                },
                "light": {
                    "default": "#ffffff",
                    "subtle": "#fff3f2f1"
                },
                "accent": {
                    "default": "#6264a7",
                    "subtle": "#8b8cc7"
                },
                "good": {
                    "default": "#92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#f8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#c4314b",
                    "subtle": "#e5c4314b"
                }
            }
        }
    },
    "actions": {
        "maxActions": 6,
        "spacing": "default",
        "buttonSpacing": 8,
        "showCard": {
            "actionMode": "inline",
            "inlineTopMargin": 16,
            "style": "emphasis"
        },
        "preExpandSingleShowCardAction": false,
        "actionsOrientation": "horizontal",
        "actionAlignment": "left"
    },
    "adaptiveCard": {
        "allowCustomStyle": false
    },
    "imageSet": {
        "imageSize": "medium",
        "maxImageHeight": 100
    },
    "factSet": {
        "title": {
            "size": "default",
            "color": "default",
            "isSubtle": false,
            "weight": "bolder",
            "wrap": true
        },
        "value": {
            "size": "default",
            "color": "default",
            "isSubtle": false,
            "weight": "normal",
            "wrap": true
        },
        "spacing": 16
    }
}"##;

const DEFAULT_TEAMS_DARK_HOST_CONFIG_JSON: &str = r##"{
    "$schema": "../../schema/host-config.schema.json",
    "choiceSetInputValueSeparator": ",",
    "supportsInteractivity": true,
    "spacing": {
        "small": 8,
        "default": 12,
        "medium": 16,
        "large": 20,
        "extraLarge": 24,
        "padding": 16
    },
    "separator": {
        "lineThickness": 1,
        "lineColor": "#EEEEEE"
    },
    "fontTypes": {
        "default": {
            "fontFamily": "'Segoe UI', Tahoma, Geneva, Verdana, sans-serif",
            "fontSizes": {
                "small": 12,
                "default": 14,
                "medium": 14,
                "large": 18,
                "extraLarge": 24
            },
            "fontWeights": {
                "lighter": 200,
                "default": 400,
                "bolder": 600
            }
        },
        "monospace": {
            "fontFamily": "'Courier New', Courier, monospace",
            "fontSizes": {
                "small": 12,
                "default": 14,
                "medium": 14,
                "large": 18,
                "extraLarge": 24
            },
            "fontWeights": {
                "lighter": 200,
                "default": 400,
                "bolder": 600
            }
        }
    },
    "textStyles": {
        "heading": {
            "fontType": "default",
            "size": "large",
            "weight": "bolder",
            "color": "default",
            "isSubtle": false
        }
    },
    "textBlock": {
        "headingLevel": 2
    },
    "imageSizes": {
        "small": 32,
        "medium": 52,
        "large": 100
    },
    "containerStyles": {
        "default": {
            "foregroundColors": {
                "default": {
                    "default": "#ffffffff",
                    "subtle": "#bfffffff"
                },
                "dark": {
                    "default": "#ff201f1f",
                    "subtle": "#ff2d2c2c"
                },
                "light": {
                    "default": "#ffffffff",
                    "subtle": "#bfffffff"
                },
                "accent": {
                    "default": "#ffa6a7dc",
                    "subtle": "#ff8b8cc7"
                },
                "good": {
                    "default": "#ff92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#fff8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#ffd74654",
                    "subtle": "#e5d74654"
                }
            },
            "borderColor": "#CCCCCC",
            "backgroundColor": "#ff2d2c2c"
        },
        "emphasis": {
            "foregroundColors": {
                "default": {
                    "default": "#ffffffff",
                    "subtle": "#bfffffff"
                },
                "dark": {
                    "default": "#ff201f1f",
                    "subtle": "#ff2d2c2c"
                },
                "light": {
                    "default": "#ffffffff",
                    "subtle": "#bfffffff"
                },
                "accent": {
                    "default": "#ffa6a7dc",
                    "subtle": "#ff8b8cc7"
                },
                "good": {
                    "default": "#ff92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#fff8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#ffd74654",
                    "subtle": "#e5d74654"
                }
            },
            "borderColor": "#666666",
            "backgroundColor": "#ff292828"
        },
        "accent": {
            "borderColor": "#62A8F7",
            "backgroundColor": "#C7DEF9",
            "foregroundColors": {
                "default": {
                    "default": "#ff201f1f",
                    "subtle": "#ff2d2c2c"
                },
                "dark": {
                    "default": "#ff201f1f",
                    "subtle": "#ff2d2c2c"
                },
                "light": {
                    "default": "#ffffffff",
                    "subtle": "#bfffffff"
                },
                "accent": {
                    "default": "#ffa6a7dc",
                    "subtle": "#ff8b8cc7"
                },
                "good": {
                    "default": "#ff92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#fff8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#ffd74654",
                    "subtle": "#e5d74654"
                }
            }
        },
        "good": {
            "borderColor": "#69E569",
            "backgroundColor": "#CCFFCC",
            "foregroundColors": {
                "default": {
                    "default": "#ff201f1f",
                    "subtle": "#ff2d2c2c"
                },
                "dark": {
                    "default": "#ff201f1f",
                    "subtle": "#ff2d2c2c"
                },
                "light": {
                    "default": "#ffffffff",
                    "subtle": "#bfffffff"
                },
                "accent": {
                    "default": "#ffa6a7dc",
                    "subtle": "#ff8b8cc7"
                },
                "good": {
                    "default": "#ff92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#fff8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#ffd74654",
                    "subtle": "#e5d74654"
                }
            }
        },
        "attention": {
            "borderColor": "#FF764C",
            "backgroundColor": "#FFC5B2",
            "foregroundColors": {
                "default": {
                    "default": "#ff201f1f",
                    "subtle": "#ff2d2c2c"
                },
                "dark": {
                    "default": "#ff201f1f",
                    "subtle": "#ff2d2c2c"
                },
                "light": {
                    "default": "#ffffffff",
                    "subtle": "#bfffffff"
                },
                "accent": {
                    "default": "#ffa6a7dc",
                    "subtle": "#ff8b8cc7"
                },
                "good": {
                    "default": "#ff92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#fff8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#ffd74654",
                    "subtle": "#e5d74654"
                }
            }
        },
        "warning": {
            "borderColor": "#FFBC51",
            "backgroundColor": "#FFE2B2",
            "foregroundColors": {
                "default": {
                    "default": "#ff201f1f",
                    "subtle": "#ff2d2c2c"
                },
                "dark": {
                    "default": "#ff201f1f",
                    "subtle": "#ff2d2c2c"
                },
                "light": {
                    "default": "#ffffffff",
                    "subtle": "#bfffffff"
                },
                "accent": {
                    "default": "#ffa6a7dc",
                    "subtle": "#ff8b8cc7"
                },
                "good": {
                    "default": "#ff92c353",
                    "subtle": "#e592c353"
                },
                "warning": {
                    "default": "#fff8d22a",
                    "subtle": "#e5f8d22a"
                },
                "attention": {
                    "default": "#ffd74654",
                    "subtle": "#e5d74654"
                }
            }
        }
    },
    "actions": {
        "maxActions": 6,
        "spacing": "default",
        "buttonSpacing": 8,
        "showCard": {
            "actionMode": "inline",
            "inlineTopMargin": 16,
            "style": "emphasis"
        },
        "preExpandSingleShowCardAction": false,
        "actionsOrientation": "horizontal",
        "actionAlignment": "left"
    },
    "adaptiveCard": {
        "allowCustomStyle": false
    },
    "imageSet": {
        "imageSize": "medium",
        "maxImageHeight": 100
    },
    "factSet": {
        "title": {
            "size": "default",
            "color": "default",
            "isSubtle": false,
            "weight": "bolder",
            "wrap": true
        },
        "value": {
            "size": "default",
            "color": "default",
            "isSubtle": false,
            "weight": "normal",
            "wrap": true
        },
        "spacing": 16
    }
}"##;

const DEFAULT_OUTLOOK_DESKTOP_HOST_CONFIG_JSON: &str = r##"{
    "supportsInteractivity": true,
    "spacing": {
        "small": 10,
        "default": 20,
        "medium": 30,
        "large": 40,
        "extraLarge": 50,
        "padding": 20
    },
    "separator": {
        "lineThickness": 1,
        "lineColor": "#EEEEEE"
    },
    "imageSizes": {
        "small": 40,
        "medium": 80,
        "large": 160
    },
    "fontTypes": {
        "default": {
            "fontFamily": "'Segoe UI', Tahoma, Geneva, Verdana, sans-serif",
            "fontSizes": {
                "small": 12,
                "default": 14,
                "medium": 17,
                "large": 21,
                "extraLarge": 26
            },
            "fontWeights": {
                "lighter": 200,
                "default": 400,
                "bolder": 600
            }
        },
        "monospace": {
            "fontFamily": "'Courier New', Courier, monospace",
            "fontSizes": {
                "small": 12,
                "default": 14,
                "medium": 17,
                "large": 21,
                "extraLarge": 26
            },
            "fontWeights": {
                "lighter": 200,
                "default": 400,
                "bolder": 600
            }
        }
    },
    "textStyles": {
        "heading": {
            "fontType": "default",
            "size": "large",
            "weight": "bolder",
            "color": "default",
            "isSubtle": false
        }
    },
    "textBlock": {
        "headingLevel": 2
    },
    "containerStyles": {
        "default": {
            "borderColor": "#CCCCCC",
            "backgroundColor": "#FFFFFF",
            "foregroundColors": {
                "default": {
                    "default": "#333333",
                    "subtle": "#EE333333"
                },
                "dark": {
                    "default": "#000000",
                    "subtle": "#66000000"
                },
                "light": {
                    "default": "#FFFFFF",
                    "subtle": "#33000000"
                },
                "accent": {
                    "default": "#2E89FC",
                    "subtle": "#882E89FC"
                },
                "attention": {
                    "default": "#cc3300",
                    "subtle": "#DDcc3300"
                },
                "good": {
                    "default": "#028A02",
                    "subtle": "#DD027502"
                },
                "warning": {
                    "default": "#e69500",
                    "subtle": "#DDe69500"
                }
            }
        },
        "emphasis": {
            "borderColor": "#666666",
            "backgroundColor": "#08000000",
            "foregroundColors": {
                "default": {
                    "default": "#333333",
                    "subtle": "#EE333333"
                },
                "dark": {
                    "default": "#000000",
                    "subtle": "#66000000"
                },
                "light": {
                    "default": "#FFFFFF",
                    "subtle": "#33000000"
                },
                "accent": {
                    "default": "#2E89FC",
                    "subtle": "#882E89FC"
                },
                "attention": {
                    "default": "#cc3300",
                    "subtle": "#DDcc3300"
                },
                "good": {
                    "default": "#028A02",
                    "subtle": "#DD027502"
                },
                "warning": {
                    "default": "#e69500",
                    "subtle": "#DDe69500"
                }
            }
        },
        "accent": {
            "borderColor": "#62A8F7",
            "backgroundColor": "#C7DEF9",
            "foregroundColors": {
                "default": {
                    "default": "#333333",
                    "subtle": "#EE333333"
                },
                "dark": {
                    "default": "#000000",
                    "subtle": "#66000000"
                },
                "light": {
                    "default": "#FFFFFF",
                    "subtle": "#33000000"
                },
                "accent": {
                    "default": "#2E89FC",
                    "subtle": "#882E89FC"
                },
                "attention": {
                    "default": "#cc3300",
                    "subtle": "#DDcc3300"
                },
                "good": {
                    "default": "#028A02",
                    "subtle": "#DD027502"
                },
                "warning": {
                    "default": "#e69500",
                    "subtle": "#DDe69500"
                }
            }
        },
        "good": {
            "borderColor": "#69E569",
            "backgroundColor": "#CCFFCC",
            "foregroundColors": {
                "default": {
                    "default": "#333333",
                    "subtle": "#EE333333"
                },
                "dark": {
                    "default": "#000000",
                    "subtle": "#66000000"
                },
                "light": {
                    "default": "#FFFFFF",
                    "subtle": "#33000000"
                },
                "accent": {
                    "default": "#2E89FC",
                    "subtle": "#882E89FC"
                },
                "attention": {
                    "default": "#cc3300",
                    "subtle": "#DDcc3300"
                },
                "good": {
                    "default": "#028A02",
                    "subtle": "#DD027502"
                },
                "warning": {
                    "default": "#e69500",
                    "subtle": "#DDe69500"
                }
            }
        },
        "attention": {
            "borderColor": "#FF764C",
            "backgroundColor": "#FFC5B2",
            "foregroundColors": {
                "default": {
                    "default": "#333333",
                    "subtle": "#EE333333"
                },
                "dark": {
                    "default": "#000000",
                    "subtle": "#66000000"
                },
                "light": {
                    "default": "#FFFFFF",
                    "subtle": "#33000000"
                },
                "accent": {
                    "default": "#2E89FC",
                    "subtle": "#882E89FC"
                },
                "attention": {
                    "default": "#cc3300",
                    "subtle": "#DDcc3300"
                },
                "good": {
                    "default": "#028A02",
                    "subtle": "#DD027502"
                },
                "warning": {
                    "default": "#e69500",
                    "subtle": "#DDe69500"
                }
            }
        },
        "warning": {
            "borderColor": "#FFBC51",
            "backgroundColor": "#FFE2B2",
            "foregroundColors": {
                "default": {
                    "default": "#333333",
                    "subtle": "#EE333333"
                },
                "dark": {
                    "default": "#000000",
                    "subtle": "#66000000"
                },
                "light": {
                    "default": "#FFFFFF",
                    "subtle": "#33000000"
                },
                "accent": {
                    "default": "#2E89FC",
                    "subtle": "#882E89FC"
                },
                "attention": {
                    "default": "#cc3300",
                    "subtle": "#DDcc3300"
                },
                "good": {
                    "default": "#028A02",
                    "subtle": "#DD027502"
                },
                "warning": {
                    "default": "#e69500",
                    "subtle": "#DDe69500"
                }
            }
        }
    },
    "actions": {
       "preExpandSingleShowCardAction": true,
        "maxActions": 5,
        "spacing": "default",
        "buttonSpacing": 10,
        "showCard": {
            "actionMode": "inline",
            "inlineTopMargin": 16
        },
        "actionsOrientation": "horizontal",
        "actionAlignment": "left"
    },
    "adaptiveCard": {
        "allowCustomStyle": true
    },
    "imageSet": {
        "imageSize": "medium",
        "maxImageHeight": 100
    },
    "factSet": {
        "title": {
            "color": "default",
            "size": "default",
            "isSubtle": false,
            "weight": "bolder",
            "wrap": true,
            "maxWidth": 150
        },
        "value": {
            "color": "default",
            "size": "default",
            "isSubtle": false,
            "weight": "normal",
            "wrap": true
        },
        "spacing": 10
    }
}
"##;
