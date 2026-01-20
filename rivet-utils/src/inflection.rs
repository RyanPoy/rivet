use regex::Regex;
use std::iter::Iterator;
use std::sync::LazyLock;

/// 不可数名词列表。
/// List of uncountable words.
pub const UNCOUNTABLE_WORDS: &[&str] = &[
    "equipment",
    "information",
    "rice",
    "money",
    "species",
    "series",
    "fish",
    "sheep",
    "sms",
];

/// 不规则复数词列表。
/// List of irregular plural words.
pub const PLURALIZE_IRREGULAR_WORDS: &[(&str, &str)] = &[
    ("person", "people"),
    ("man", "men"),
    ("child", "children"),
    ("sex", "sexes"),
    ("move", "moves"),
];
/// 不规则单数词列表。
/// List of irregular singular words.
pub const SINGULARIZE_IRREGULAR_WORDS: &[(&str, &str)] = &[
    ("people", "person"),
    ("men", "man"),
    ("children", "child"),
    ("sexes", "sex"),
    ("moves", "move"),
];

/// 复数化规则列表。
/// List of pluralization rules.
static PLURALIZE_RULES: LazyLock<Vec<(Regex, &str)>> = LazyLock::new(|| {
    vec![
        (r"(?i)(quiz)$", r"${1}zes"),
        (r"(?i)^(ox)$", r"${1}en"),
        (r"(?i)([ml])ouse$", r"${1}ice"),
        (r"(?i)(matr|vert|ind)ix|ex$", r"${1}ices"),
        (r"(?i)(x|ch|ss|sh)$", r"${1}es"),
        (r"(?i)([^aeiouy]|qu)ies$", r"${1}y"),
        (r"(?i)([^aeiouy]|qu)y$", r"${1}ies"),
        (r"(?i)(hive)$", r"${1}s"),
        (r"(?i)(?:([^f])fe|([lr])f)$", r"${1}${2}ves"),
        (r"(?i)sis$", "ses"),
        (r"(?i)([ti])um$", r"${1}a"),
        (r"(?i)(buffal|tomat)o$", r"${1}oes"),
        (r"(?i)(bu)s$", r"${1}ses"),
        (r"(?i)(alias|status)", r"${1}es"),
        (r"(?i)(octop|vir)us$", r"${1}i"),
        (r"(?i)(ax|test)is$", r"${1}es"),
        (r"(?i)s$", "s"),
        (r"(?i)$", "s"),
    ]
    .into_iter()
    .map(|(pattern, replacement)| {
        (
            Regex::new(pattern).expect(&format!("Static regex patter invalid: {}", pattern)),
            replacement,
        )
    })
    .collect()
});

/// 单数化规则列表。
/// List of singularization rules.
// const SINGULARIZE_RULES: &[(&str, &str)] = &[
static SINGULARIZE_RULES: LazyLock<Vec<(Regex, &str)>> = LazyLock::new(|| {
    vec![
        (r"(?i)(quiz)zes$", r"${1}"),
        (r"(?i)(matr)ices$", r"${1}ix"),
        (r"(?i)(vert|ind)ices$", r"${1}ex"),
        (r"(?i)^(ox)en", r"${1}"),
        (r"(?i)(alias|status)es$", r"${1}"),
        (r"(?i)([octopvir])i$", r"${1}us"),
        (r"(?i)(cris|ax|test)es$", r"${1}is"),
        (r"(?i)(shoe)s$", r"${1}"),
        (r"(?i)(o)es$", r"${1}"),
        (r"(?i)(bus)es$", r"${1}"),
        (r"(?i)([ml])ice$", r"${1}ouse"),
        (r"(?i)(x|ch|ss|sh)es$", r"${1}"),
        (r"(?i)(m)ovies$", r"${1}ovie"),
        (r"(?i)(s)eries$", r"${1}eries"),
        (r"(?i)([^aeiouy]|qu)ies$", r"${1}y"),
        (r"(?i)([lr])ves$", r"${1}f"),
        (r"(?i)(tive)s$", r"${1}"),
        (r"(?i)(hive)s$", r"${1}"),
        (r"(?i)([^f])ves$", r"${1}fe"),
        (r"(?i)(^analy)ses$", r"${1}sis"),
        (
            r"(?i)((a)naly|(b)a|(d)iagno|(p)arenthe|(p)rogno|(s)ynop|(t)he)ses$",
            r"${1}${2}sis",
        ),
        (r"(?i)([ti])a$", r"${1}um"),
        (r"(?i)(n)ews$", r"${1}ews"),
        (r"(?i)s$", ""),
    ]
    .into_iter()
    .map(|(pattern, replacement)| {
        (
            Regex::new(pattern).expect(&format!("Static regex pattern is invalid: {}", pattern)),
            replacement,
        )
    })
    .collect()
});

