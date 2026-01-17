use super::inflection::*;

#[cfg(test)]
const CASES: [(&str, &str); 75] = [
    ("search", "searches"),
    ("switch", "switches"),
    ("fix", "fixes"),
    ("box", "boxes"),
    ("process", "processes"),
    ("address", "addresses"),
    ("case", "cases"),
    ("stack", "stacks"),
    ("wish", "wishes"),
    ("fish", "fish"),
    ("category", "categories"),
    ("query", "queries"),
    ("ability", "abilities"),
    ("agency", "agencies"),
    ("movie", "movies"),
    ("archive", "archives"),
    ("index", "indices"),
    ("wife", "wives"),
    ("safe", "saves"),
    ("half", "halves"),
    ("move", "moves"),
    ("salesperson", "salespeople"),
    ("person", "people"),
    ("spokesman", "spokesmen"),
    ("man", "men"),
    ("woman", "women"),
    ("basis", "bases"),
    ("diagnosis", "diagnoses"),
    ("datum", "data"),
    ("medium", "media"),
    ("analysis", "analyses"),
    ("node_child", "node_children"),
    ("child", "children"),
    ("experience", "experiences"),
    ("day", "days"),
    ("comment", "comments"),
    ("foobar", "foobars"),
    ("newsletter", "newsletters"),
    ("old_news", "old_news"),
    ("news", "news"),
    ("series", "series"),
    ("species", "species"),
    ("quiz", "quizzes"),
    ("perspective", "perspectives"),
    ("ox", "oxen"),
    ("photo", "photos"),
    ("buffalo", "buffaloes"),
    ("tomato", "tomatoes"),
    ("dwarf", "dwarves"),
    ("elf", "elves"),
    ("information", "information"),
    ("equipment", "equipment"),
    ("bus", "buses"),
    ("status", "statuses"),
    ("mouse", "mice"),
    ("louse", "lice"),
    ("house", "houses"),
    ("octopus", "octopi"),
    ("virus", "viri"),
    ("alias", "aliases"),
    ("portfolio", "portfolios"),
    ("vertex", "vertices"),
    ("matrix", "matrices"),
    ("axis", "axes"),
    ("testis", "testes"),
    ("crisis", "crises"),
    ("rice", "rice"),
    ("shoe", "shoes"),
    ("horse", "horses"),
    ("prize", "prizes"),
    ("edge", "edges"),
    ("person", "people"),
    ("student_and_teacher", "student_and_teachers"),
    ("money", "money"),
    ("pretty_fish", "pretty_fish"),
];

#[test]
fn test_pluralize() {
    for (singular, plural) in CASES {
        assert_eq!(pluralize(singular), String::from(plural))
    }
}

#[test]
fn test_singularize() {
    for (singular, plural) in CASES {
        assert_eq!(singularize(plural), String::from(singular))
    }
}

#[test]
fn test_pascal_case() {
    assert_eq!(String::from("User"), pascal_case_of("User"));
    assert_eq!(
        String::from("TestModelForTableName"),
        pascal_case_of("TestModelForTableName")
    );
    assert_eq!(
        String::from("TestModelForTableName"),
        pascal_case_of("Test_model_for_Table_Name")
    );
    assert_eq!(
        String::from("TestModelForTableName"),
        pascal_case_of("Test_model_forTableName")
    );
    assert_eq!(
        String::from("TestModelForTableName"),
        pascal_case_of("_Test_model_forTableName")
    );
    assert_eq!(
        String::from("TestModelForTableName"),
        pascal_case_of("1002_test_model_forTableName")
    );
}

#[test]
fn test_snake_case() {
    assert_eq!(String::from("user"), snake_case_of("User"));
    assert_eq!(
        String::from("test_model_for_table_name"),
        snake_case_of("TestModelForTableName")
    );
    assert_eq!(
        String::from("test_model_for_table_name"),
        snake_case_of("Test_model_for_Table_Name")
    );
    assert_eq!(
        String::from("test_model_for_table_name"),
        snake_case_of("Test_model_forTableName")
    );
    assert_eq!(String::from("create_user"), snake_case_of("CreateUser"));
}

#[test]
fn test_is_uncountable() {
    assert!(is_uncountable("fish"));
    assert!(is_uncountable("sheep"));
    assert!(is_uncountable("equipment"));
    assert!(is_uncountable("information"));
    assert!(is_uncountable("rice"));
    assert!(is_uncountable("money"));
    assert!(is_uncountable("species"));
    assert!(is_uncountable("series"));
    assert!(is_uncountable("sms"));
    assert!(!is_uncountable("cat"));
    assert!(!is_uncountable("dog"));
    assert!(!is_uncountable("person"));
}

#[test]
fn test_irregular() {
    assert_eq!(
        irregular("person", PLURALIZE_IRREGULAR_WORDS),
        Some("people".to_string())
    );
    assert_eq!(
        irregular("man", PLURALIZE_IRREGULAR_WORDS),
        Some("men".to_string())
    );
    assert_eq!(
        irregular("child", PLURALIZE_IRREGULAR_WORDS),
        Some("children".to_string())
    );
    assert_eq!(irregular("cat", PLURALIZE_IRREGULAR_WORDS), None);
    assert_eq!(irregular("dog", SINGULARIZE_IRREGULAR_WORDS), None);
}

#[test]
fn test_core_deal() {
    assert_eq!(core_deal("cat", PLURALIZE_RULES), String::from("cats"));
    assert_eq!(core_deal("box", PLURALIZE_RULES), String::from("boxes"));
    assert_eq!(core_deal("quiz", PLURALIZE_RULES), String::from("quizzes"));
    assert_eq!(core_deal("ox", PLURALIZE_RULES), String::from("oxen"));
}

#[test]
fn test_table_name_of() {
    assert_eq!(table_name_of("User"), String::from("users"));
    assert_eq!(table_name_of("Category"), String::from("categories"));
    assert_eq!(table_name_of("Person"), String::from("people")); // irregular case
    assert_eq!(table_name_of("Fish"), String::from("fish")); // uncountable case
    assert_eq!(table_name_of("Box"), String::from("boxes"));
    assert_eq!(table_name_of("STUDENT"), String::from("students"));
    assert_eq!(table_name_of("ID"), String::from("ids"));
}
