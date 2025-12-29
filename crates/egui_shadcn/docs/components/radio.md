# Radio Group

A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time.

![Radio Group Light](../assets/screenshots/light/radio.png)

## Usage

```rust
use egui_shadcn::RadioGroup;

let mut selected = 0;

// Basic radio group
RadioGroup::new("options", &mut selected)
    .option("Option 1")
    .option("Option 2")
    .option("Option 3")
    .show(ui);

// With descriptions
RadioGroup::new("plan", &mut selected)
    .option_with_description("Free", "Up to 3 projects")
    .option_with_description("Pro", "Unlimited projects")
    .option_with_description("Enterprise", "Custom features")
    .show(ui);

// Disabled
RadioGroup::new("disabled", &mut selected)
    .option("Can't select this")
    .enabled(false)
    .show(ui);
```

## Features

- **Apple HIG Compliant**: 44px touch targets
- **Descriptions**: Optional secondary text per option
- **Group Enable/Disable**: Control entire group at once
- **Focus Ring**: Visual hover/focus feedback

## API

```rust
impl<'a> RadioGroup<'a> {
    pub fn new(id: &'a str, selected: &'a mut usize) -> Self;
    pub fn option(self, label: impl Into<String>) -> Self;
    pub fn option_with_description(
        self,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> Self;
    pub fn enabled(self, enabled: bool) -> Self;
    pub fn show(self, ui: &mut Ui) -> Response;
}

// Single radio button
impl<'a> RadioButton<'a> {
    pub fn new(selected: &'a mut bool) -> Self;
    pub fn label(self, label: impl Into<String>) -> Self;
    pub fn enabled(self, enabled: bool) -> Self;
}
```

## Examples

### Subscription Plan Selection

```rust
let mut plan = 0;

RadioGroup::new("subscription", &mut plan)
    .option_with_description(
        "Free",
        "Basic features for individuals"
    )
    .option_with_description(
        "Pro",
        "$9/month - Advanced features for professionals"
    )
    .option_with_description(
        "Team",
        "$29/month - Collaboration features for teams"
    )
    .show(ui);

ui.add_space(16.0);
ui.label(format!("Selected plan: {}", ["Free", "Pro", "Team"][plan]));
```

### Settings Options

```rust
let mut theme = 0;

ui.label("Theme");
RadioGroup::new("theme", &mut theme)
    .option("Light")
    .option("Dark")
    .option("System")
    .show(ui);

ui.add_space(16.0);

let mut notifications = 1;
ui.label("Notifications");
RadioGroup::new("notifications", &mut notifications)
    .option("All")
    .option("Important only")
    .option("None")
    .show(ui);
```

### Form Selection

```rust
struct OrderForm {
    shipping: usize,
    payment: usize,
}

fn show_checkout(ui: &mut Ui, form: &mut OrderForm) {
    ui.heading("Shipping Method");
    RadioGroup::new("shipping", &mut form.shipping)
        .option_with_description(
            "Standard",
            "5-7 business days - Free"
        )
        .option_with_description(
            "Express",
            "2-3 business days - $9.99"
        )
        .option_with_description(
            "Overnight",
            "Next business day - $24.99"
        )
        .show(ui);

    ui.add_space(24.0);

    ui.heading("Payment Method");
    RadioGroup::new("payment", &mut form.payment)
        .option("Credit Card")
        .option("PayPal")
        .option("Bank Transfer")
        .show(ui);
}
```

### Survey Question

```rust
let mut answer = 0;

Card::new(ui)
    .header(|ui| {
        card_title(ui, "How satisfied are you?");
        card_description(ui, "Rate your experience with our service");
    })
    .content(|ui| {
        RadioGroup::new("satisfaction", &mut answer)
            .option("Very satisfied")
            .option("Satisfied")
            .option("Neutral")
            .option("Dissatisfied")
            .option("Very dissatisfied")
            .show(ui);
    })
    .show();
```
