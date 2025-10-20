use godot::prelude::*;
use crate::modelica_component::GenericModelicaComponent;
use modelica_rust_ffi::SimulationComponent;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ModelicaNode {
    base: Base<Node3D>,
    component: GenericModelicaComponent,
    
    #[export]
    component_name: GString,
    
    #[export]
    auto_initialize: bool,
}

#[godot_api]
impl INode3D for ModelicaNode {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            component: GenericModelicaComponent::new(),
            component_name: GString::new(),
            auto_initialize: true,
        }
    }
    
    fn ready(&mut self) {
        if self.auto_initialize && !self.component_name.is_empty() {
            if let Err(e) = self.load_component(self.component_name.clone()) {
                godot_error!("Failed to load component: {:?}", e);
            }
        }
    }
    
    fn process(&mut self, delta: f64) {
        if let Err(e) = self.component.step(delta) {
            godot_error!("Simulation step failed: {:?}", e);
        }
    }
}

#[godot_api]
impl ModelicaNode {
    /// Load a Modelica component by name
    #[func]
    pub fn load_component(&mut self, component_name: GString) -> bool {
        godot_print!("Loading Modelica component: {}", component_name);
        
        match self.component.load_from_file(&component_name.to_string()) {
            Ok(_) => {
                godot_print!("✓ Component loaded");
                if let Err(e) = self.component.initialize() {
                    godot_error!("Failed to initialize: {:?}", e);
                    return false;
                }
                godot_print!("✓ Component initialized");
                true
            }
            Err(e) => {
                godot_error!("Failed to load component: {:?}", e);
                false
            }
        }
    }
    
    /// Set a real input value
    #[func]
    pub fn set_real_input(&mut self, name: GString, value: f64) -> bool {
        self.component.set_input(&name.to_string(), value).is_ok()
    }
    
    /// Set a boolean input value
    #[func]
    pub fn set_bool_input(&mut self, name: GString, value: bool) -> bool {
        self.component.set_bool_input(&name.to_string(), value).is_ok()
    }
    
    /// Get a real output value
    #[func]
    pub fn get_real_output(&self, name: GString) -> f64 {
        self.component.get_output(&name.to_string()).unwrap_or(0.0)
    }
    
    /// Get all output values as a dictionary
    #[func]
    pub fn get_all_outputs(&self) -> Dictionary {
        let dict = Dictionary::new();
        for (key, value) in self.component.get_all_outputs() {
            dict.set(key, value);
        }
        dict
    }
    
    /// Reset the simulation to initial conditions
    #[func]
    pub fn reset_simulation(&mut self) -> bool {
        self.component.reset().is_ok()
    }
}