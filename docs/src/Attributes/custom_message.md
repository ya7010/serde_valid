# Custom Message

For user custom message, Serde Valid provides `message_fn` or `message`.

```rust
# extern crate serde_json;
# extern crate serde_valid;
use serde_json::json;
use serde_valid::Validate;


fn min_error_message(_params: &serde_valid::MinItemsError) -> String {
    "this is min custom message_fn.".to_string()
}

#[derive(Validate)]
struct Data (
    #[validate(min_items = 4, message_fn = min_error_message)]
    #[validate(max_items = 2, message = "this is max custom message.")]
    Vec<i32>,
);

assert_eq!(
    Data(vec![1, 2, 3]).validate().unwrap_err().to_string(),
    json!({
        "errors": [
            "this is min custom message_fn.",
            "this is max custom message."
        ]
    })
    .to_string()
);
```

<div class="warning">
Custom message is not supported in
<code>#[validate(custom = ???)]</code> validation.

Custom validation allows you to create error messages without your own validation functions.    
</div>
