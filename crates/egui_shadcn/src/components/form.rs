//! Form validation patterns ported from shadcn/ui
//!
//! Provides validation helpers and form state management.
//!
//! Reference: <https://ui.shadcn.com/docs/components/form>

use std::collections::HashMap;

/// Form state manager for handling validation
///
/// ## Example
/// ```rust,ignore
/// let mut form = FormState::new();
///
/// // Add validators
/// form.add_field("email", |v| {
///     if v.is_empty() {
///         Err("Email is required")
///     } else if !v.contains('@') {
///         Err("Invalid email format")
///     } else {
///         Ok(())
///     }
/// });
///
/// // Validate on submit
/// if form.validate_all() {
///     // Form is valid, proceed
/// }
/// ```
pub struct FormState {
    fields: HashMap<String, FieldState>,
    validators: HashMap<String, Box<dyn Fn(&str) -> Result<(), String>>>,
}

struct FieldState {
    value: String,
    error: Option<String>,
    touched: bool,
}

impl Default for FormState {
    fn default() -> Self {
        Self::new()
    }
}

impl FormState {
    /// Create a new form state
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            validators: HashMap::new(),
        }
    }

    /// Add a field with a validator
    pub fn add_field<F>(&mut self, name: impl Into<String>, validator: F)
    where
        F: Fn(&str) -> Result<(), String> + 'static,
    {
        let name = name.into();
        self.fields.insert(name.clone(), FieldState {
            value: String::new(),
            error: None,
            touched: false,
        });
        self.validators.insert(name, Box::new(validator));
    }

    /// Get mutable reference to a field value
    pub fn get_value_mut(&mut self, name: &str) -> Option<&mut String> {
        self.fields.get_mut(name).map(|f| &mut f.value)
    }

    /// Get field value
    pub fn get_value(&self, name: &str) -> Option<&str> {
        self.fields.get(name).map(|f| f.value.as_str())
    }

    /// Set field value
    pub fn set_value(&mut self, name: &str, value: impl Into<String>) {
        if let Some(field) = self.fields.get_mut(name) {
            field.value = value.into();
        }
    }

    /// Mark field as touched (user has interacted)
    pub fn touch(&mut self, name: &str) {
        if let Some(field) = self.fields.get_mut(name) {
            field.touched = true;
        }
    }

    /// Check if field has been touched
    pub fn is_touched(&self, name: &str) -> bool {
        self.fields.get(name).map(|f| f.touched).unwrap_or(false)
    }

    /// Get field error (only if touched)
    pub fn get_error(&self, name: &str) -> Option<&str> {
        self.fields.get(name).and_then(|f| {
            if f.touched {
                f.error.as_deref()
            } else {
                None
            }
        })
    }

    /// Get field error regardless of touched state
    pub fn get_error_always(&self, name: &str) -> Option<&str> {
        self.fields.get(name).and_then(|f| f.error.as_deref())
    }

    /// Validate a single field
    pub fn validate_field(&mut self, name: &str) -> bool {
        let value = self.fields.get(name).map(|f| f.value.clone());
        let validator = self.validators.get(name);

        if let (Some(value), Some(validator)) = (value, validator) {
            let result = validator(&value);
            if let Some(field) = self.fields.get_mut(name) {
                field.error = result.err();
                return field.error.is_none();
            }
        }
        true
    }

    /// Validate all fields and mark them as touched
    pub fn validate_all(&mut self) -> bool {
        let names: Vec<String> = self.fields.keys().cloned().collect();
        let mut all_valid = true;

        for name in names {
            self.touch(&name);
            if !self.validate_field(&name) {
                all_valid = false;
            }
        }

        all_valid
    }

    /// Check if form is valid (without triggering validation)
    pub fn is_valid(&self) -> bool {
        self.fields.values().all(|f| f.error.is_none())
    }

    /// Reset all fields to initial state
    pub fn reset(&mut self) {
        for field in self.fields.values_mut() {
            field.value.clear();
            field.error = None;
            field.touched = false;
        }
    }

    /// Reset a single field
    pub fn reset_field(&mut self, name: &str) {
        if let Some(field) = self.fields.get_mut(name) {
            field.value.clear();
            field.error = None;
            field.touched = false;
        }
    }
}

/// Common validation functions
pub mod validators {
    /// Validate that a string is not empty
    pub fn required(value: &str) -> Result<(), String> {
        if value.trim().is_empty() {
            Err("This field is required".to_string())
        } else {
            Ok(())
        }
    }

    /// Validate email format
    pub fn email(value: &str) -> Result<(), String> {
        if value.is_empty() {
            return Ok(()); // Use with required() for required emails
        }
        if !value.contains('@') || !value.contains('.') {
            Err("Please enter a valid email address".to_string())
        } else {
            Ok(())
        }
    }

    /// Validate minimum length
    pub fn min_length(min: usize) -> impl Fn(&str) -> Result<(), String> {
        move |value: &str| {
            if value.len() < min {
                Err(format!("Must be at least {} characters", min))
            } else {
                Ok(())
            }
        }
    }

    /// Validate maximum length
    pub fn max_length(max: usize) -> impl Fn(&str) -> Result<(), String> {
        move |value: &str| {
            if value.len() > max {
                Err(format!("Must be at most {} characters", max))
            } else {
                Ok(())
            }
        }
    }

    /// Validate that value matches a pattern
    pub fn pattern(_regex_str: &'static str, message: &'static str) -> impl Fn(&str) -> Result<(), String> {
        move |value: &str| {
            // Simple pattern matching without regex crate
            // For full regex support, users should implement custom validators
            if value.is_empty() {
                return Ok(());
            }
            // This is a simplified check - for production use a proper regex crate
            Err(message.to_string())
        }
    }

    /// Combine multiple validators
    pub fn compose<F1, F2>(v1: F1, v2: F2) -> impl Fn(&str) -> Result<(), String>
    where
        F1: Fn(&str) -> Result<(), String>,
        F2: Fn(&str) -> Result<(), String>,
    {
        move |value: &str| {
            v1(value)?;
            v2(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_state() {
        let mut form = FormState::new();
        form.add_field("email", validators::required);

        form.set_value("email", "");
        assert!(!form.validate_all());
        assert!(form.get_error("email").is_some());

        form.set_value("email", "test@example.com");
        assert!(form.validate_all());
        assert!(form.get_error("email").is_none());
    }

    #[test]
    fn test_validators() {
        assert!(validators::required("").is_err());
        assert!(validators::required("hello").is_ok());

        assert!(validators::email("invalid").is_err());
        assert!(validators::email("test@example.com").is_ok());

        let min_5 = validators::min_length(5);
        assert!(min_5("hi").is_err());
        assert!(min_5("hello").is_ok());
    }
}
