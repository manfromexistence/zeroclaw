use crate::config::Config;
use crate::tools::openapi::{ApisGuruSource, OpenApiRegistry, SpecHarvester};
use crate::tools::traits::Tool;
use anyhow::{Context, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
struct RegistryEntry {
    id: String,
    provider: String,
    service: String,
    version: String,
    tier: String,
    quality_score: u8,
    source: String,
    base_url: String,
    path: String,
}

pub async fn harvest_command(config: &Config, source: Option<&str>) -> Result<()> {
    let specs_root = expand_tilde(&config.openapi.specs_dir);
    let source_dir = source
        .map(PathBuf::from)
        .unwrap_or_else(|| specs_root.join("apis-guru"));

    let mut harvester = SpecHarvester::new();
    harvester.add_source(Box::new(ApisGuruSource::new(source_dir.clone())));

    println!("Harvesting OpenAPI specs from {}...", source_dir.display());
    let specs = harvester.harvest_all().await?;
    println!("Found {} specs", specs.len());
    
    println!("Deduplicating...");
    let unique = harvester.deduplicate(specs).await;
    println!("Unique specs: {}", unique.len());
    
    println!("Building registry (not copying files, using original paths)...");
    println!("Building registry (not copying files, using original paths)...");
    let mut entries = Vec::new();
    for spec in &unique {
        let spec_id = spec.dedup_key();
        
        // Use the original source path instead of copying
        let relative_path = if let Some(source_path) = spec.metadata.source.strip_prefix("file://") {
            // Make path relative to specs_root
            if let Ok(rel) = PathBuf::from(source_path).strip_prefix(&specs_root) {
                rel.to_string_lossy().to_string()
            } else {
                source_path.to_string()
            }
        } else {
            format!("apis-guru/{}", spec_id.replace("::", "_"))
        };
        
        entries.push(RegistryEntry {
            id: spec_id,
            provider: spec.metadata.provider.clone(),
            service: spec.metadata.service.clone(),
            version: spec.metadata.version.clone(),
            tier: format!("{:?}", spec.metadata.tier),
            quality_score: spec.metadata.quality_score,
            source: spec.metadata.source.clone(),
            base_url: spec.base_url.clone(),
            path: relative_path,
        });
    }

    // Write registry
    tokio::fs::create_dir_all(&specs_root)
        .await
        .with_context(|| format!("failed to create specs dir {}", specs_root.display()))?;
    let registry_path = specs_root.join("registry.json");
    let registry_data = serde_json::json!({
        "specs": entries,
        "harvested_at": chrono::Utc::now().to_rfc3339(),
        "total_specs": unique.len(),
    });
    let payload = serde_json::to_string_pretty(&registry_data)?;
    tokio::fs::write(&registry_path, payload)
        .await
        .with_context(|| format!("failed to write {}", registry_path.display()))?;

    println!("\nHarvest complete!");
    println!("  Source: {}", source_dir.display());
    println!("  Specs harvested: {}", unique.len());
    println!("  Registry: {}", registry_path.display());
    println!("\nRun 'zeroclaw openapi list' to see available specs.");
    Ok(())
}

pub async fn list_command(config: &Config) -> Result<()> {
    let registry = load_registry(config)?;
    
    let spec_count = registry.spec_count();
    let tool_count = registry.tool_count();
    
    println!("OpenAPI Integration Status");
    println!("==========================");
    println!("Specs loaded: {}", spec_count);
    println!("Tools available: {}", tool_count);
    println!();
    
    if spec_count == 0 {
        println!("No specs loaded. Run 'zeroclaw openapi harvest' to load specs.");
        return Ok(());
    }
    
    println!("Available specs:");
    for spec_id in registry.list_specs() {
        if let Some(spec) = registry.get_spec(&spec_id) {
            let tool_count = registry.get_tools_for_spec(&spec_id).len();
            println!("  {} - {} tools", spec_id, tool_count);
            println!("    Service: {}", spec.metadata.service);
            println!("    Version: {}", spec.metadata.version);
            println!("    Tier: {:?}", spec.metadata.tier);
            println!("    Quality: {}/100", spec.metadata.quality_score);
            println!();
        }
    }
    
    Ok(())
}

pub async fn tools_command(config: &Config, spec_id: &str) -> Result<()> {
    let registry = load_registry(config)?;
    
    let spec = registry.get_spec(spec_id)
        .ok_or_else(|| anyhow::anyhow!("Spec not found: {}", spec_id))?;
    
    let tools = registry.get_tools_for_spec(spec_id);
    
    println!("Tools for spec: {}", spec_id);
    println!("Service: {}", spec.metadata.service);
    println!("Version: {}", spec.metadata.version);
    println!("Base URL: {}", spec.base_url);
    println!();
    println!("Available tools ({}):", tools.len());
    
    for tool_name in tools {
        if let Some(tool) = registry.get_tool(&tool_name) {
            println!("  {}", tool.name());
            println!("    {}", tool.description());
        }
    }
    
    Ok(())
}

pub async fn test_command(config: &Config, tool_name: &str, args: serde_json::Value) -> Result<()> {
    let registry = load_registry(config)?;
    
    let tool = registry.get_tool(tool_name)
        .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", tool_name))?;
    
    println!("Testing tool: {}", tool.name());
    println!("Description: {}", tool.description());
    println!("Arguments: {}", serde_json::to_string_pretty(&args)?);
    println!();
    println!("Executing...");
    
    let result = tool.execute(args).await?;
    
    println!();
    if result.success {
        println!("✓ Success");
        println!("{}", result.output);
    } else {
        println!("✗ Failed");
        if let Some(error) = result.error {
            println!("Error: {}", error);
        }
    }
    
    Ok(())
}

pub async fn search_command(config: &Config, query: &str) -> Result<()> {
    let registry = load_registry(config)?;
    
    let results = registry.search_tools(query);
    
    println!("Search results for '{}':", query);
    println!("Found {} tools:", results.len());
    println!();
    
    for tool_name in results {
        if let Some(tool) = registry.get_tool(&tool_name) {
            println!("  {}", tool.name());
            println!("    {}", tool.description());
        }
    }
    
    Ok(())
}

fn load_registry(config: &Config) -> Result<OpenApiRegistry> {
    let specs_root = expand_tilde(&config.openapi.specs_dir);
    let registry_path = specs_root.join("registry.json");
    
    if !registry_path.exists() {
        anyhow::bail!(
            "Registry not found at {}. Run 'zeroclaw openapi harvest' first.",
            registry_path.display()
        );
    }
    
    let registry = OpenApiRegistry::new();
    registry.load_from_disk(&registry_path)?;
    
    Ok(registry)
}

fn expand_tilde(path: &str) -> PathBuf {
    Path::new(shellexpand::tilde(path).as_ref()).to_path_buf()
}
