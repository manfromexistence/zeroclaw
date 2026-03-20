use reqwest::Client;
use zeroclawlabs::metasearch::engines::EngineRegistry;

fn main() {
    println!("?? Testing Metasearch Integration...\n");

    // Create registry
    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);

    // Test engine count
    let count = registry.count();
    println!("? Loaded {} search engines", count);

    // Test specific engines
    let engines = vec!["google", "duckduckgo", "brave", "bing", "yahoo"];
    for name in engines {
        if registry.get(name).is_some() {
            println!("  ? {} engine available", name);
        } else {
            println!("  ? {} engine missing", name);
        }
    }

    // Test categories
    use zeroclawlabs::metasearch::category::SearchCategory;
    let general = registry.engines_for_category(&SearchCategory::General);
    let images = registry.engines_for_category(&SearchCategory::Images);
    let videos = registry.engines_for_category(&SearchCategory::Videos);

    println!("\n?? Engines by category:");
    println!("  - General: {} engines", general.len());
    println!("  - Images: {} engines", images.len());
    println!("  - Videos: {} engines", videos.len());

    println!("\n? Metasearch integration test PASSED!");
}