/// 判断单词是否为不可数名词。
/// Determines if the word is an uncountable noun.
pub fn is_uncountable(word: &str) -> bool {
    UNCOUNTABLE_WORDS
        .iter()
        .any(|&u| word.len() >= u.len() && word[word.len() - u.len()..].eq_ignore_ascii_case(u))
}

/// 处理不规则词。
/// Handles irregular words.
pub fn irregular(word: &str, irregular_words: &[(&str, &str)]) -> Option<String> {
    let word_lower = word.to_lowercase(); // 仅一次小写化，用于匹配

    for &(irr, dest) in irregular_words {
        // irr 是单数/复数原形，如 "person" 或 "people"
        if word_lower.ends_with(irr) {
            let prefix_len = word.len() - irr.len();
            let mut result = String::with_capacity(prefix_len + dest.len());

            // 保持原词的前缀（保留大小写），只替换后缀
            result.push_str(&word[..prefix_len]);
            result.push_str(dest);
            return Some(result);
        }
    }
    None
}

/// 核心处理函数，应用规则进行转换。
/// Core function to apply rules and perform the transformation.
pub fn core_deal(word: &str, rules: &[(Regex, &str)]) -> String {
    for (re, replacement) in rules.iter() {
        if re.is_match(word) {
            return re.replace_all(word, *replacement).to_string();
        }
    }
    word.to_string()
}

/// 将单词转换为单数形式。
/// Converts a word to its singular form.
pub fn singularize(word: &str) -> String {
    singularize_or_pluralize(word, &SINGULARIZE_RULES, SINGULARIZE_IRREGULAR_WORDS)
}

/// 将单词转换为复数形式。
/// Converts a word to its plural form.
pub fn pluralize(word: &str) -> String {
    singularize_or_pluralize(word, &PLURALIZE_RULES, PLURALIZE_IRREGULAR_WORDS)
}

/// 根据规则和不规则词列表将单词转换为单数或复数形式。
/// Converts a word to its singular or plural form based on the rules and irregular words list.
fn singularize_or_pluralize(
    word: &str,
    rules: &[(Regex, &str)],
    irregular_words: &[(&str, &str)],
) -> String {
    if is_uncountable(word) {
        return word.to_string();
    }

    if let Some(result) = irregular(word, irregular_words) {
        return result;
    }

    core_deal(word, rules)
}

/// 将名称转换为PascalCase格式。
/// Converts a name to PascalCase format.
pub fn pascal_case_of(name: &str) -> String {
    let name = snake_case_of(name);
    let mut pascal_name = String::with_capacity(name.len());
    let mut upper_flag = true;
    let mut start = false;

    for ch in name.chars() {
        if ch.is_ascii_alphabetic() {
            start = true;
        }
        if start {
            if upper_flag {
                pascal_name.push(ch.to_ascii_uppercase());
                upper_flag = false;
            } else if ch.is_ascii_alphabetic() {
                pascal_name.push(ch);
            } else {
                upper_flag = true;
            }
        }
    }
    pascal_name
}

/// 将名称转换为snake_case格式。
/// Converts a name to snake_case format.
pub fn snake_case_of(name: &str) -> String {
    let mut snake_name = String::with_capacity(name.len() + 2);
    let mut chars = name.chars().peekable();
    let mut last_char: Option<char> = None;

    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            if let Some(prev) = last_char {
                // 核心判定条件：
                // 1. 前一个是小写，当前是大写 (aB -> a_b)
                // 2. 当前是大写，下一个是小写 (XPa -> xp_a) —— 处理缩写结尾
                if prev.is_lowercase() || chars.peek().map_or(false, |c| c.is_lowercase()) {
                    if !snake_name.ends_with('_') {
                        snake_name.push('_');
                    }
                }
            }
            snake_name.push(ch.to_ascii_lowercase());
        } else {
            if ch == '_' {
                if !snake_name.ends_with('_') {
                    snake_name.push('_');
                }
            } else {
                snake_name.push(ch);
            }
        }
        last_char = Some(ch);
    }
    snake_name
}

/// 生成表名。
/// Generates a table name.
pub fn table_name_of(name: &str) -> String {
    pluralize(&snake_case_of(name))
}

#[cfg(test)]
#[path = "inflection_test.rs"]
mod inflection_test;
