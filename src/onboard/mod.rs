// Module declarations
mod models;
mod provider_setup;
mod channel_setup;

pub mod wizard;

// Re-exported for CLI and external use
pub use wizard::{run_channels_repair_wizard, run_quick_setup, run_wizard};

// Re-export models functions
pub use models::{
    cached_model_catalog_stats, run_models_list, run_models_refresh, run_models_refresh_all,
    run_models_set, run_models_status,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_reexport_exists<F>(_value: F) {}

    #[test]
    fn wizard_functions_are_reexported() {
        assert_reexport_exists(run_channels_repair_wizard);
        assert_reexport_exists(run_quick_setup);
        assert_reexport_exists(run_wizard);
        assert_reexport_exists(run_models_refresh);
        assert_reexport_exists(run_models_list);
        assert_reexport_exists(run_models_set);
        assert_reexport_exists(run_models_status);
        assert_reexport_exists(run_models_refresh_all);
        assert_reexport_exists(cached_model_catalog_stats);
    }
}
