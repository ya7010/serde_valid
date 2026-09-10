use serde_json::json;
use serde_valid::Validate;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[test]
fn sequence_vec_array_and_borrowed_slice() {
    #[derive(Validate)]
    struct Cases<'a> {
        #[validate(min_length = 1)]
        values: Vec<&'a str>,
        #[validate(min_length = 1)]
        array: [&'a str; 1],
        #[validate(min_length = 1)]
        slice: &'a [&'a str],
    }

    assert_eq!(
        serde_json::to_value(
            Cases {
                values: vec![""],
                array: [""],
                slice: &[""],
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "values": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The length of the value must be `>= 1`."]
                        }
                    }
                },
                "array": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The length of the value must be `>= 1`."]
                        }
                    }
                },
                "slice": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The length of the value must be `>= 1`."]
                        }
                    }
                }
            }
        })
    );
}

#[test]
fn sequence_path_also_covers_maps() {
    #[derive(Validate)]
    struct Cases<'a> {
        #[validate(minimum = 1)]
        hash_map: HashMap<&'a str, i32>,
        #[validate(minimum = 1)]
        btree_map: BTreeMap<&'a str, i32>,
        #[validate(minimum = 1)]
        index_map: indexmap::IndexMap<&'a str, i32>,
    }

    assert_eq!(
        serde_json::to_value(
            Cases {
                hash_map: HashMap::from([("hash", 0)]),
                btree_map: BTreeMap::from([("btree", 0)]),
                index_map: indexmap::IndexMap::from([("index", 0)]),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "hash_map": {
                    "errors": [],
                    "properties": {
                        "hash": {
                            "errors": ["The number must be `>= 1`."]
                        }
                    }
                },
                "btree_map": {
                    "errors": [],
                    "properties": {
                        "btree": {
                            "errors": ["The number must be `>= 1`."]
                        }
                    }
                },
                "index_map": {
                    "errors": [],
                    "properties": {
                        "index": {
                            "errors": ["The number must be `>= 1`."]
                        }
                    }
                }
            }
        })
    );
}

#[test]
fn sequence_nested_containers_preserve_item_paths() {
    #[derive(Validate)]
    struct Cases {
        #[validate(minimum = 1)]
        nested: Vec<[i32; 1]>,
    }

    assert_eq!(
        serde_json::to_value(Cases { nested: vec![[0]] }.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "nested": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": [],
                            "items": {
                                "0": {
                                    "errors": ["The number must be `>= 1`."]
                                }
                            }
                        }
                    }
                }
            }
        })
    );
}

#[test]
#[allow(clippy::box_collection)]
fn sequence_wrappers_forward_to_inner_sequence_or_map() {
    #[derive(Validate)]
    struct Cases<'a> {
        #[validate(minimum = 1)]
        boxed_vec: Box<Vec<i32>>,
        #[validate(minimum = 1)]
        boxed_slice: Box<[i32]>,
        #[validate(minimum = 1)]
        boxed_array: Box<[i32; 1]>,
        #[validate(minimum = 1)]
        boxed_map: Box<HashMap<&'a str, i32>>,
        #[validate(minimum = 1)]
        borrowed_vec: &'a Vec<i32>,
        #[validate(minimum = 1)]
        rc_vec: Rc<Vec<i32>>,
        #[validate(minimum = 1)]
        arc_map: Arc<HashMap<&'a str, i32>>,
        #[validate(minimum = 1)]
        cow_slice: Cow<'a, [i32]>,
        #[validate(minimum = 1)]
        pinned_vec: Pin<Box<Vec<i32>>>,
    }

    let owned_vec = vec![0];
    let value = Cases {
        boxed_vec: Box::new(vec![0]),
        boxed_slice: vec![0].into_boxed_slice(),
        boxed_array: Box::new([0]),
        boxed_map: Box::new(HashMap::from([("k", 0)])),
        borrowed_vec: &owned_vec,
        rc_vec: Rc::new(vec![0]),
        arc_map: Arc::new(HashMap::from([("k", 0)])),
        cow_slice: Cow::Borrowed(&[0]),
        pinned_vec: Pin::new(Box::new(vec![0])),
    };

    assert_eq!(
        serde_json::to_value(value.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "boxed_vec": {
                    "errors": [],
                    "items": { "0": { "errors": ["The number must be `>= 1`."] } }
                },
                "boxed_slice": {
                    "errors": [],
                    "items": { "0": { "errors": ["The number must be `>= 1`."] } }
                },
                "boxed_array": {
                    "errors": [],
                    "items": { "0": { "errors": ["The number must be `>= 1`."] } }
                },
                "boxed_map": {
                    "errors": [],
                    "properties": { "k": { "errors": ["The number must be `>= 1`."] } }
                },
                "borrowed_vec": {
                    "errors": [],
                    "items": { "0": { "errors": ["The number must be `>= 1`."] } }
                },
                "rc_vec": {
                    "errors": [],
                    "items": { "0": { "errors": ["The number must be `>= 1`."] } }
                },
                "arc_map": {
                    "errors": [],
                    "properties": { "k": { "errors": ["The number must be `>= 1`."] } }
                },
                "cow_slice": {
                    "errors": [],
                    "items": { "0": { "errors": ["The number must be `>= 1`."] } }
                },
                "pinned_vec": {
                    "errors": [],
                    "items": { "0": { "errors": ["The number must be `>= 1`."] } }
                }
            }
        })
    );
}
