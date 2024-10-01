use eframe::egui;
use rand::Rng;
use std::{clone, collections::HashMap};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Time Manager",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    tasks: HashMap<i32, (i32, String)>,
    completed_tasks: HashMap<i32, (i32, String)>,
    errors: HashMap<i32, String>,
    new_task_description: String,
    new_task_difficulty: i32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut tasks_to_remove = Vec::new(); // Collect keys of tasks to remove
            let mut errors_to_remove = Vec::new();
            let mut complete_task = Vec::new();

            if !self.tasks.is_empty() {
                ui.heading("Tasks: ");

                for (key, (difficulty, description)) in &self.tasks {
                    ui.label(format!("Id: {}, Difficulty: {}, Description: {}", key, difficulty, description));
                    if ui.button("❌").clicked() {
                        tasks_to_remove.push(*key);
                    }
                    if ui.button("✔").clicked() {
                        complete_task.push(*key);
                    }
                }
            }

            ui.heading("Add Task:");
            let mut rng = rand::thread_rng();
            let mut new_task_id;
            // Generate a random key until a unique one is found
            loop {
                new_task_id = rng.gen_range(1..=1000);  // Generate a random number between 1 and 1000
                if !self.tasks.contains_key(&new_task_id) {
                    break;  // Break the loop if the key doesn't exist in the map
                }
            }
            ui.label(format!("Task Description: "));
            let response = ui.add(egui::TextEdit::singleline(&mut self.new_task_description));
            if response.changed() {
                println!("New Task Description: {}", self.new_task_description);
            }

            let response = ui.add(egui::Slider::new(&mut self.new_task_difficulty, 0..=100).text("Task Difficulty"));
            if response.changed() {
                println!("New Task Difficulty: {}", self.new_task_difficulty);
            }

            if ui.button("Add Task").clicked() {
                if !self.new_task_description.is_empty() {
                    self.tasks.insert(new_task_id, (self.new_task_difficulty.clone(), self.new_task_description.clone()));
                    println!("Created task: {}, {}", self.new_task_difficulty, self.new_task_description);
                    self.new_task_description = "".to_owned();
                    self.new_task_difficulty = 0.to_owned();
                } else {
                    let mut new_error_id;
                    // Generate a random key until a unique one is found
                    loop {
                        new_error_id = rng.gen_range(1..=1000);  // Generate a random number between 1 and 1000
                        if !self.errors.contains_key(&new_error_id) {
                            break;  // Break the loop if the key doesn't exist in the map
                        }
                    }
                    self.errors.insert(new_error_id, "Error: task description empty!".to_owned());
                }
                
            }

            if !self.completed_tasks.is_empty() {
                ui.heading("Completed Tasks: ");

                for (key, (difficulty, description)) in &self.completed_tasks {
                    ui.label(format!("Id: {}, Difficulty: {}, Description: {}", key, difficulty, description));
                    if ui.button("❌").clicked() {
                        tasks_to_remove.push(*key);
                    }
                }
            }

            // Render Errors
            if !self.errors.is_empty() {
                ui.heading("Error(s): ");
                for (key, error) in &self.errors {
                    ui.label(error);
                    if ui.button("❌").clicked() {
                        errors_to_remove.push(*key);
                    }
                }
            }

            for key in complete_task {
                if let Some((key, (difficulty, description))) = self.tasks.get_key_value(&key) {
                    let cloned_task = (difficulty.clone(), description.clone());
                    self.completed_tasks.insert(*key, cloned_task);
                    tasks_to_remove.push(*key);
                    println!("Completed task: {}, {}", difficulty, description);
                } else {
                    add_error(self, "Error: Unable to complete task!".to_owned());
                }
            }
            
            // After the iteration, remove the tasks
            for key in tasks_to_remove {
                // Try removing from the first `HashMap` (self.tasks)
                let removed_value = self.tasks.remove(&key);

                // If not found in `self.tasks`, try removing from `self.completed_tasks`
                let removed_value = match removed_value {
                    Some(value) => {
                        println!("Removed Task from tasks: {}", value.0);
                        Some(value) // Task found in `self.tasks`
                    }
                    None => {
                        // Task not found in `self.tasks`, check `self.completed_tasks`
                        match self.completed_tasks.remove(&key) {
                            Some(value) => {
                                println!("Removed Task from completed_tasks: {}", value.0);
                                Some(value) // Task found in `self.completed_tasks`
                            }
                            None => {
                                println!("Key not found in either tasks or completed_tasks.");
                                None // Task not found in either `HashMap`
                            }
                        }
                    }
                };
            }

            for key in errors_to_remove {
                let removed_value = self.errors.remove(&key);
                // Check what was removed
                match removed_value {
                    Some(value) => println!("Removed Error: {}", value),
                    None => println!("Key not found."),
                }
            }

            fn add_error(app: &mut MyApp, error: String) {
                let mut rng = rand::thread_rng();
                let mut new_error_id;
                // Generate a random key until a unique one is found
                loop {
                    new_error_id = rng.gen_range(1..=1000);  // Generate a random number between 1 and 1000
                    if !app.errors.contains_key(&new_error_id) {
                        break;  // Break the loop if the key doesn't exist in the map
                    }
                }
                app.errors.insert(new_error_id, error);
            }
        });
    }
    
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}
    
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
    
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()
    
        // _visuals.window_fill() would also be a natural choice
    }
    
    fn persist_egui_memory(&self) -> bool {
        true
    }
    
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {}
}