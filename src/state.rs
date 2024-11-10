use async_lsp::lsp_types::Url;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tracing::info;

pub struct LanguageState {
    documents: Arc<RwLock<HashMap<Url, String>>>,
    language_ids: Arc<RwLock<HashMap<Url, String>>>,
    pub client_info: ClientInfo,
}

#[derive(Default)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

impl LanguageState {
    pub fn new() -> Self {
        LanguageState {
            documents: Default::default(),
            language_ids: Default::default(),
            client_info: Default::default(),
        }
    }

    pub fn get_contents(&self, uri: &Url) -> String {
        self.documents
            .read()
            .expect("poison")
            .get(uri)
            .map(|s| s.to_owned())
            .unwrap_or_default()
    }

    pub fn get_language_id(&self, uri: &Url) -> String {
        self.language_ids
            .read()
            .expect("poison")
            .get(uri)
            .map(|s| s.to_owned())
            .unwrap_or_default()
    }

    pub fn upsert_content(&mut self, uri: &Url, content: String) {
        let mut docs = self.documents.write().expect("poison");
        docs.insert(uri.clone(), content);
    }

    pub fn upsert_file(&mut self, uri: &Url, content: String, language_id: Option<String>) {
        info!("upserting file: {}", uri);
        if let Some(language_id) = language_id {
            self.language_ids
                .write()
                .expect("poison")
                .insert(uri.clone(), language_id);
        };
        self.upsert_content(uri, content);
    }

    pub fn update_client_info(&mut self, name: String, version: String) {
        self.client_info = ClientInfo { name, version };
    }
}
