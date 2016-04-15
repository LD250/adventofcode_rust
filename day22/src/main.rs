/*
Magic Missile costs 53 mana. It instantly does 4 damage.
Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.
*/

fn one_spell(spell_id: &str, hp: i8, mp: u16, s_e: u8, p_e: u8, r_e: u8, m_spend: u16, b_hp: i8, b_dmg: &i8, m_spend_min: &mut u16) {
    let s_e = if s_e > 0 {s_e - 1} else {0};
    let (p_e, b_hp) = if p_e > 0 {(p_e - 1, b_hp - 3)} else {(0, b_hp)};
    let (r_e, mp) = if r_e > 0 {(r_e - 1, mp + 101)} else {(0, mp)};
    if (spell_id == "S" && s_e > 0) || (spell_id == "P" && p_e > 0) || (spell_id == "R" && r_e > 0) {
        return;
    };
    if b_hp <= 0 {
        *m_spend_min = m_spend;
        return;
    }
    if mp <= 0 || hp <= 0 {
        return;
    }
    let (hp, mp, s_e, p_e, r_e, m_spend, b_hp) = match spell_id {
        "M" if mp >= 53 => {(hp, mp - 53, s_e, p_e, r_e, m_spend + 53, b_hp - 4)},
        "D" if mp >= 73 => {(hp + 2, mp - 73, s_e, p_e, r_e, m_spend + 73, b_hp - 2)},
        "S" if mp >= 113 => {(hp, mp - 113, 6, p_e, r_e, m_spend + 113, b_hp)},
        "P" if mp >= 173 => {(hp, mp - 173, s_e, 6, r_e, m_spend + 173, b_hp)},
        "R" if mp >= 229 => {(hp, mp - 229, s_e, p_e, 5, m_spend + 229, b_hp)},
        _ => return,
    
    };
    if (*m_spend_min != 0) && (m_spend >= *m_spend_min) {
        return;
    }
    if b_hp <= 0 {
        if (m_spend < *m_spend_min) || (*m_spend_min == 0) {
            *m_spend_min = m_spend;
        };
        return;
    };
    let (s_e, bdmg) = if s_e > 0 {(s_e - 1, *b_dmg - 7)} else {(0, *b_dmg)};
    let (p_e, b_hp) = if p_e > 0 {(p_e - 1, b_hp - 3)} else {(0, b_hp)};
    let (r_e, mp) = if r_e > 0 {(r_e - 1, mp + 101)} else {(0, mp)};
    if b_hp <= 0 {
        *m_spend_min = m_spend;
        return;
    }

    if hp - bdmg <= 0 {
        return;
    };
    for s in ["M", "D", "S", "P", "R"].iter() {
        one_spell(s, hp - bdmg, mp, s_e, p_e, r_e, m_spend, b_hp, b_dmg, m_spend_min);
    }
}


fn main() {
    let dmg: i8 = 9;
    let mut m_spend_min: u16 = 0;
    for s in ["M", "D", "S", "P", "R"].iter() {
        one_spell(s, 50, 500, 0, 0, 0, 0, 51, &dmg, &mut m_spend_min);
    };
    println!("First part: {}", m_spend_min);
    let dmg: i8 = 10;
    let mut m_spend_min: u16 = 0;
    for s in ["M", "D", "S", "P", "R"].iter() {
        one_spell(s, 49, 500, 0, 0, 0, 0, 51, &dmg, &mut m_spend_min);
    };
    println!("Second part: {}", m_spend_min);
}
