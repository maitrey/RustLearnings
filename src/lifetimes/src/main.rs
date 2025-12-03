
fn next_langauge<'a>(languages: &'a [String], current:&str) -> &'a str {
    let mut found = false;
    for lang in languages {
        if found {
            return lang;
        }
        if lang == current {
            found = true;
        }
    }
    languages.last().unwrap()
}

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> & 'a str {
    if lang_a.len() > lang_b.len() {
        return lang_a;
    } else { 
        return lang_b;
    }

}

fn main() {
    let langauges = vec![String::from("Rust"), String::from("Python"), String::from("Javascript")];

    let result = next_langauge(&langauges, "Rust");
    println!("{}", result);
    let results = last_language(&langauges);
    println!("{}", results);
    let longlang = longest_language(langauges[0].as_str(), langauges[1].as_str());
    println!("{}", longlang);
}
