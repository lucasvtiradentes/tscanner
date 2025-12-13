use crate::config_ext::ConfigExt;
use crate::executors::{AiExecutor, ScriptExecutor};
use globset::GlobSet;
use std::path::PathBuf;
use std::sync::Arc;
use tscanner_cache::{AiCache, FileCache, ScriptCache};
use tscanner_config::{compile_globset, TscannerConfig, TscannerConfigExt};
use tscanner_rules::RuleRegistry;

pub struct Scanner {
    pub registry: RuleRegistry,
    pub(crate) config: TscannerConfig,
    pub(crate) cache: Arc<FileCache>,
    pub(crate) ai_cache: Arc<AiCache>,
    pub(crate) script_cache: Arc<ScriptCache>,
    pub(crate) root: PathBuf,
    pub(crate) global_include: GlobSet,
    pub(crate) global_exclude: GlobSet,
    pub(crate) custom_include: Option<GlobSet>,
    pub(crate) script_executor: ScriptExecutor,
    pub(crate) ai_executor: AiExecutor,
    pub(crate) log_info: fn(&str),
    pub(crate) log_debug: fn(&str),
}

impl Scanner {
    pub fn new(config: TscannerConfig, root: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_logger(config, root, |_| {}, |_| {}, |_| {}, |_| {})
    }

    pub fn with_cache(
        config: TscannerConfig,
        cache: Arc<FileCache>,
        root: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_cache_and_logger(config, cache, root, None, |_| {}, |_| {}, |_| {}, |_| {})
    }

    pub fn with_cache_and_config_dir(
        config: TscannerConfig,
        cache: Arc<FileCache>,
        root: PathBuf,
        config_dir: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_cache_and_logger(
            config,
            cache,
            root,
            Some(config_dir),
            |_| {},
            |_| {},
            |_| {},
            |_| {},
        )
    }

    pub fn with_caches(
        config: TscannerConfig,
        cache: Arc<FileCache>,
        ai_cache: Arc<AiCache>,
        script_cache: Arc<ScriptCache>,
        root: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_all_caches_and_logger(
            config,
            cache,
            ai_cache,
            script_cache,
            root,
            None,
            |_| {},
            |_| {},
            |_| {},
            |_| {},
        )
    }

    pub fn with_caches_and_config_dir(
        config: TscannerConfig,
        cache: Arc<FileCache>,
        ai_cache: Arc<AiCache>,
        script_cache: Arc<ScriptCache>,
        root: PathBuf,
        config_dir: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_all_caches_and_logger(
            config,
            cache,
            ai_cache,
            script_cache,
            root,
            Some(config_dir),
            |_| {},
            |_| {},
            |_| {},
            |_| {},
        )
    }

    pub fn with_logger(
        config: TscannerConfig,
        root: PathBuf,
        log_info: fn(&str),
        log_debug: fn(&str),
        log_error: fn(&str),
        log_warn: fn(&str),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config_hash = config.compute_hash();
        let cache = Arc::new(FileCache::with_config_hash(config_hash));
        Self::with_cache_and_logger(
            config, cache, root, None, log_info, log_debug, log_error, log_warn,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_cache_and_logger(
        config: TscannerConfig,
        cache: Arc<FileCache>,
        root: PathBuf,
        config_dir: Option<PathBuf>,
        log_info: fn(&str),
        log_debug: fn(&str),
        log_error: fn(&str),
        log_warn: fn(&str),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config_hash = config.compute_hash();
        let ai_cache = Arc::new(AiCache::with_config_hash(config_hash));
        Self::with_caches_and_logger(
            config, cache, ai_cache, root, config_dir, log_info, log_debug, log_error, log_warn,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_caches_and_logger(
        config: TscannerConfig,
        cache: Arc<FileCache>,
        ai_cache: Arc<AiCache>,
        root: PathBuf,
        config_dir: Option<PathBuf>,
        log_info: fn(&str),
        log_debug: fn(&str),
        log_error: fn(&str),
        log_warn: fn(&str),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config_hash = config.compute_hash();
        let script_cache = Arc::new(ScriptCache::with_config_hash(config_hash));
        Self::with_all_caches_and_logger(
            config,
            cache,
            ai_cache,
            script_cache,
            root,
            config_dir,
            log_info,
            log_debug,
            log_error,
            log_warn,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_all_caches_and_logger(
        config: TscannerConfig,
        cache: Arc<FileCache>,
        ai_cache: Arc<AiCache>,
        script_cache: Arc<ScriptCache>,
        root: PathBuf,
        config_dir: Option<PathBuf>,
        log_info: fn(&str),
        log_debug: fn(&str),
        log_error: fn(&str),
        log_warn: fn(&str),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = RuleRegistry::with_config(
            &config,
            ConfigExt::compile_builtin_rule,
            ConfigExt::compile_custom_rule,
            log_info,
            log_error,
        )?;
        let global_include = compile_globset(&config.files.include)?;
        let global_exclude = compile_globset(&config.files.exclude)?;
        let custom_patterns = config.get_rule_specific_include_patterns();
        let custom_include = if custom_patterns.is_empty() {
            None
        } else {
            Some(compile_globset(&custom_patterns)?)
        };
        let (script_executor, ai_executor) = match config_dir {
            Some(ref dir) => (
                ScriptExecutor::with_config_dir_and_logger(
                    dir.clone(),
                    script_cache.clone(),
                    log_error,
                    log_debug,
                ),
                AiExecutor::with_config_dir(
                    &root,
                    dir.clone(),
                    config.ai.clone(),
                    ai_cache.clone(),
                    log_warn,
                    log_debug,
                ),
            ),
            None => (
                ScriptExecutor::with_logger(&root, script_cache.clone(), log_error, log_debug),
                AiExecutor::with_config(
                    &root,
                    config.ai.clone(),
                    ai_cache.clone(),
                    log_warn,
                    log_debug,
                ),
            ),
        };
        Ok(Self {
            registry,
            config,
            cache,
            ai_cache,
            script_cache,
            root,
            global_include,
            global_exclude,
            custom_include,
            script_executor,
            ai_executor,
            log_info,
            log_debug,
        })
    }

    pub fn cache(&self) -> Arc<FileCache> {
        self.cache.clone()
    }

    pub fn ai_cache(&self) -> Arc<AiCache> {
        self.ai_cache.clone()
    }

    pub fn clear_script_cache(&self) {
        self.script_executor.clear_cache();
    }

    pub fn flush_ai_cache(&self) {
        self.ai_executor.flush_cache();
    }

    pub fn flush_script_cache(&self) {
        self.script_executor.flush_cache();
    }
}
