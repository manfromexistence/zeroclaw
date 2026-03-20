use super::spec::{OpenApiSpec, SpecTier};
use async_trait::async_trait;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub struct SpecHarvester {
    sources: Vec<Box<dyn SpecSource>>,
}

#[async_trait]
pub trait SpecSource: Send + Sync {
    async fn fetch_specs(&self) -> anyhow::Result<Vec<OpenApiSpec>>;
    fn name(&self) -> &str;
}

pub struct ApisGuruSource {
    pub repo_path: PathBuf,
}

impl ApisGuruSource {
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }
}

#[async_trait]
impl SpecSource for ApisGuruSource {
    async fn fetch_specs(&self) -> anyhow::Result<Vec<OpenApiSpec>> {
        let mut specs = Vec::new();
        let mut files = Vec::new();
        collect_openapi_files(&self.repo_path, &mut files)?;

        for file in files {
            match OpenApiSpec::from_file(&file) {
                Ok(mut spec) => {
                    if let Some((provider, service, version)) =
                        infer_provider_service_version(&self.repo_path, &file)
                    {
                        spec.metadata.provider = provider;
                        spec.metadata.service = service;
                        spec.metadata.version = version;
                    }
                    spec.metadata.tier = SpecTier::Community;
                    if let Ok(report) = spec.validate() {
                        spec.metadata.quality_score = report.quality_score;
                        if report.is_valid && report.quality_score >= 80 {
                            spec.metadata.tier = SpecTier::Verified;
                        }
                    }
                    specs.push(spec);
                }
                Err(error) => {
                    tracing::debug!("openapi: failed to parse {}: {error}", file.display());
                }
            }
        }
        Ok(specs)
    }

    fn name(&self) -> &str {
        "apis_guru"
    }
}

impl SpecHarvester {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn add_source(&mut self, source: Box<dyn SpecSource>) {
        self.sources.push(source);
    }

    pub async fn harvest_all(&self) -> anyhow::Result<Vec<OpenApiSpec>> {
        let mut all_specs = Vec::new();
        for source in &self.sources {
            let specs = source.fetch_specs().await?;
            tracing::info!(
                "openapi: source={} harvested={} specs",
                source.name(),
                specs.len()
            );
            all_specs.extend(specs);
        }
        Ok(all_specs)
    }

    pub async fn deduplicate(&self, specs: Vec<OpenApiSpec>) -> Vec<OpenApiSpec> {
        let mut seen = HashSet::new();
        let mut unique = Vec::new();
        for spec in specs {
            let key = spec.dedup_key();
            if seen.insert(key) {
                unique.push(spec);
            }
        }
        unique
    }
}

fn collect_openapi_files(root: &Path, out: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    if !root.exists() {
        anyhow::bail!("OpenAPI source directory not found: {}", root.display());
    }
    for entry in std::fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_openapi_files(&path, out)?;
            continue;
        }
        if path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|ext| matches!(ext.to_ascii_lowercase().as_str(), "json" | "yaml" | "yml"))
        {
            out.push(path);
        }
    }
    Ok(())
}

fn infer_provider_service_version(root: &Path, file: &Path) -> Option<(String, String, String)> {
    let relative = file.strip_prefix(root).ok()?;
    let components = relative
        .iter()
        .map(|part| part.to_string_lossy().to_string())
        .collect::<Vec<_>>();
    if components.len() < 3 {
        return None;
    }
    let provider = components.first()?.clone();
    let service = components.get(1)?.clone();
    let version = components.get(2)?.clone();
    Some((provider, service, version))
}
