use bevy::prelude::*;
use std::collections::HashMap;
use std::path::Path;

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Localization::default())
            .insert_resource(CurrentLanguage::default())
            .add_systems(Startup, setup_localization);
    }
}

// ---------------------------------------------------------------------------
// Language enum
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    ChineseSimplified,
    ChineseTraditional,
    Japanese,
    Korean,
    French,
    German,
    SpanishES,
    SpanishLatAm,
    PortugueseBR,
    Italian,
    Russian,
    Polish,
    Dutch,
    Indonesian,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

/// All supported languages in load order.
const ALL_LANGUAGES: [(Language, &str); 15] = [
    (Language::English, "en-us"),
    (Language::ChineseSimplified, "zh_CN"),
    (Language::ChineseTraditional, "zh_TW"),
    (Language::Japanese, "ja"),
    (Language::Korean, "ko"),
    (Language::French, "fr"),
    (Language::German, "de"),
    (Language::SpanishES, "es_ES"),
    (Language::SpanishLatAm, "es_419"),
    (Language::PortugueseBR, "pt_BR"),
    (Language::Italian, "it"),
    (Language::Russian, "ru"),
    (Language::Polish, "pl"),
    (Language::Dutch, "nl"),
    (Language::Indonesian, "id"),
];

impl Language {
    /// Returns the filename stem used inside `balatro-source-code/localization/`.
    pub fn file_stem(self) -> &'static str {
        for &(lang, stem) in &ALL_LANGUAGES {
            if lang == self {
                return stem;
            }
        }
        "en-us"
    }

    /// Iterate over every supported language.
    pub fn all() -> &'static [(Language, &'static str); 15] {
        &ALL_LANGUAGES
    }
}

// ---------------------------------------------------------------------------
// CurrentLanguage resource
// ---------------------------------------------------------------------------

#[derive(Resource, Default)]
pub struct CurrentLanguage {
    pub lang: Language,
}

// ---------------------------------------------------------------------------
// Description entry – name + text lines
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct DescriptionEntry {
    pub name: String,
    pub text: Vec<String>,
}

// ---------------------------------------------------------------------------
// MiscValue – a single string *or* a list of strings
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum MiscValue {
    Single(String),
    List(Vec<String>),
}

impl MiscValue {
    pub fn as_str(&self) -> &str {
        match self {
            MiscValue::Single(s) => s,
            MiscValue::List(v) => v.first().map(|s| s.as_str()).unwrap_or(""),
        }
    }

    pub fn as_lines(&self) -> &[String] {
        match self {
            MiscValue::Single(s) => std::slice::from_ref(s),
            MiscValue::List(v) => v,
        }
    }
}

// ---------------------------------------------------------------------------
// Per‑language data produced by the Lua parser
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct LanguageData {
    /// `descriptions.<Category>.<key>` → DescriptionEntry
    pub descriptions: HashMap<String, HashMap<String, DescriptionEntry>>,
    /// `misc.<section>.<key>` → MiscValue
    pub misc: HashMap<String, HashMap<String, MiscValue>>,
}

// ---------------------------------------------------------------------------
// Localization resource
// ---------------------------------------------------------------------------

#[derive(Resource)]
pub struct Localization {
    /// Flat key→(lang→string) map used by the existing UI (`loc.get("play_hand")`).
    pub strings: HashMap<String, HashMap<Language, String>>,
    /// Full per‑language data parsed from the Lua files.
    pub data: HashMap<Language, LanguageData>,
}

impl Default for Localization {
    fn default() -> Self {
        Self {
            strings: HashMap::new(),
            data: HashMap::new(),
        }
    }
}

impl Localization {
    // -- Backward‑compatible flat‑string API -----------------------------------

    /// Get a flat UI string for the current default language (English).
    pub fn get(&self, key: &str) -> String {
        self.get_for(key, &Language::English)
    }

