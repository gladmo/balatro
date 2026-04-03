#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use std::collections::HashMap;

/// Supported UI languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Resource)]
pub enum Language {
    #[default]
    English,
    Chinese,
}

/// The active language setting used at runtime.
pub type CurrentLanguage = Language;

/// All localized strings for a single language.
pub struct LanguageStrings {
    strings: HashMap<&'static str, &'static str>,
}

impl LanguageStrings {
    fn new(pairs: &[(&'static str, &'static str)]) -> Self {
        let mut strings = HashMap::new();
        for (k, v) in pairs {
            strings.insert(*k, *v);
        }
        LanguageStrings { strings }
    }

    fn get(&self, key: &str) -> Option<&'static str> {
        self.strings.get(key).copied()
    }
}

/// Runtime localization resource — holds all language string tables and the
/// currently active language.  Call `set_language()` to switch at runtime.
#[derive(Resource)]
pub struct Localization {
    lang: Language,
    tables: HashMap<Language, LanguageStrings>,
}

fn english_strings() -> LanguageStrings {
    LanguageStrings::new(&[
        // Main menu
        ("menu.title",        "BALATRO"),
        ("menu.new_run",      "New Run"),
        ("menu.continue",     "Continue"),
        ("menu.quit",         "Quit"),
        ("menu.language",     "Language"),
        ("menu.help",         "How to Play"),
        // Game UI
        ("ui.score",          "Score"),
        ("ui.target",         "Target"),
        ("ui.money",          "Money"),
        ("ui.hands",          "Hands"),
        ("ui.discards",       "Discards"),
        ("ui.ante",           "Ante"),
        ("ui.round",          "Round"),
        ("ui.play_hand",      "Play Hand"),
        ("ui.discard",        "Discard"),
        ("ui.jokers",         "Jokers"),
        ("ui.score_vs_target","Score / Target"),
        ("ui.hand_type",      "Hand Type"),
        ("ui.select_cards",   "Select cards to play"),
        // Blinds
        ("blind.small",       "Small Blind"),
        ("blind.big",         "Big Blind"),
        ("blind.boss",        "BOSS BLIND"),
        ("blind.select",      "Select Blind"),
        ("blind.skip",        "Skip"),
        ("blind.play",        "Play"),
        ("blind.reward",      "Reward"),
        ("blind.face_boss",   "Face Boss"),
        ("blind.skip_hint",   "Skip Small/Big Blind to proceed to the next blind"),
        // Shop
        ("shop.title",        "Shop"),
        ("shop.buy",          "Buy"),
        ("shop.sell",         "Sell"),
        ("shop.reroll",       "Reroll ($5)"),
        ("shop.reroll_btn",   "Reroll"),
        ("shop.continue",     "Next Round"),
        ("shop.money",        "$"),
        ("shop.jokers",       "Jokers"),
        ("shop.consumables",  "Consumables"),
        ("shop.for_sale",     "For Sale"),
        ("shop.your_jokers",  "Your Jokers (sell):"),
        ("shop.sell_btn",     "Sell"),
        ("shop.money_label",  "Money"),
        ("shop.interest_label", "Interest"),
        ("shop.beat_blind",   "Beat"),
        ("shop.earned",       "Earned"),
        // Game over / victory
        ("game_over.title",   "Game Over"),
        ("game_over.score",   "Final Score"),
        ("game_over.reached_ante", "Reached Ante"),
        ("game_over.play_again", "Play Again"),
        ("game_over.menu",    "Main Menu"),
        ("victory.title",     "Victory!"),
        ("victory.message",   "You beat all 8 antes!"),
        // Main menu
        ("menu.subtitle",     "A Poker Roguelite"),
        // Hand types
        ("hand.high_card",    "High Card"),
        ("hand.pair",         "Pair"),
        ("hand.two_pair",     "Two Pair"),
        ("hand.three_of_a_kind", "Three of a Kind"),
        ("hand.straight",     "Straight"),
        ("hand.flush",        "Flush"),
        ("hand.full_house",   "Full House"),
        ("hand.four_of_a_kind","Four of a Kind"),
        ("hand.straight_flush","Straight Flush"),
        ("hand.five_of_a_kind","Five of a Kind"),
        ("hand.flush_house",  "Flush House"),
        ("hand.flush_five",   "Flush Five"),
        // Help screen
        ("help.title",        "How to Play"),
        ("help.scoring",      "Scoring: Chips × Mult"),
        ("help.formula",      "Score = (Base Chips + Card Chips) × (Base Mult + Bonuses)"),
        ("help.formula_note", "Each played card adds its chip value.\nJokers add Chips, Mult, or ×Mult."),
        ("help.hand_types_title", "Hand Types  (Base Chips × Base Mult)"),
        ("help.col_hand",     "Hand"),
        ("help.col_chips",    "Chips"),
        ("help.col_mult",     "Mult"),
        ("help.editions_title",     "Card Editions"),
        ("help.edition.foil",       "Foil  →  +50 Chips"),
        ("help.edition.holo",       "Holographic  →  +10 Mult"),
        ("help.edition.poly",       "Polychrome  →  ×1.5 Mult"),
        ("help.edition.neg",        "Negative  →  +1 Joker slot"),
        ("help.enhancements_title", "Card Enhancements"),
        ("help.enhance.bonus",      "Bonus Card  →  +30 Chips"),
        ("help.enhance.mult",       "Mult Card  →  +4 Mult"),
        ("help.enhance.wild",       "Wild Card  →  Any suit"),
        ("help.enhance.glass",      "Glass Card  →  ×2 Mult (breaks)"),
        ("help.enhance.steel",      "Steel Card  →  ×1.5 Mult in hand"),
        ("help.enhance.stone",      "Stone Card  →  +50 Chips"),
        ("help.enhance.gold",       "Gold Card  →  +$3/round"),
        ("help.enhance.lucky",      "Lucky Card  →  1-in-5: +20 Mult"),
        ("help.seals_title",        "Card Seals"),
        ("help.seal.gold",          "Gold Seal  →  +$3 when scored"),
        ("help.seal.red",           "Red Seal  →  Retrigger once"),
        ("help.seal.blue",          "Blue Seal  →  Create Planet"),
        ("help.seal.purple",        "Purple Seal  →  Create Tarot"),
        ("help.editions",     "Card Editions: Foil +50 chips | Holo +10 mult | Poly ×1.5 mult"),
        ("help.enhancements", "Enhancements: Bonus +30ch | Mult +4× | Wild=any suit | Glass ×2mult"),
        ("help.close",        "Close"),
        // Tooltip
        ("tooltip.chips",     "Chips"),
        ("tooltip.enhancement","Enhancement"),
        ("tooltip.edition",   "Edition"),
        ("tooltip.seal",      "Seal"),
    ])
}

