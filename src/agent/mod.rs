#[allow(clippy::module_inception)]
pub mod agent;
pub mod classifier;
pub mod dispatcher;
pub mod loop_;
pub mod memory_loader;
pub mod prompt;
pub mod serializer_helper;
pub mod serializer_instructions;

#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub use agent::{Agent, AgentBuilder};
#[allow(unused_imports)]
pub use loop_::{process_message, run};
#[allow(unused_imports)]
pub use serializer_helper::{
    SerializerMessage, analyze_format, convert_json_tool_call, format_context_data,
    format_tool_args, format_tool_response, parse_tool_args,
};
#[allow(unused_imports)]
pub use serializer_instructions::{get_serializer_instruction, is_serializer_format};