    /// Get a flat UI string for a specific language.
    pub fn get_for(&self, key: &str, lang: &Language) -> String {
        self.strings
            .get(key)
            .and_then(|map| map.get(lang).or_else(|| map.get(&Language::English)))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    // -- Rich lookup API -------------------------------------------------------

    /// Look up a card/item description.
    ///
    /// `category` is the Lua table name: `"Joker"`, `"Tarot"`, `"Planet"`,
    /// `"Spectral"`, `"Voucher"`, `"Blind"`, `"Tag"`, `"Other"`, `"Enhanced"`,
    /// `"Edition"`, `"Back"`, `"Stake"`.
    ///
    /// `key` is the item id, e.g. `"j_joker"`, `"c_fool"`.
    pub fn get_description(
        &self,
        category: &str,
        key: &str,
        lang: &Language,
    ) -> Option<&DescriptionEntry> {
        let try_lang = |l: &Language| -> Option<&DescriptionEntry> {
            self.data.get(l)?.descriptions.get(category)?.get(key)
        };
        try_lang(lang).or_else(|| try_lang(&Language::English))
    }

    /// Look up a misc value.
    ///
    /// `section` is the second‑level key under `misc`:
    /// `"dictionary"`, `"poker_hands"`, `"poker_hand_descriptions"`, `"labels"`,
    /// `"challenge_names"`, `"v_dictionary"`, `"v_text"`, `"tutorial"`, `"quips"`,
    /// `"suits_singular"`, `"suits_plural"`, `"blind_states"`, `"ranks"`, etc.
    pub fn get_misc(
        &self,
        section: &str,
        key: &str,
        lang: &Language,
    ) -> Option<&MiscValue> {
        let try_lang = |l: &Language| -> Option<&MiscValue> {
            self.data.get(l)?.misc.get(section)?.get(key)
        };
        try_lang(lang).or_else(|| try_lang(&Language::English))
    }
}

// ---------------------------------------------------------------------------
// Startup system
// ---------------------------------------------------------------------------

fn setup_localization(mut loc: ResMut<Localization>) {
    let base = Path::new("balatro-source-code/localization");

    for &(lang, stem) in Language::all() {
        let path = base.join(format!("{stem}.lua"));
        match std::fs::read_to_string(&path) {
            Ok(contents) => {
                let lang_data = parse_lua(&contents);
                populate_flat_strings(&mut loc.strings, &lang_data, lang);
                loc.data.insert(lang, lang_data);
            }
            Err(e) => {
                warn!("Failed to load localization file {}: {e}", path.display());
            }
        }
    }
}

/// Populate the flat `strings` map from common dictionary / misc keys so that
/// the existing UI code (`loc.get("play_hand")` etc.) keeps working.
fn populate_flat_strings(
    strings: &mut HashMap<String, HashMap<Language, String>>,
    lang_data: &LanguageData,
    lang: Language,
) {
    // Map well-known misc.dictionary keys to the flat UI keys the rest of the
    // codebase already uses. Left = flat key expected by UI, Right = dictionary key.
    const DICT_MAPPINGS: &[(&str, &str)] = &[
        ("play_hand", "b_play_hand"),
        ("discard", "b_discard"),
        ("sort_hand", "b_sort_hand"),
        ("shop", "k_arcana_pack"), // fallback, but we also try direct
        ("reroll", "k_reroll"),
        ("next_round", "b_next_round_1"),
        ("new_run", "b_new_run"),
        ("settings", "b_settings"),
        ("quit", "b_quit_cap"),
        ("continue_run", "b_continue"),
        ("main_menu", "b_main_menu"),
        ("help", "b_options"),
        ("back", "b_back"),
        ("jokers", "b_stat_jokers"),
        ("deck", "k_deck"),
        ("ante", "k_ante"),
        ("round", "k_round"),
        ("money", "k_money"),
        ("blind", "k_locked"), // not a perfect match – see below
        ("hands", "k_hud_hands"),
        ("discards", "k_hud_discards"),
        ("score", "k_lower_score"),
        ("chips", "a_chips"),
        ("mult", "k_mult"),
        ("skip_blind", "b_skip_blind"),
        ("select_blind", "b_select"),
    ];

    // Direct dictionary entries first
    if let Some(dict) = lang_data.misc.get("dictionary") {
        // Insert every dictionary entry with its original key
        for (k, v) in dict {
            strings
                .entry(k.clone())
                .or_default()
                .insert(lang, v.as_str().to_string());
        }

        // Insert mapped aliases
        for &(flat_key, dict_key) in DICT_MAPPINGS {
            if let Some(v) = dict.get(dict_key) {
                strings
                    .entry(flat_key.to_string())
                    .or_default()
                    .insert(lang, v.as_str().to_string());
            }
        }
    }

    // Poker hand names from misc.poker_hands
    if let Some(hands) = lang_data.misc.get("poker_hands") {
        // Insert with original keys (e.g. "Flush Five")
        for (k, v) in hands {
            strings
                .entry(k.clone())
                .or_default()
                .insert(lang, v.as_str().to_string());
        }
        // Also insert snake_case aliases for backward compat
        const HAND_ALIASES: &[(&str, &str)] = &[
            ("high_card", "High Card"),
            ("pair", "Pair"),
            ("two_pair", "Two Pair"),
            ("three_of_a_kind", "Three of a Kind"),
            ("straight", "Straight"),
            ("flush", "Flush"),
            ("full_house", "Full House"),
            ("four_of_a_kind", "Four of a Kind"),
            ("straight_flush", "Straight Flush"),
            ("royal_flush", "Royal Flush"),
            ("five_of_a_kind", "Five of a Kind"),
            ("flush_house", "Flush House"),
            ("flush_five", "Flush Five"),
        ];
        for &(alias, poker_key) in HAND_ALIASES {
            if let Some(v) = hands.get(poker_key) {
                strings
                    .entry(alias.to_string())
                    .or_default()
                    .insert(lang, v.as_str().to_string());
            }
        }
    }

    // Labels
    if let Some(labels) = lang_data.misc.get("labels") {
        for (k, v) in labels {
            strings
                .entry(format!("label_{k}"))
                .or_default()
                .insert(lang, v.as_str().to_string());
        }
    }

    // Blind description names → flat keys like "small_blind", "big_blind", "boss_blind"
    if let Some(blinds) = lang_data.descriptions.get("Blind") {
        if let Some(e) = blinds.get("bl_small") {
            strings
                .entry("small_blind".to_string())
                .or_default()
                .insert(lang, e.name.clone());
        }
        if let Some(e) = blinds.get("bl_big") {
            strings
                .entry("big_blind".to_string())
                .or_default()
                .insert(lang, e.name.clone());
        }
        // "boss_blind" doesn't have a single entry – use a label if present
    }

    // Hard-coded keys that don't cleanly map from Lua
    const STATIC_KEYS: &[&str] = &[
        "title",
        "blind",
        "boss_blind",
        "game_over",
        "you_win",
        "hand_reference",
        "sort_rank",
        "sort_suit",
        "shop",
        "score",
        "chips",
    ];
    // Only fill these for English from Lua's dictionary when available,
    // otherwise they stay as whatever was previously set (or fallback to key).
    if lang == Language::English {
        let defaults: &[(&str, &str)] = &[
            ("title", "BALATRO"),
            ("blind", "Blind"),
            ("boss_blind", "Boss Blind"),
            ("game_over", "Game Over"),
            ("you_win", "You Win!"),
            ("hand_reference", "Hand Reference"),
            ("sort_rank", "Sort Rank"),
            ("sort_suit", "Sort Suit"),
            ("shop", "Shop"),
            ("score", "Score"),
            ("chips", "Chips"),
        ];
        for &(k, v) in defaults {
            strings.entry(k.to_string()).or_default().entry(lang).or_insert_with(|| v.to_string());
        }
    }

    // Ensure every STATIC_KEY at least has the English fallback for non-English
    // languages – the `get_for` method already falls back to English so this is
    // not strictly necessary but keeps the map consistent.
    let _ = STATIC_KEYS; // suppress unused warning
}

// ===========================================================================
// Simple Lua table parser
// ===========================================================================

/// Parse the entire contents of a Balatro `*.lua` localization file and return
/// a [`LanguageData`] with all descriptions and misc entries.
fn parse_lua(source: &str) -> LanguageData {
    let mut data = LanguageData::default();
    let mut parser = LuaParser::new(source);
    parser.parse(&mut data);
    data
}

// ---------------------------------------------------------------------------
// Parser internals
// ---------------------------------------------------------------------------

struct LuaParser<'a> {
    lines: Vec<&'a str>,
    pos: usize,
}

