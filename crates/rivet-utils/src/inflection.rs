use regex::Regex;
use std::iter::Iterator;

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
pub const PLURALIZE_RULES: &[(&str, &str)] = &[
    (r"(?i)(quiz)$", r"${1}zes"),
    (r"(?i)^(ox)$", r"${1}en"),
    (r"(?i)([ml])ouse$", r"${1}ice"), // 修复了正则表达式
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
];

/// 单数化规则列表。
/// List of singularization rules.
const SINGULARIZE_RULES: &[(&str, &str)] = &[
    (r"(?i)(quiz)zes$", r"${1}"),
    (r"(?i)(matr)ices$", r"${1}ix"),
    (r"(?i)(vert|ind)ices$", r"${1}ex"),
    (r"(?i)^(ox)en", r"${1}"),
    (r"(?i)(alias|status)es$", r"${1}"),
    (r"(?i)([octopvir])i$", r"${1}us"), // 修复了正则表达式
    (r"(?i)(cris|ax|test)es$", r"${1}is"),
    (r"(?i)(shoe)s$", r"${1}"),
    (r"(?i)(o)es$", r"${1}"),
    (r"(?i)(bus)es$", r"${1}"),
    (r"(?i)([ml])ice$", r"${1}ouse"), // 修复了正则表达式
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
];

/// 判断单词是否为不可数名词。
/// Determines if the word is an uncountable noun.
pub fn is_uncountable(word: &str) -> bool {
    let lower_cased_word = word.to_lowercase();
    UNCOUNTABLE_WORDS
        .iter()
        .any(|&uncountable_word| lower_cased_word.ends_with(uncountable_word))
}

/// 处理不规则词。
/// Handles irregular words.
pub fn irregular(word: &str, irregular_words: &[(&str, &str)]) -> Option<String> {
    for &(irregular, dest) in irregular_words {
        if let Ok(re) = Regex::new(&format!(r"(?i){}$", regex::escape(irregular))) {
            if let Some(captures) = re.captures(word) {
                if let Some(_) = captures.get(0) {
                    // 使用完整的单词替换，而不是复杂的字符串拼接
                    return Some(re.replace(word, dest).to_string());
                }
            }
        }
    }
    None
}

/// 核心处理函数，应用规则进行转换。
/// Core function to apply rules and perform the transformation.
pub fn core_deal(word: &str, rules: &[(&str, &str)]) -> String {
    for &(pattern, replacement) in rules {
        if let Ok(re) = Regex::new(pattern) {
            if re.is_match(word) {
                return re.replace_all(word, replacement).to_string();
            }
        }
    }
    word.to_string()
}

/// 将单词转换为单数形式。
/// Converts a word to its singular form.
pub fn singularize(word: &str) -> String {
    singularize_or_pluralize(word, SINGULARIZE_RULES, SINGULARIZE_IRREGULAR_WORDS)
}

/// 将单词转换为复数形式。
/// Converts a word to its plural form.
pub fn pluralize(word: &str) -> String {
    singularize_or_pluralize(word, PLURALIZE_RULES, PLURALIZE_IRREGULAR_WORDS)
}

/// 根据规则和不规则词列表将单词转换为单数或复数形式。
/// Converts a word to its singular or plural form based on the rules and irregular words list.
fn singularize_or_pluralize(
    word: &str,
    rules: &[(&str, &str)],
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

/// 判断字符是否为字母。
/// Determines if the character is a letter.
fn is_letter(ch: char) -> bool {
    ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch)
}

/// 判断字符是否为字母或数字。
/// Determines if the character is a letter or digit.
fn is_char(ch: char) -> bool {
    ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ('0'..='9').contains(&ch)
}

/// 将名称转换为PascalCase格式。
/// Converts a name to PascalCase format.
pub fn pascal_case_of(name: &str) -> String {
    let name = snake_case_of(name);
    let pos = name.chars().position(|ch| is_letter(ch)).unwrap_or(0);

    let mut java_name = Vec::new();
    let mut upper_flag = true;

    for ch in name.chars().skip(pos) {
        if upper_flag {
            java_name.push(ch.to_ascii_uppercase());
            upper_flag = false;
        } else if is_char(ch) {
            java_name.push(ch);
        } else {
            upper_flag = true;
        }
    }

    java_name.iter().collect()
}

/// 将名称转换为snake_case格式。
/// Converts a name to snake_case format.
pub fn snake_case_of(name: &str) -> String {
    let mut python_name = Vec::new();
    let mut last_uppercase_idx: i32 = -1;
    for (i, ch) in name.chars().enumerate() {
        let i = i as i32;
        if i == 0 {
            // 第一个字符
            if ch.is_uppercase() {
                last_uppercase_idx = i;
            }
            python_name.push(ch.to_ascii_lowercase());
        } else if ch.is_uppercase() {
            // 如果是大写字母，在前面添加下划线（如果不是最后一个字符是下划线）
            if !python_name.is_empty()
                && *python_name.last().unwrap() != '_'
                && last_uppercase_idx + 1 != i
            {
                python_name.push('_');
            }
            python_name.push(ch.to_ascii_lowercase());
            last_uppercase_idx = i;
        } else if ch == '_' {
            // 如果是下划线，确保不会重复添加
            if !python_name.is_empty() && *python_name.last().unwrap() != '_' {
                python_name.push('_');
            }
        } else {
            python_name.push(ch.to_ascii_lowercase());
        }
    }

    python_name.iter().collect()
}

/// 生成表名。
/// Generates a table name.
pub fn table_name_of(name: &str) -> String {
    pluralize(&snake_case_of(name))
}

#[cfg(test)]
#[path = "inflection_test.rs"]
mod inflection_test;
