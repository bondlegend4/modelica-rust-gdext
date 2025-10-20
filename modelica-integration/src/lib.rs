use godot::prelude::*;

mod modelica_node;
mod modelica_component;

struct ModelicaIntegration;

#[gdextension]
unsafe impl ExtensionLibrary for ModelicaIntegration {}