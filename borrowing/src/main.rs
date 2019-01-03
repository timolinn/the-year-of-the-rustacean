fn main() {
    let mut team = String::from("Chelsea Football Club");
    let newTeam = to_upper(&team);
    // modify_borrowed(&team); //   We are not allowed to modify something we have immutable reference to
    modify_borrowed2(&mut team);
    println!("old name = {}, new name = {}", team, newTeam);
}

fn to_upper(s: &String) -> String {
    s.to_uppercase()
}

// fn modify_borrowed(s: &String) {
//     s.push_str(" , Stamford Bridge.");
// }

fn modify_borrowed2(s: &mut String) {
    s.push_str(" , Stamford Bridge.");
}
