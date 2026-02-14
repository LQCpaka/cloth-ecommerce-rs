use std::collections::HashMap;

use validator::ValidationErrors;

pub fn flatten_errors(errors: ValidationErrors) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

    for (field, errs) in errors.field_errors() {
        let messages: Vec<String> = errs
            .iter()
            .map(|e| {
                e.message
                    .clone()
                    .unwrap_or_else(|| "Lỗi không xác định".into())
                    .to_string()
            })
            .collect();

        result.insert(field.to_string(), messages);
    }

    result
}
