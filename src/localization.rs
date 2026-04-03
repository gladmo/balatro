use bevy::prelude::*;
use std::collections::HashMap;

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Localization::default())
            .insert_resource(CurrentLanguage::default())
            .add_systems(Startup, setup_localization);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
    Japanese,
    Korean,
    French,
    German,
    Spanish,
    Portuguese,
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

#[derive(Resource, Default)]
pub struct CurrentLanguage {
    pub lang: Language,
}

#[derive(Resource)]
pub struct Localization {
    pub strings: HashMap<String, HashMap<Language, String>>,
}

impl Default for Localization {
    fn default() -> Self {
        Self {
            strings: HashMap::new(),
        }
    }
}

impl Localization {
    pub fn get(&self, key: &str) -> String {
        self.get_for(key, &Language::English)
    }

    pub fn get_for(&self, key: &str, lang: &Language) -> String {
        self.strings
            .get(key)
            .and_then(|map| map.get(lang))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    fn insert(&mut self, key: &str, en: &str, zh: &str) {
        let mut map = HashMap::new();
        map.insert(Language::English, en.to_string());
        map.insert(Language::Chinese, zh.to_string());
        self.strings.insert(key.to_string(), map);
    }
}

fn setup_localization(mut loc: ResMut<Localization>) {
    // UI strings
    loc.insert("title", "BALATRO", "巴拉特罗");
    loc.insert("new_run", "New Run", "新游戏");
    loc.insert("continue_run", "Continue", "继续");
    loc.insert("settings", "Settings", "设置");
    loc.insert("quit", "Quit", "退出");
    loc.insert("play_hand", "Play Hand", "出牌");
    loc.insert("discard", "Discard", "弃牌");
    loc.insert("sort_rank", "Sort Rank", "按点数排序");
    loc.insert("sort_suit", "Sort Suit", "按花色排序");
    loc.insert("hands", "Hands", "手牌次数");
    loc.insert("discards", "Discards", "弃牌次数");
    loc.insert("money", "Money", "金钱");
    loc.insert("ante", "Ante", "底注");
    loc.insert("round", "Round", "回合");
    loc.insert("score", "Score", "得分");
    loc.insert("chips", "Chips", "筹码");
    loc.insert("mult", "Mult", "倍率");
    loc.insert("blind", "Blind", "盲注");
    loc.insert("small_blind", "Small Blind", "小盲注");
    loc.insert("big_blind", "Big Blind", "大盲注");
    loc.insert("boss_blind", "Boss Blind", "Boss盲注");
    loc.insert("select_blind", "Select Blind", "选择盲注");
    loc.insert("skip_blind", "Skip", "跳过");
    loc.insert("shop", "Shop", "商店");
    loc.insert("next_round", "Next Round", "下一回合");
    loc.insert("reroll", "Reroll", "刷新");
    loc.insert("game_over", "Game Over", "游戏结束");
    loc.insert("you_win", "You Win!", "你赢了！");
    loc.insert("main_menu", "Main Menu", "主菜单");
    loc.insert("hand_reference", "Hand Reference", "牌型参考");
    loc.insert("help", "Help", "帮助");
    loc.insert("back", "Back", "返回");
    loc.insert("jokers", "Jokers", "小丑牌");
    loc.insert("deck", "Deck", "牌组");

    // Hand type names
    loc.insert("high_card", "High Card", "高牌");
    loc.insert("pair", "Pair", "对子");
    loc.insert("two_pair", "Two Pair", "两对");
    loc.insert("three_of_a_kind", "Three of a Kind", "三条");
    loc.insert("straight", "Straight", "顺子");
    loc.insert("flush", "Flush", "同花");
    loc.insert("full_house", "Full House", "葫芦");
    loc.insert("four_of_a_kind", "Four of a Kind", "四条");
    loc.insert("straight_flush", "Straight Flush", "同花顺");
    loc.insert("royal_flush", "Royal Flush", "皇家同花顺");
    loc.insert("five_of_a_kind", "Five of a Kind", "五条");
    loc.insert("flush_house", "Flush House", "同花葫芦");
    loc.insert("flush_five", "Flush Five", "同花五条");
}
