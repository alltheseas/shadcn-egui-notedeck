# Progress

Displays an indicator showing the completion progress of a task.

![Progress Light](../assets/screenshots/light/progress.png)

## Usage

```rust
use egui_shadcn::Progress;

// Basic progress bar (75% complete)
ui.add(Progress::new(0.75));

// Full progress (100%)
ui.add(Progress::new(1.0));

// Custom height
ui.add(Progress::new(0.5).height(12.0));

// Indeterminate loading state
ui.add(Progress::new(0.0).indeterminate(true));
```

## Features

- **Value Range**: 0.0 to 1.0 (automatically clamped)
- **Custom Height**: Default 8px, configurable
- **Indeterminate Mode**: For unknown progress duration
- **Theme Colors**: Primary fill, secondary track

## API

```rust
impl Progress {
    pub fn new(value: f32) -> Self;
    pub fn height(self, height: f32) -> Self;
    pub fn indeterminate(self, indeterminate: bool) -> Self;
}

// Progress implements Widget trait
impl Widget for Progress {
    fn ui(self, ui: &mut Ui) -> Response;
}
```

## Examples

### File Upload Progress

```rust
fn show_upload_progress(ui: &mut Ui, bytes_sent: u64, total_bytes: u64) {
    let progress = bytes_sent as f32 / total_bytes as f32;

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("Uploading...");
            ui.label(format!("{}%", (progress * 100.0) as i32));
        });
        ui.add(Progress::new(progress));
        ui.small(format!(
            "{} / {} bytes",
            bytes_sent, total_bytes
        ));
    });
}
```

### Multi-Step Progress

```rust
fn show_step_progress(ui: &mut Ui, current_step: usize, total_steps: usize) {
    let progress = current_step as f32 / total_steps as f32;

    ui.vertical(|ui| {
        ui.label(format!("Step {} of {}", current_step, total_steps));
        ui.add(Progress::new(progress).height(4.0));
    });
}
```

### Loading State

```rust
fn show_loading(ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.label("Loading data...");
        ui.add(Progress::new(0.0).indeterminate(true));
    });
}
```

### Download Manager

```rust
struct Download {
    name: String,
    progress: f32,
}

fn show_downloads(ui: &mut Ui, downloads: &[Download]) {
    for download in downloads {
        ui.horizontal(|ui| {
            ui.label(&download.name);
            ui.add_space(8.0);
            ui.add_sized(
                [200.0, 8.0],
                Progress::new(download.progress),
            );
            ui.label(format!("{}%", (download.progress * 100.0) as i32));
        });
    }
}
```

### Progress with Status

```rust
enum Status {
    Pending,
    InProgress(f32),
    Completed,
    Failed,
}

fn show_task_status(ui: &mut Ui, status: &Status) {
    match status {
        Status::Pending => {
            ui.label("Waiting...");
            ui.add(Progress::new(0.0));
        }
        Status::InProgress(progress) => {
            ui.label(format!("Processing... {}%", (*progress * 100.0) as i32));
            ui.add(Progress::new(*progress));
        }
        Status::Completed => {
            ui.label("Complete!");
            ui.add(Progress::new(1.0));
        }
        Status::Failed => {
            ui.colored_label(egui::Color32::RED, "Failed");
            ui.add(Progress::new(0.0));
        }
    }
}
```
