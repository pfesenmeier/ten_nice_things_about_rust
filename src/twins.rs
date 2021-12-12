#[derive(Debug, Clone)]
#[allow(unused)]
struct Twin {
    is: String,
}

#[allow(unused)]
fn good_or_evil() -> String {
    let mut good_twin = Twin {
        is: "good".to_string(),
    };
    let mut evil_twin = &mut good_twin;
    evil_twin.is = "evil".to_string();

    format!("Good twin: {:?}", good_twin)
}

#[test]
fn good_or_evil_test() {
    assert_eq!(good_or_evil(), "Good twin: Twin { is: \"evil\" }");
}