fn chinese_strings() -> LanguageStrings {
    LanguageStrings::new(&[
        // 主菜单
        ("menu.title",        "BALATRO"),
        ("menu.new_run",      "开始新局"),
        ("menu.continue",     "继续游戏"),
        ("menu.quit",         "退出"),
        ("menu.language",     "语言"),
        ("menu.help",         "游戏说明"),
        // 游戏界面
        ("ui.score",          "得分"),
        ("ui.target",         "目标"),
        ("ui.money",          "金钱"),
        ("ui.hands",          "手牌次数"),
        ("ui.discards",       "弃牌次数"),
        ("ui.ante",           "前注"),
        ("ui.round",          "回合"),
        ("ui.play_hand",      "出牌"),
        ("ui.discard",        "弃牌"),
        ("ui.jokers",         "小丑牌"),
        ("ui.score_vs_target","得分 / 目标"),
        ("ui.hand_type",      "牌型"),
        ("ui.select_cards",   "选择要出的牌"),
        // 盲注
        ("blind.small",       "小盲注"),
        ("blind.big",         "大盲注"),
        ("blind.boss",        "Boss盲注"),
        ("blind.select",      "选择盲注"),
        ("blind.skip",        "跳过"),
        ("blind.play",        "出牌"),
        ("blind.reward",      "奖励"),
        ("blind.face_boss",   "迎战Boss"),
        ("blind.skip_hint",   "跳过小/大盲注可直接进入下一关"),
        // 商店
        ("shop.title",        "商店"),
        ("shop.buy",          "购买"),
        ("shop.sell",         "出售"),
        ("shop.reroll",       "重掷($5)"),
        ("shop.reroll_btn",   "重掷"),
        ("shop.continue",     "下一回合"),
        ("shop.money",        "$"),
        ("shop.jokers",       "小丑牌"),
        ("shop.consumables",  "消耗品"),
        ("shop.for_sale",     "在售"),
        ("shop.your_jokers",  "我的小丑牌（出售）："),
        ("shop.sell_btn",     "出售"),
        ("shop.money_label",  "金钱"),
        ("shop.interest_label", "利息"),
        ("shop.beat_blind",   "打败"),
        ("shop.earned",       "获得"),
        // 游戏结束 / 胜利
        ("game_over.title",   "游戏结束"),
        ("game_over.score",   "最终得分"),
        ("game_over.reached_ante", "到达前注"),
        ("game_over.play_again", "再玩一局"),
        ("game_over.menu",    "主菜单"),
        ("victory.title",     "胜利！"),
        ("victory.message",   "你打败了所有8个前注！"),
        // 主菜单
        ("menu.subtitle",     "扑克Roguelite"),
        // 牌型
        ("hand.high_card",    "高牌"),
        ("hand.pair",         "一对"),
        ("hand.two_pair",     "两对"),
        ("hand.three_of_a_kind", "三条"),
        ("hand.straight",     "顺子"),
        ("hand.flush",        "同花"),
        ("hand.full_house",   "葫芦"),
        ("hand.four_of_a_kind","四条"),
        ("hand.straight_flush","同花顺"),
        ("hand.five_of_a_kind","五条"),
        ("hand.flush_house",  "同花葫芦"),
        ("hand.flush_five",   "同花五条"),
        // 帮助界面
        ("help.title",        "游戏说明"),
        ("help.scoring",      "计分方式：筹码 × 倍数"),
        ("help.formula",      "得分 = (基础筹码 + 卡牌筹码) × (基础倍数 + 加成)"),
        ("help.formula_note", "每张出牌将其筹码值加入筹码池。\n小丑牌可增加筹码、倍数或×倍数。"),
        ("help.hand_types_title", "牌型 (基础筹码 × 基础倍数)"),
        ("help.col_hand",     "牌型"),
        ("help.col_chips",    "筹码"),
        ("help.col_mult",     "倍数"),
        ("help.editions_title",     "卡牌版本"),
        ("help.edition.foil",       "闪光  →  +50筹码"),
        ("help.edition.holo",       "全息  →  +10倍数"),
        ("help.edition.poly",       "多彩  →  ×1.5倍数"),
        ("help.edition.neg",        "负片  →  +1小丑槽"),
        ("help.enhancements_title", "卡牌增强"),
        ("help.enhance.bonus",      "加成牌  →  +30筹码"),
        ("help.enhance.mult",       "倍数牌  →  +4倍数"),
        ("help.enhance.wild",       "万能牌  →  任意花色"),
        ("help.enhance.glass",      "玻璃牌  →  ×2倍数(会破碎)"),
        ("help.enhance.steel",      "钢铁牌  →  手中×1.5倍数"),
        ("help.enhance.stone",      "石头牌  →  +50筹码"),
        ("help.enhance.gold",       "黄金牌  →  回合结束+$3"),
        ("help.enhance.lucky",      "幸运牌  →  1/5概率+20倍数"),
        ("help.seals_title",        "卡牌印章"),
        ("help.seal.gold",          "金印章  →  计分时+$3"),
        ("help.seal.red",           "红印章  →  再次触发"),
        ("help.seal.blue",          "蓝印章  →  创建星球牌"),
        ("help.seal.purple",        "紫印章  →  弃牌时创建塔罗"),
        ("help.editions",     "版本加成：闪光+50筹 | 全息+10倍 | 多彩×1.5倍"),
        ("help.enhancements", "增强效果：加成牌+30筹 | 倍数牌+4倍 | 万能牌=任意花色 | 玻璃牌×2倍"),
        ("help.close",        "关闭"),
        // 悬浮提示
        ("tooltip.chips",     "筹码"),
        ("tooltip.enhancement","增强"),
        ("tooltip.edition",   "版本"),
        ("tooltip.seal",      "印章"),
    ])
}

impl Default for Localization {
    fn default() -> Self {
        let mut tables = HashMap::new();
        tables.insert(Language::English, english_strings());
        tables.insert(Language::Chinese, chinese_strings());
        Localization { lang: Language::English, tables }
    }
}

impl Localization {
    /// Look up a localized string by key, falling back to English.
    /// Returns the string value (always `'static` from the tables).
    /// If the key is not found in any table, returns `"?"` as a sentinel.
    pub fn get(&self, key: &str) -> &'static str {
        if let Some(table) = self.tables.get(&self.lang) {
            if let Some(s) = table.get(key) {
                return s;
            }
        }
        // Fall back to English
        if let Some(table) = self.tables.get(&Language::English) {
            if let Some(s) = table.get(key) {
                return s;
            }
        }
        "?"
    }

    pub fn language(&self) -> Language {
        self.lang
    }

    pub fn set_language(&mut self, lang: Language) {
        self.lang = lang;
    }

    pub fn toggle_language(&mut self) {
        self.lang = match self.lang {
            Language::English => Language::Chinese,
            Language::Chinese => Language::English,
        };
    }
}

/// Convenience: get a localized string as an owned String.
pub fn loc(localization: &Localization, key: &str) -> String {
    localization.get(key).to_string()
}
