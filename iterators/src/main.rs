#![allow(unused)]

#[derive(Debug, PartialEq, Clone)]
struct Player {
    name: String,
}

#[derive(Debug, PartialEq)]
struct Team {
    member_count: u32,
    members: Vec<Player>
}

impl Team {
    fn new() -> Team {
        Team { member_count: 0, members: Vec::new() }
    }

    fn add_player(&mut self, player: Player) -> () {
        // let n = vec![];
        // n.();
        self.member_count += 1;
        self.members.push(player);
    }
}

impl Iterator for Team {
    type Item = Player;

    fn next(&mut self) -> Option<Self::Item> {
        if self.member_count < 1 {
            None
        } else {
            self.member_count -= 1;
            Some(self.members.remove(self.member_count as usize))
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2 = vec![3,4,5];
    let v: usize = v2.iter().sum();
    println!("{:?}", v2);
    // iterator_sum();

    let mut team = Team::new();
    let hazard = Player { name: String::from("Eden Hazard") };
    let messi = Player { name: String::from("Lionel Messi") };
    let ronaldo = Player { name: String::from("Cristiano Ronaldo") };
    team.add_player(hazard);
    team.add_player(messi);
    team.add_player(ronaldo);

    println!("The Team has {:?} players", team.count());
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn team_iterator_returns_none() {
    let mut team = Team::new();
    let hazard = Player { name: String::from("Eden Hazard") };
    let messi = Player { name: String::from("Lionel Messi") };
    let ronaldo = Player { name: String::from("Cristiano Ronaldo") };
    team.add_player(hazard);
    team.add_player(messi);
    team.add_player(ronaldo);

    assert_eq!(team.next(), Some(Player { name: String::from("Cristiano Ronaldo") }));
    assert_eq!(team.next(), Some(Player { name: String::from("Lionel Messi") }));
    assert_eq!(team.next(), Some(Player { name: String::from("Eden Hazard") }));
    assert_eq!(team.next(), None);
}