//! voca_rs::query testing
use voca_rs::Voca;

#[test]
fn ends_with() {
    assert!(voca_rs::query::ends_with("the world is yours", "is yours"));
    assert!(voca_rs::query::ends_with("Zażółć gęślą jaźń", "jaźń"));
    assert!(voca_rs::query::ends_with("the world is yours", ""));
    assert!(voca_rs::query::ends_with("", ""));
}
#[test]
fn _ends_with() {
    assert!("the world is yours"._ends_with("is yours"));
    assert!("Zażółć gęślą jaźń"._ends_with("jaźń"));
}
#[test]
fn includes() {
    assert!(voca_rs::query::includes("", "", 0));
    assert!(voca_rs::query::includes("a", "a", 0));
    assert!(voca_rs::query::includes("abc", "c", 2));
    assert!(voca_rs::query::includes(
        "the world is yours",
        "the world",
        0
    ));
    assert!(voca_rs::query::includes("Zażółć gęślą jaźń", "gęślą", 7));
    assert!(voca_rs::query::includes("the world is yours", "", 0));
    assert!(!voca_rs::query::includes("abc", "c", 20));
    assert!(!voca_rs::query::includes("abc", "z", 0));
}
#[test]
fn _includes() {
    assert!("Zażółć gęślą jaźń"._includes("gęślą", 7));
    assert!("abc"._includes("c", 2));
}
#[test]
fn is_alpha() {
    assert!(voca_rs::query::is_alpha("bart"));
    assert!(voca_rs::query::is_alpha("café"));
    assert!(voca_rs::query::is_alpha("cafe\u{0301}"));
    assert!(voca_rs::query::is_alpha("Zażółć"));
    assert!(!voca_rs::query::is_alpha(""));
    assert!(!voca_rs::query::is_alpha("T1000"));
    assert!(!voca_rs::query::is_alpha("\n\t"));
    assert!(!voca_rs::query::is_alpha("lisa!"));
    assert!(!voca_rs::query::is_alpha("lisa and bart"));
    assert!(!voca_rs::query::is_alpha("Zażółć gęślą jaźń"));
}
#[test]
fn _is_alpha() {
    assert!("bart"._is_alpha());
    assert!("café"._is_alpha());
    assert!("cafe\u{0301}"._is_alpha());
}
#[test]
fn is_alphadigit() {
    assert!(voca_rs::query::is_alphadigit("bart"));
    assert!(voca_rs::query::is_alphadigit("café"));
    assert!(voca_rs::query::is_alphadigit("cafe\u{0301}"));
    assert!(voca_rs::query::is_alphadigit("T1000"));
    assert!(voca_rs::query::is_alphadigit("1000"));
    assert!(voca_rs::query::is_alphadigit("Zażółć"));
    assert!(!voca_rs::query::is_alphadigit(""));
    assert!(!voca_rs::query::is_alphadigit("10-00"));
    assert!(!voca_rs::query::is_alphadigit("\n\t"));
    assert!(!voca_rs::query::is_alphadigit("lisa!"));
    assert!(!voca_rs::query::is_alphadigit("lisa and bart"));
    assert!(!voca_rs::query::is_alphadigit("Zażółć gęślą jaźń"));
}
#[test]
fn _is_alphadigit() {
    assert!("bart"._is_alphadigit());
    assert!(!"10-00"._is_alphadigit());
    assert!("T1000"._is_alphadigit());
}
#[test]
fn is_blank() {
    assert!(voca_rs::query::is_blank(""));
    assert!(voca_rs::query::is_blank("   "));
    assert!(voca_rs::query::is_blank("\n\t\r"));
    assert!(!voca_rs::query::is_blank("Zażółć gęślą jaźń"));
}
#[test]
fn _is_blank() {
    assert!("   "._is_blank());
    assert!(!"Zażółć gęślą jaźń"._is_blank());
}
#[test]
fn is_camel_case() {
    assert!(voca_rs::query::is_camel_case(""));
    assert!(voca_rs::query::is_camel_case("birdFlight"));
    assert!(!voca_rs::query::is_camel_case("bird flight"));
    assert!(!voca_rs::query::is_camel_case("-BIRD-FLIGHT-"));
    assert!(!voca_rs::query::is_camel_case("Zażółć gęślą jaźń"));
    assert!(voca_rs::query::is_camel_case("zażółćGęśląJaźń"));
}
#[test]
fn _is_camel_case() {
    assert!("birdFlight"._is_camel_case());
    assert!(!"bird flight"._is_camel_case());
    assert!("zażółćGęśląJaźń"._is_camel_case());
}
#[test]
fn is_capitalize() {
    assert!(voca_rs::query::is_capitalize(""));
    assert!(voca_rs::query::is_capitalize("John has a motorcycle"));
    assert!(!voca_rs::query::is_capitalize("the world is yours"));
    assert!(voca_rs::query::is_capitalize("The world is yours"));
    assert!(!voca_rs::query::is_capitalize("The World IS YourS"));
    assert!(voca_rs::query::is_capitalize("Zażółć gęślą jaźń"));
    assert!(!voca_rs::query::is_capitalize("ZAżółć GĘŚLĄ jAźń"));
    assert!(voca_rs::query::is_capitalize("Это вообще работает?"),);
    assert!(!voca_rs::query::is_capitalize("это Вообще РАБОТАЕТ?"));
}
#[test]
fn _is_capitalize() {
    assert!("John has a motorcycle"._is_capitalize());
    assert!(!"the world is yours"._is_capitalize());
}
#[test]
fn is_decapitalize() {
    assert!(voca_rs::query::is_decapitalize(""));
    assert!(!voca_rs::query::is_decapitalize("John has a motorcycle"));
    assert!(voca_rs::query::is_decapitalize("the world is yours"));
    assert!(!voca_rs::query::is_decapitalize("The world is yours"));
    assert!(!voca_rs::query::is_decapitalize("the World IS YourS"));
    assert!(voca_rs::query::is_decapitalize("zażółć gęślą jaźń"));
    assert!(!voca_rs::query::is_decapitalize("ZAżółć GĘŚLĄ jAźń"));
    assert!(voca_rs::query::is_decapitalize("это вообще работает?"));
    assert!(!voca_rs::query::is_decapitalize("Это вообще работает?"));
}
#[test]
fn _is_decapitalize() {
    assert!(!"John has a motorcycle"._is_decapitalize());
    assert!("the world is yours"._is_decapitalize());
}
#[test]
fn is_digit() {
    assert!(voca_rs::query::is_digit(""));
    assert!(voca_rs::query::is_digit("0"));
    assert!(voca_rs::query::is_digit("100"));
    assert!(voca_rs::query::is_digit("100500"));
    assert!(!voca_rs::query::is_digit("1.5"));
    assert!(!voca_rs::query::is_digit("0xFF"));
    assert!(!voca_rs::query::is_digit("ten"));
}
#[test]
fn _is_digit() {
    assert!("0"._is_digit());
    assert!(!"1.5"._is_digit());
}
#[test]
fn is_empty() {
    assert!(voca_rs::query::is_empty(""));
    assert!(!voca_rs::query::is_empty("Zażółć gęślą jaźń"));
    assert!(!voca_rs::query::is_empty("the world is yours"));
}
#[test]
fn _is_empty() {
    assert!(""._is_empty());
    assert!(!"the world is yours"._is_empty());
}
#[test]
fn is_foreign_key() {
    assert!(voca_rs::query::is_foreign_key(""));
    assert!(voca_rs::query::is_foreign_key("foo_bar_id"));
    assert!(!voca_rs::query::is_foreign_key("foo_bar"));
    assert!(!voca_rs::query::is_foreign_key("the world is yours"));
    assert!(!voca_rs::query::is_foreign_key(
        "foo-bar-string-that-is-really-really-long"
    ));
    assert!(!voca_rs::query::is_foreign_key(
        "FooBarIsAReallyReallyLongString"
    ));
    assert!(!voca_rs::query::is_foreign_key(
        "foo_bar_string_that_is_really_really_long"
    ));
    assert!(!voca_rs::query::is_foreign_key(
        "voca::voca_rs::query::is_foreign_key"
    ));
}
#[test]
fn _is_foreign_key() {
    assert!(""._is_foreign_key());
    assert!("foo_bar_id"._is_foreign_key());
    assert!(!"foo_bar"._is_foreign_key());
    assert!(!"the world is yours"._is_foreign_key());
    assert!(!"foo-bar-string-that-is-really-really-long"._is_foreign_key());
}
#[test]
fn is_lowercase() {
    assert!(voca_rs::query::is_lowercase(""));
    assert!(voca_rs::query::is_lowercase("the world is yours"));
    assert!(!voca_rs::query::is_lowercase("Zażółć gęślą jaźń"));
    assert!(!voca_rs::query::is_lowercase("T1000"));
}
#[test]
fn _is_lowercase() {
    assert!("the world is yours"._is_lowercase());
    assert!(!"T1000"._is_lowercase());
}
#[test]
fn is_lower_first() {
    assert!(voca_rs::query::is_lower_first(""));
    assert!(voca_rs::query::is_lower_first("the world is yours"));
    assert!(voca_rs::query::is_lower_first("tHE World"));
    assert!(voca_rs::query::is_lower_first("żółć niedźwiedzia"));
    assert!(!voca_rs::query::is_lower_first("Zażółć gęślą jaźń"));
    assert!(!voca_rs::query::is_lower_first("T1000"));
}
#[test]
fn _is_lower_first() {
    assert!("the world is yours"._is_lower_first());
    assert!(!"Zażółć gęślą jaźń"._is_lower_first());
}
#[test]
fn is_kebab_case() {
    assert!(voca_rs::query::is_kebab_case(""));
    assert!(voca_rs::query::is_kebab_case("bird-flight"));
    assert!(voca_rs::query::is_kebab_case("is-kebab-case"));
    assert!(voca_rs::query::is_kebab_case("zażółć-gęślą-jaźń"));
    assert!(!voca_rs::query::is_kebab_case("-BIRD-FLIGHT-"));
    assert!(!voca_rs::query::is_kebab_case("tHE World"));
    assert!(!voca_rs::query::is_kebab_case("żółć niedźwiedzia"));
    assert!(!voca_rs::query::is_kebab_case("T1000"));
}
#[test]
fn _is_kebab_case() {
    assert!("bird-flight"._is_kebab_case());
    assert!(!"-BIRD-FLIGHT-"._is_kebab_case());
}
#[test]
fn is_numeric() {
    assert!(voca_rs::query::is_numeric(""));
    assert!(voca_rs::query::is_numeric("0"));
    assert!(voca_rs::query::is_numeric("+0"));
    assert!(voca_rs::query::is_numeric("0.0"));
    assert!(voca_rs::query::is_numeric("1000"));
    assert!(voca_rs::query::is_numeric("1.56"));
    assert!(voca_rs::query::is_numeric("-10.888"));
    assert!(voca_rs::query::is_numeric("350"));
    assert!(voca_rs::query::is_numeric("-20.5"));
    assert!(voca_rs::query::is_numeric("1.5E+2"));
    assert!(voca_rs::query::is_numeric("1.25E-3"));
    assert!(voca_rs::query::is_numeric("125e5"));
    assert!(voca_rs::query::is_numeric("125e-3"));
    assert!(voca_rs::query::is_numeric("0xFF"));
    assert!(voca_rs::query::is_numeric("0x22"));
    assert!(voca_rs::query::is_numeric("0x123ABC"));
    assert!(voca_rs::query::is_numeric("0x123ABC"));
    assert!(voca_rs::query::is_numeric("0x1ab9"));
    assert!(!voca_rs::query::is_numeric("0x123z"));
    assert!(!voca_rs::query::is_numeric("five"));
    assert!(!voca_rs::query::is_numeric(".."));
    assert!(!voca_rs::query::is_numeric(" "));
}
#[test]
fn _is_numeric() {
    assert!("0"._is_numeric());
    assert!("1.5E+2"._is_numeric());
    assert!("0x22"._is_numeric());
    assert!(!"0x123z"._is_numeric());
}
#[test]
fn is_pascal_case() {
    assert!(voca_rs::query::is_pascal_case(""));
    assert!(voca_rs::query::is_pascal_case("BirdFlight"));
    assert!(voca_rs::query::is_pascal_case("ЭтоПаскальКейс"));
    assert!(!voca_rs::query::is_pascal_case("birdFlight"));
    assert!(!voca_rs::query::is_pascal_case("bird flight"));
    assert!(!voca_rs::query::is_pascal_case("-BIRD-FLIGHT-"));
    assert!(!voca_rs::query::is_pascal_case("Zażółć gęślą jaźń"));
    assert!(voca_rs::query::is_pascal_case("ZażółćGęśląJaźń"));
    assert!(!voca_rs::query::is_pascal_case("zażółćGęśląJaźń"));
}
#[test]
fn _is_pascal_case() {
    assert!("BirdFlight"._is_pascal_case());
    assert!(!"birdFlight"._is_pascal_case());
}
#[test]
fn is_shouty_kebab_case() {
    assert!(voca_rs::query::is_shouty_kebab_case(""));
    assert!(voca_rs::query::is_shouty_kebab_case("BIRD-FLIGHT"));
    assert!(voca_rs::query::is_shouty_kebab_case("ЭТО-ОЙ-КЕБАБ-КЕЙС"));
    assert!(!voca_rs::query::is_shouty_kebab_case("birdFlight"));
    assert!(!voca_rs::query::is_shouty_kebab_case("bird flight"));
    assert!(!voca_rs::query::is_shouty_kebab_case("-BIRD-FLIGHT-"));
    assert!(!voca_rs::query::is_shouty_kebab_case("Zażółć gęślą jaźń"));
    assert!(voca_rs::query::is_shouty_kebab_case("ZAŻÓŁĆ-GĘŚLĄ-JAŹŃ"));
    assert!(!voca_rs::query::is_shouty_kebab_case("zażółćGęśląJaźń"));
}
#[test]
fn _is_shouty_kebab_case() {
    assert!("BIRD-FLIGHT"._is_shouty_kebab_case());
    assert!(!"birdFlight"._is_shouty_kebab_case());
}
#[test]
fn is_snake_case() {
    assert!(voca_rs::query::is_snake_case(""));
    assert!(voca_rs::query::is_snake_case("bird_flight"));
    assert!(voca_rs::query::is_snake_case("это_снэйк_кейс"));
    assert!(!voca_rs::query::is_snake_case("birdFlight"));
    assert!(!voca_rs::query::is_snake_case("bird flight"));
    assert!(!voca_rs::query::is_snake_case("-BIRD-FLIGHT-"));
    assert!(!voca_rs::query::is_snake_case("Zażółć gęślą jaźń"));
    assert!(voca_rs::query::is_snake_case("zażółć_gęślą_jaźń"));
    assert!(!voca_rs::query::is_snake_case("zażółćGęśląJaźń"));
}
#[test]
fn _is_snake_case() {
    assert!("bird_flight"._is_snake_case());
    assert!(!"birdFlight"._is_snake_case());
}
#[test]
fn is_shouty_snake_case() {
    assert!(voca_rs::query::is_shouty_snake_case(""));
    assert!(voca_rs::query::is_shouty_snake_case("BIRD_FLIGHT"));
    assert!(!voca_rs::query::is_shouty_snake_case("bird_flight"));
    assert!(voca_rs::query::is_shouty_snake_case("ЭТО_СНЭЙК_КЕЙС"));
    assert!(!voca_rs::query::is_shouty_snake_case("birdFlight"));
    assert!(!voca_rs::query::is_shouty_snake_case("bird flight"));
    assert!(!voca_rs::query::is_shouty_snake_case("-BIRD-FLIGHT-"));
    assert!(!voca_rs::query::is_shouty_snake_case("Zażółć gęślą jaźń"));
    assert!(voca_rs::query::is_shouty_snake_case("ZAŻÓŁĆ_GĘŚLĄ_JAŹŃ"));
    assert!(!voca_rs::query::is_shouty_snake_case("zażółćGęśląJaźń"));
}
#[test]
fn _is_shouty_snake_case() {
    assert!("BIRD_FLIGHT"._is_shouty_snake_case());
    assert!(!"bird_flight"._is_shouty_snake_case());
}
#[test]
fn is_title() {
    assert!(!voca_rs::query::is_title(""));
    assert!(voca_rs::query::is_title("The World Is Yours"));
    assert!(!voca_rs::query::is_title("the world is yours"));
    assert!(voca_rs::query::is_title("This Is String Example...Wow!!!"));
    assert!(!voca_rs::query::is_title(
        "This is string example....wow!!!"
    ));
    assert!(voca_rs::query::is_title("Zażółć Gęślą Jaźń"));
    assert!(!voca_rs::query::is_title("Zażółć gęślą jaźń"));
    assert!(voca_rs::query::is_title("T1000"));
}
#[test]
fn _is_title() {
    assert!("The World Is Yours"._is_title());
    assert!(!"the world is yours"._is_title());
}
#[test]
fn is_train_case() {
    assert!(voca_rs::query::is_train_case(""));
    assert!(voca_rs::query::is_train_case("Goodbye-Blue-Sky"));
    assert!(!voca_rs::query::is_train_case("bird_flight"));
    assert!(voca_rs::query::is_train_case("Это-Снэйк-Кейс"));
    assert!(!voca_rs::query::is_train_case("birdFlight"));
    assert!(!voca_rs::query::is_train_case("bird flight"));
    assert!(!voca_rs::query::is_train_case("-BIRD-FLIGHT-"));
    assert!(!voca_rs::query::is_train_case("Zażółć gęślą jaźń"));
    assert!(voca_rs::query::is_train_case("Zażółć-Gęślą-Jaźń"));
    assert!(!voca_rs::query::is_train_case("zażółćGęśląJaźń"));
}
#[test]
fn _is_train_case() {
    assert!(voca_rs::query::is_train_case("Goodbye-Blue-Sky"));
    assert!(!voca_rs::query::is_train_case("bird_flight"));
}
#[test]
fn is_uppercase() {
    assert!(voca_rs::query::is_uppercase(""));
    assert!(voca_rs::query::is_uppercase("THE WORLD IS YOURS"));
    assert!(!voca_rs::query::is_uppercase("Zażółć gęślą jaźń"));
    assert!(!voca_rs::query::is_uppercase("t1000"));
}
#[test]
fn _is_uppercase() {
    assert!("THE WORLD IS YOURS"._is_uppercase());
    assert!(!"Zażółć gęślą jaźń"._is_uppercase());
}
#[test]
fn is_upper_first() {
    assert!(voca_rs::query::is_upper_first(""));
    assert!(voca_rs::query::is_upper_first("The world is yours"));
    assert!(voca_rs::query::is_upper_first("Zażółć gęślą jaźń"));
    assert!(voca_rs::query::is_upper_first("T1000"));
    assert!(voca_rs::query::is_upper_first("Żółć niedźwiedzia"));
    assert!(!voca_rs::query::is_upper_first("żółć niedźwiedzia"));
    assert!(!voca_rs::query::is_upper_first("tHE World"));
}
#[test]
fn _is_upper_first() {
    assert!(voca_rs::query::is_upper_first("The world is yours"));
    assert!(!voca_rs::query::is_upper_first("żółć niedźwiedzia"));
}
#[test]
fn matches() {
    assert!(voca_rs::query::matches("", "", 0));
    assert!(!voca_rs::query::matches("pluto", "a", 0));
    assert!(voca_rs::query::matches("pluto", r"plu.{2}", 0));
    assert!(!voca_rs::query::matches("apollo 11", r"\d{3}", 0));
    assert!(voca_rs::query::matches("pacific", "", 0));
    assert!(!voca_rs::query::matches("", "1", 0));
    assert!(voca_rs::query::matches("pacific ocean", "ocean", 0));
    assert!(voca_rs::query::matches(
        "pacific ocean",
        "^pacific ocean$",
        0
    ));
    assert!(voca_rs::query::matches("pacific ocean", r"\s", 0));
    assert!(voca_rs::query::matches("1500", r"\d", 0));
    assert!(voca_rs::query::matches("Zażółć gęślą jaźń", "gęślą", 0));
    assert!(voca_rs::query::matches("Zażółć gęślą jaźń", "gęślą", 11));
    assert!(!voca_rs::query::matches("Zażółć gęślą jaźń", "gęślą", 12));
    assert!(
        // [^] is not a valid regex
        !voca_rs::query::matches("Zażółć gęślą jaźń", "[^]", 0)
    );
}
#[test]
fn _matches() {
    assert!(!"pluto"._matches("a", 0));
    assert!("pluto"._matches(r"plu.{2}", 0));
}
#[test]
fn query() {
    assert!(voca_rs::query::query("", "", 0));
    assert!(voca_rs::query::query("a", "a", 0));
    assert!(voca_rs::query::query("abc", "c", 2));
    assert!(voca_rs::query::query("the world is yours", "the world", 0));
    assert!(voca_rs::query::query("the world is yours", "te wld", 0));
    assert!(voca_rs::query::query("the world is yours", "td", 0));
    assert!(voca_rs::query::query("Zażółć gęślą jaźń", "gęślą", 7));
    assert!(voca_rs::query::query("the world is yours", "", 0));
    assert!(!voca_rs::query::query("the world is yours", "asdd", 0));
    assert!(!voca_rs::query::query("the world is yours", "eht", 0));
    assert!(!voca_rs::query::query("abc", "c", 20));
    assert!(!voca_rs::query::query("abc", "z", 0));
}
#[test]
fn _query() {
    assert!("the world is yours"._query("the world", 0));
    assert!(!"the world is yours"._query("eht", 0));
}
#[test]
fn starts_with() {
    assert!(voca_rs::query::starts_with(
        "the world is yours",
        "the world"
    ));
    assert!(voca_rs::query::starts_with("Zażółć gęślą jaźń", "Zażółć"));
    assert!(voca_rs::query::starts_with("the world is yours", ""));
    assert!(voca_rs::query::starts_with("", ""));
    assert!(!voca_rs::query::starts_with("the world is yours", "s"));
}
#[test]
fn _starts_with() {
    assert!("the world is yours"._starts_with("the world"));
    assert!(!"the world is yours"._starts_with("s"));
}
