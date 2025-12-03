fn print_elements(colors: &[String]) {
    //Option 1 for loop
    for color in colors.iter() {
        println!("{}", color);
    }

    //Option 2 built-in methods: iterator_adaptors or consumers
    colors.iter().for_each(|color| {println!("{}", color);}); //use closure

    //Use iterator adaptor: map
    colors.iter().map(|color| format!("{} {} ", color, color)).for_each(|color| {println!("{}", color);});
}

fn to_uppercase(colors: &[String]) -> Vec<String> {
    colors.iter()
          .map(|color| color.to_uppercase())
          .collect()
}
fn shorten_strings(colors: &mut [String]) {
    colors.iter_mut().for_each(| color| { color.truncate(1);});
}

fn move_elements(vec_a: Vec<String> , vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|a| vec_b.push(a));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(
            |el| el.chars().map(|c| c.to_string()).collect()
        )
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements.iter().find(|el|el.contains(search)).map_or(String::from(fallback),|el|el.to_string())
}

fn main() {
    let mut colors = vec![String::from("red"), String::from("green"), String::from("blue")];

    /*println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());*/
    print_elements(&colors);

    let newcolors = to_uppercase(&colors);
    println!("{:?}", newcolors);

    let found_color = find_color_or(&colors, "re", "orange");
    println!("{:?}", found_color);
    let exploded = explode(&newcolors);
    println!("Exploded is : {:?}", exploded);
    shorten_strings(&mut colors);
    print_elements(&colors);
    let mut vec_b =  vec![];
    move_elements(colors, &mut vec_b);
    println!("{:?}", vec_b);
}