impl<'a> LuaParser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            lines: source.lines().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&'a str> {
        self.lines.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<&'a str> {
        let line = self.lines.get(self.pos).copied();
        self.pos += 1;
        line
    }

    /// Main entry: expects `return { ... }`.
    fn parse(&mut self, data: &mut LanguageData) {
        // Skip to the opening `{` of the returned table.
        while let Some(line) = self.advance() {
            let t = line.trim();
            if t.starts_with("return") && t.contains('{') {
                break;
            }
        }
        self.parse_root_table(data);
    }

    /// Inside the root table we expect `descriptions = { ... }`, `misc = { ... }`, etc.
    fn parse_root_table(&mut self, data: &mut LanguageData) {
        while let Some(line) = self.peek() {
            let t = line.trim();
            if t == "}" || t == "}," {
                self.advance();
                return;
            }

            if let Some(key) = Self::table_key(t) {
                self.advance(); // consume the line with `key = {`
                match key {
                    "descriptions" => self.parse_descriptions(&mut data.descriptions),
                    "misc" => self.parse_misc_sections(&mut data.misc),
                    // UI, tutorial at root level – we handle tutorial inside misc
                    _ => self.skip_table(),
                }
            } else {
                self.advance();
            }
        }
    }

    // -- descriptions ----------------------------------------------------------

    /// `descriptions = { Joker = { ... }, Tarot = { ... }, ... }`
    fn parse_descriptions(
        &mut self,
        descs: &mut HashMap<String, HashMap<String, DescriptionEntry>>,
    ) {
        while let Some(line) = self.peek() {
            let t = line.trim();
            if t == "}" || t == "}," {
                self.advance();
                return;
            }
            if let Some(category) = Self::table_key(t) {
                self.advance();
                let entries = self.parse_description_category();
                descs.insert(category.to_string(), entries);
            } else {
                self.advance();
            }
        }
    }

    /// Parse one category, e.g. `Joker = { j_joker = { name=..., text={...} }, ... }`.
    fn parse_description_category(&mut self) -> HashMap<String, DescriptionEntry> {
        let mut map = HashMap::new();
        while let Some(line) = self.peek() {
            let t = line.trim();
            if t == "}" || t == "}," {
                self.advance();
                return map;
            }
            if let Some(key) = Self::table_key(t) {
                self.advance();
                let entry = self.parse_description_entry();
                map.insert(key.to_string(), entry);
            } else {
                self.advance();
            }
        }
        map
    }

    /// Parse a single item: `{ name = "...", text = { "...", ... } }`.
    fn parse_description_entry(&mut self) -> DescriptionEntry {
        let mut entry = DescriptionEntry::default();
        while let Some(line) = self.peek() {
            let t = line.trim();
            if t == "}" || t == "}," {
                self.advance();
                return entry;
            }
            if t.starts_with("name") {
                if let Some(v) = Self::extract_string_value(t) {
                    entry.name = v.to_string();
                }
                self.advance();
            } else if t.starts_with("text") && t.contains('{') {
                self.advance();
                if t.contains('}') {
                    // Inline: `text = { "single line" }` or `text = {}`
                    entry.text = Self::extract_inline_strings(t);
                } else {
                    entry.text = self.parse_string_list();
                }
            } else {
                self.advance();
            }
        }
        entry
    }

    // -- misc ------------------------------------------------------------------

    /// `misc = { dictionary = { ... }, poker_hands = { ... }, ... }`
    fn parse_misc_sections(
        &mut self,
        misc: &mut HashMap<String, HashMap<String, MiscValue>>,
    ) {
        while let Some(line) = self.peek() {
            let t = line.trim();
            if t == "}" || t == "}," {
                self.advance();
                return;
            }
            if let Some(section_key) = Self::table_key(t) {
                self.advance();
                let entries = self.parse_misc_table();
                misc.insert(section_key.to_string(), entries);
            } else {
                self.advance();
            }
        }
    }

    /// Parse a misc sub-table whose values are either simple strings or string lists.
    fn parse_misc_table(&mut self) -> HashMap<String, MiscValue> {
        let mut map = HashMap::new();
        while let Some(line) = self.peek() {
            let t = line.trim();
            if t == "}" || t == "}," {
                self.advance();
                return map;
            }

            // `key = "value"` or `['key'] = "value"` or `key = { ... }`
            if let Some((key, rest)) = Self::kv_split(t) {
                let rest = rest.trim();
                if rest.starts_with('{') {
                    self.advance();
                    if rest.contains('}') {
                        let items = Self::extract_inline_strings(rest);
                        if items.len() == 1 {
                            map.insert(key.to_string(), MiscValue::Single(items.into_iter().next().unwrap()));
                        } else {
                            map.insert(key.to_string(), MiscValue::List(items));
                        }
                    } else {
                        // Multi-line table
                        let items = self.parse_string_list();
                        if items.len() == 1 {
                            map.insert(key.to_string(), MiscValue::Single(items.into_iter().next().unwrap()));
                        } else {
                            map.insert(key.to_string(), MiscValue::List(items));
                        }
                    }
                } else if let Some(v) = Self::extract_quoted(rest) {
                    map.insert(key.to_string(), MiscValue::Single(v.to_string()));
                } else {
                    // Bare value (shouldn't normally appear)
                    let clean = rest.trim_end_matches(',').trim().to_string();
                    if !clean.is_empty() {
                        map.insert(key.to_string(), MiscValue::Single(clean));
                    }
                    self.advance();
                    continue;
                }
            } else {
                self.advance();
                continue;
            }
        }
        map
    }

    // -- string list -----------------------------------------------------------

    /// Parse lines of `"string",` until the closing `}`.
    fn parse_string_list(&mut self) -> Vec<String> {
        let mut items = Vec::new();
        while let Some(line) = self.peek() {
            let t = line.trim();
            if t == "}" || t == "}," {
                self.advance();
                return items;
            }
            if let Some(s) = Self::extract_quoted(t) {
                items.push(s.to_string());
            }
            self.advance();
        }
        items
    }

    // -- skip ------------------------------------------------------------------

    /// Skip over a `{ ... }` block, handling nesting.
    fn skip_table(&mut self) {
        let mut depth: usize = 1;
        while let Some(line) = self.advance() {
            for ch in line.chars() {
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            return;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // -- helpers ---------------------------------------------------------------

    /// Detect `key = {` (opening a sub-table). Returns the key name.
    fn table_key(trimmed: &str) -> Option<&str> {
        // `Joker = {`  or  `j_joker = {`
        if !trimmed.ends_with('{') {
            return None;
        }
        let without_brace = trimmed[..trimmed.len() - 1].trim();
        if !without_brace.ends_with('=') {
            return None;
        }
        let key_part = without_brace[..without_brace.len() - 1].trim();
        // Handle ['quoted'] keys
        if key_part.starts_with("['") && key_part.ends_with("']") {
            return Some(&key_part[2..key_part.len() - 2]);
        }
        if key_part.chars().all(|c| c.is_alphanumeric() || c == '_') && !key_part.is_empty() {
            return Some(key_part);
        }
        None
    }

    /// Split `key = rest` or `['key'] = rest`. Returns `(key, rest)`.
    fn kv_split(trimmed: &str) -> Option<(&str, &str)> {
        let eq = trimmed.find('=')?;
        let raw_key = trimmed[..eq].trim();
        let rest = trimmed[eq + 1..].trim();

        let key = if raw_key.starts_with("['") && raw_key.ends_with("']") {
            &raw_key[2..raw_key.len() - 2]
        } else if raw_key.chars().all(|c| c.is_alphanumeric() || c == '_') && !raw_key.is_empty() {
            raw_key
        } else {
            return None;
        };
        Some((key, rest))
    }

    /// Extract value from `name = "Joker",` → `Joker`.
    fn extract_string_value(trimmed: &str) -> Option<&str> {
        let eq = trimmed.find('=')?;
        let after = trimmed[eq + 1..].trim();
        Self::extract_quoted(after)
    }

    /// Extract the first quoted string from a fragment, e.g. `"hello",` → `hello`.
    fn extract_quoted(s: &str) -> Option<&str> {
        let start = s.find('"')?;
        let rest = &s[start + 1..];
        let end = rest.find('"')?;
        Some(&rest[..end])
    }

    /// Extract all quoted strings from a single line like `{ "a", "b" }`.
    fn extract_inline_strings(line: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut remaining = line;
        while let Some(start) = remaining.find('"') {
            remaining = &remaining[start + 1..];
            if let Some(end) = remaining.find('"') {
                result.push(remaining[..end].to_string());
                remaining = &remaining[end + 1..];
            } else {
                break;
            }
        }
        result
    }
}
