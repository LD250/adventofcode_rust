extern crate itertools;
use itertools::Itertools;

fn main() {
    #[derive(Debug)]
    struct Item<'a> {
        name: &'a str,
        cost: i16,
        damage: i16,
        armor: i16,
    };
    let input_data = "
        Dagger        8     4       0
        Shortsword   10     5       0
        Warhammer    25     6       0
        Longsword    40     7       0
        Greataxe     74     8       0

        NoArmor      0     0       0
        Leather      13     0       1
        Chainmail    31     0       2
        Splintmail   53     0       3
        Bandedmail   75     0       4
        Platemail   102     0       5

        NoRing    0     0       0
        Damage+1    25     1       0
        Damage+2    50     2       0
        Damage+3   100     3       0
        Defense+1   20     0       1
        Defense+2   40     0       2
        Defense+3   80     0       3
        ";
    struct Boss {
        hp: i16,
        dmg: i16,
        armor: i16,
    }
    let boss: Boss = Boss {
        hp: 103,
        dmg: 9,
        armor: 2,
    };
    let my_hp: i16 = 100;
    fn get_item_from_line(line: &str) -> Item {
        let parts: Vec<&str> = line.split_whitespace().collect();
        Item {
            name: parts[0],
            cost: parts[1].parse().unwrap(),
            damage: parts[2].parse().unwrap(),
            armor: parts[3].parse().unwrap(),
        }
    };
    let weapons: Vec<Item> = input_data.split('\n')
                                       .skip(1)
                                       .take(5)
                                       .map(get_item_from_line)
                                       .collect();
    let armor: Vec<Item> = input_data.split('\n')
                                     .skip(7)
                                     .take(6)
                                     .map(get_item_from_line)
                                     .collect();
    let rings: Vec<Item> = input_data.split('\n')
                                     .skip(14)
                                     .take(7)
                                     .map(get_item_from_line)
                                     .collect();

    fn can_win(dmg: i16, armor: i16, hp: &i16, boss: &Boss) -> bool {
        let dmg_done = if boss.armor >= dmg {
            1
        } else {
            dmg - boss.armor
        };
        let dmg_recieved = if armor >= boss.dmg {
            1
        } else {
            boss.dmg - armor
        };
        (*hp as f32 / dmg_recieved as f32).ceil() >= (boss.hp as f32 / dmg_done as f32).ceil()


    }
    let mut gold_amount_win: Vec<i16> = vec![];
    let mut gold_amount_loose: Vec<i16> = vec![];
    for w in &weapons {
        for a in &armor {
            let cost = w.cost + a.cost;
            if can_win(w.damage, a.armor, &my_hp, &boss) {
                gold_amount_win.push(cost);
            } else {
                gold_amount_loose.push(cost);
            };
            for (r1, r2) in rings.iter().combinations() {
                let cost = w.cost + a.cost + r1.cost + r2.cost;
                if can_win(w.damage + r1.damage + r2.damage,
                           a.armor + r1.armor + r2.armor,
                           &my_hp,
                           &boss) {
                    gold_amount_win.push(cost);
                } else {
                    gold_amount_loose.push(cost);
                };

            }
        }
    }
    println!("{:?}", gold_amount_win.iter().min().unwrap());
    println!("{:?}", gold_amount_loose.iter().max().unwrap());
}
