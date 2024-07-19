use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::RwLock as StdRwLock;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use tokenizers::Tokenizer;
use async_trait::async_trait;


#[async_trait]
pub trait VecdbSearch: Send {
    async fn vecdb_search(
        &self,
        query: String,
        top_n: usize,
        filter_mb: Option<String>,
    ) -> Result<SearchResult, String>;
}

#[derive(Debug, Clone)]
pub struct VecdbConstants {
    // constant in a sense it cannot be changed without creating a new db
    pub model_name: String,
    pub embedding_size: i32,
    pub tokenizer: Arc<StdRwLock<Tokenizer>>,
    pub vectorizer_n_ctx: usize,
    pub endpoint_embeddings_template: String,
    pub endpoint_embeddings_style: String,
    pub cooldown_secs: u64,
    pub splitter_window_size: usize,
    pub vecdb_max_files: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VecDbStatus {
    pub files_unprocessed: usize,
    pub files_total: usize,  // only valid for status bar in the UI, resets to 0 when done
    pub requests_made_since_start: usize,
    pub vectors_made_since_start: usize,
    pub db_size: usize,
    pub db_cache_size: usize,
    pub state: String,   // "starting", "parsing", "done"
    pub queue_additions: bool,
    pub vecdb_max_files_hit: bool,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VecdbRecord {
    pub vector: Option<Vec<f32>>,
    pub window_text: String,
    pub window_text_hash: String,
    pub file_path: PathBuf,
    pub start_line: u64,
    pub end_line: u64,
    pub distance: f32,
    pub usefulness: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SplitResult {
    pub file_path: PathBuf,
    pub window_text: String,
    pub window_text_hash: String,
    pub start_line: u64,
    pub end_line: u64,
    pub symbol_path: String,
}

// #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[derive(Clone)]
pub struct SimpleTextHashVector {
    pub window_text: String,
    pub window_text_hash: String,
    pub vector: Option<Vec<f32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub query_text: String,
    pub results: Vec<VecdbRecord>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MemoRecord {
    pub memid: String,
    pub thevec: Option<Vec<f32>>,
    pub distance: f32,
    pub m_type: String,
    pub m_goal: String,
    pub m_project: String,
    pub m_payload: String,
    pub mstat_correct: f64,
    pub mstat_relevant: f64,
    pub mstat_times_used: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoSearchResult {
    pub query_text: String,
    pub results: Vec<MemoRecord>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OngoingWork {
    pub ongoing_goal: String,        // several structures might present inside -lsp process for different goals
    pub ongoing_attempt_n: usize,    // attempt number
    pub ongoing_progress: IndexMap<String, serde_json::Value>,                 // any dict that model sends to its future self, no additional operations on top
    pub ongoing_action_sequences: Vec<IndexMap<String, serde_json::Value>>,    // a new sequence appended to the list
    pub ongoing_output: IndexMap<String, IndexMap<String, serde_json::Value>>, // this dict updated from new data each attempt
}
