use godot::prelude::*;
use modelica_rust_ffi::{ModelicaRuntime, ComponentResult, SimulationComponent};
use std::collections::HashMap;

/// Generic Modelica component that can load any .mo file
pub struct GenericModelicaComponent {
    runtime: Option<ModelicaRuntime>,
    component_name: String,
}

impl GenericModelicaComponent {
    pub fn new() -> Self {
        Self {
            runtime: None,
            component_name: String::new(),
        }
    }
    
    pub fn load_from_file(&mut self, component_name: &str) -> ComponentResult<()> {
        self.component_name = component_name.to_string();
        self.runtime = Some(ModelicaRuntime::new(component_name)?);
        Ok(())
    }
}

impl SimulationComponent for GenericModelicaComponent {
    fn component_type(&self) -> &str {
        &self.component_name
    }
    
    fn initialize(&mut self) -> ComponentResult<()> {
        if let Some(ref mut runtime) = self.runtime {
            runtime.reset()
        } else {
            Err(modelica_rust_ffi::ComponentError::InitializationFailed(
                "Component not loaded".to_string()
            ))
        }
    }
    
    fn set_input(&mut self, name: &str, value: f64) -> ComponentResult<()> {
        if let Some(ref mut runtime) = self.runtime {
            runtime.set_real_variable(name, value)
        } else {
            Err(modelica_rust_ffi::ComponentError::InvalidInput(
                "Component not loaded".to_string()
            ))
        }
    }
    
    fn set_bool_input(&mut self, name: &str, value: bool) -> ComponentResult<()> {
        if let Some(ref mut runtime) = self.runtime {
            runtime.set_bool_variable(name, value)
        } else {
            Err(modelica_rust_ffi::ComponentError::InvalidInput(
                "Component not loaded".to_string()
            ))
        }
    }
    
    fn get_output(&self, name: &str) -> ComponentResult<f64> {
        if let Some(ref runtime) = self.runtime {
            runtime.get_real_variable(name)
        } else {
            Err(modelica_rust_ffi::ComponentError::InvalidOutput(
                "Component not loaded".to_string()
            ))
        }
    }
    
    fn step(&mut self, dt: f64) -> ComponentResult<()> {
        if let Some(ref mut runtime) = self.runtime {
            runtime.step(dt)
        } else {
            Err(modelica_rust_ffi::ComponentError::StepFailed(
                "Component not loaded".to_string()
            ))
        }
    }
    
    fn reset(&mut self) -> ComponentResult<()> {
        if let Some(ref mut runtime) = self.runtime {
            runtime.reset()
        } else {
            Ok(())
        }
    }
    
    fn get_all_outputs(&self) -> HashMap<String, f64> {
        HashMap::new()
    }
    
    fn metadata(&self) -> modelica_rust_ffi::ComponentMetadata {
        modelica_rust_ffi::ComponentMetadata {
            name: self.component_name.clone(),
            component_type: "Generic".to_string(),
            inputs: vec![],
            outputs: vec![],
        }
    }
}