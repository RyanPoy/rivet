use super::*;

// --- 1. 数值类型转换测试 ---
#[test]
fn test_numeric_into_value() {
    // 测试基础整数 (i32)
    let val_i32 = IntoValue::into_value(42i32);
    assert_eq!(val_i32, Value::Single(Scalar::I32(42)));

    // 测试无符号整数 (u64)
    let val_u64 = IntoValue::into_value(1024u64);
    assert_eq!(val_u64, Value::Single(Scalar::U64(1024)));

    // 测试布尔值
    let val_bool = IntoValue::into_value(true);
    assert_eq!(val_bool, Value::Single(Scalar::Bool(true)));
}

// --- 2. 字符串转换测试 (核心：所有权与引用) ---
#[test]
fn test_string_into_value() {
    // 测试字面量 &str -> Value::String
    let s_ref = "literal";
    let val_from_ref = IntoValue::into_value(s_ref);
    assert_eq!(val_from_ref, Value::Single(Scalar::String("literal".to_string())));

    // 测试所有权 String -> Value::String (验证所有权转移)
    let s_owned = String::from("owned");
    // 这一步之后 s_owned 所有权被转移进函数
    let val_from_owned = IntoValue::into_value(s_owned);
    assert_eq!(val_from_owned, Value::Single(Scalar::String("owned".to_string())));

    // 测试 &String -> Value::String
    let s_obj = String::from("string_obj");
    let val_from_obj_ref = IntoValue::into_value(&s_obj);
    assert_eq!(val_from_obj_ref, Value::Single(Scalar::String("string_obj".to_string())));
    // s_obj 在这里依然有效，证明了借用转换的可用性
    assert_eq!(s_obj.len(), 10);
}

// --- 3. Option 包装转换测试 ---
#[test]
fn test_option_into_value() {
    // Option<i32> -> Value
    let opt_i: Option<i32> = Some(123);
    assert_eq!(IntoValue::into_value(opt_i), Value::Single(Scalar::I32(123)));

    // Option<String> (Owned) -> Value
    let opt_s: Option<String> = Some(String::from("inner"));
    assert_eq!(IntoValue::into_value(opt_s), Value::Single(Scalar::String("inner".into())));

    // Option<&str> -> Value
    let opt_str: Option<&str> = Some("inner_ref");
    assert_eq!(IntoValue::into_value(opt_str), Value::Single(Scalar::String("inner_ref".into())));

    // None 情况测试 (针对不同 T)
    let none_i: Option<i32> = None;
    assert_eq!(IntoValue::into_value(none_i), Value::Single(Scalar::Null));

    let none_s: Option<String> = None;
    assert_eq!(IntoValue::into_value(none_s), Value::Single(Scalar::Null));
}

// --- 4. 极端边界测试 ---
#[test]
fn test_nested_logic() {
    // 验证 &str 是否只能匹配 IntoValue<String>
    // 注意：IntoValue::into_value("1") 在这里会编译报错，
    // 这种报错正是我们依赖 IntoValue<T> 泛型约束实现类型安全的原因。

    let val = IntoValue::into_value("");
    assert_eq!(val, Value::Single(Scalar::String("".into())));
}
