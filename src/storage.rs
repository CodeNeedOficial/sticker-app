use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sticker {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Store {
    pub next_id: u32,
    pub stickers: Vec<Sticker>,
}

impl Store {
    pub fn empty() -> Self {
        Self {
            next_id: 1,
            stickers: Vec::new(),
        }
    }

    pub fn add(&mut self, title: String, content: String, tags: Vec<String>) -> &Sticker {
        let id = self.next_id;
        self.next_id += 1;
        let now = Utc::now();
        self.stickers.push(Sticker {
            id,
            title,
            content,
            tags,
            created_at: now,
            updated_at: None,
        });
        self.stickers.last().unwrap()
    }

    pub fn update(
        &mut self,
        id: u32,
        title: String,
        content: String,
        tags: Vec<String>,
    ) -> bool {
        if let Some(s) = self.stickers.iter_mut().find(|s| s.id == id) {
            s.title = title;
            s.content = content;
            s.tags = tags;
            s.updated_at = Some(Utc::now());
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, id: u32) -> bool {
        let before = self.stickers.len();
        self.stickers.retain(|s| s.id != id);
        self.stickers.len() != before
    }

    pub fn get(&self, id: u32) -> Option<&Sticker> {
        self.stickers.iter().find(|s| s.id == id)
    }
}

pub fn store_path() -> io::Result<PathBuf> {
    if let Ok(custom) = env::var("STICKER_FILE") {
        return Ok(PathBuf::from(custom));
    }
    let base = dirs::data_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "could not resolve data directory")
    })?;
    let dir = base.join("sticker");
    fs::create_dir_all(&dir)?;
    Ok(dir.join("stickers.json"))
}

pub fn load() -> io::Result<Store> {
    let path = store_path()?;
    if !path.exists() {
        return Ok(Store::empty());
    }
    let mut file = fs::File::open(&path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    if buf.trim().is_empty() {
        return Ok(Store::empty());
    }
    serde_json::from_str(&buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn save(store: &Store) -> io::Result<()> {
    let path = store_path()?;
    let json = serde_json::to_string_pretty(store)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json)?;
    fs::rename(&tmp, &path)?;
    Ok(())
}

pub fn parse_tags(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

pub fn render_sticker_md(s: &Sticker) -> String {
    let local: DateTime<Local> = s.created_at.with_timezone(&Local);
    let mut out = String::new();
    out.push_str(&format!("## #{} — {}\n\n", s.id, s.title));
    out.push_str(&format!(
        "_criado em {}_\n\n",
        local.format("%Y-%m-%d %H:%M")
    ));
    if let Some(updated) = s.updated_at {
        let l: DateTime<Local> = updated.with_timezone(&Local);
        out.push_str(&format!("_editado em {}_\n\n", l.format("%Y-%m-%d %H:%M")));
    }
    if !s.tags.is_empty() {
        let tags: Vec<String> = s.tags.iter().map(|t| format!("`{}`", t)).collect();
        out.push_str(&format!("**tags:** {}\n\n", tags.join(" ")));
    }
    out.push_str(&s.content);
    if !s.content.ends_with('\n') {
        out.push('\n');
    }
    out
}

pub fn render_all_md(store: &Store) -> String {
    if store.stickers.is_empty() {
        return "_nenhum sticker salvo._\n".to_string();
    }
    let mut out = String::from("# Stickers\n\n");
    for s in &store.stickers {
        out.push_str(&render_sticker_md(s));
        out.push_str("\n---\n\n");
    }
    out
}
