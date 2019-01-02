fn main() {
    let team = String::from("Chelsea Football Club");
    let newTeam = toUpper(&team);
    println!("old name = {}, new name = {}", team, newTeam);
}

fn toUpper(s: &String) -> String {
    s.to_uppercase()
}
