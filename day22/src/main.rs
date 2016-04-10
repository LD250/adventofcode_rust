/*
Magic Missile costs 53 mana. It instantly does 4 damage.
Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.
*/

fn one_spell(spell_id: &str, hp: i8, mp: u8, s_e: u8, p_e: u8, r_e: u8, m_spend: u16, b_hp: u8, b_dmg: &u8, m_spend_min: &mut u16) -> u16 {
    if ((spell_id == "S" && s_e > 0) || (spell_id == "P" && p_e > 0) || (spell_id == "R" && r_e > 0)) {
        return 0;
    };
    let (s_e, bdmg) =  if s_e > 0 {(s_e - 1, *b_dmg - 7)} else {(0, *b_dmg)};
    let (p_e, b_hp) =  if p_e > 0 {(p_e - 1, b_hp - 3)} else {(0, b_hp)};
    let (r_e, mp) =  if r_e > 0 {(r_e - 1, mp + 101)} else {(0, mp)};
    if (mp <= 0 || hp <= 0) {
        return 0;
    }
    if (bhp <= 0) {
        return m_spend;
        *m_spend_min = m_spend;
    }
    let (hp, mp, s_e, p_e, r_e, m_spend, b_hp) = match spell_id {
        "M" => {(hp, mp - 53, s_e, p_e, r_e, m_spend + 53, b_hp - 4)},
        "D" => {(hp + 2, mp - 73, s_e, p_e, r_e, m_spend + 73, b_hp - 2)},
        "S" => {(hp, mp - 113, 6, p_e, r_e, m_spend + 113, b_hp)},
        "P" => {(hp, mp - 173, s_e, 6, r_e, m_spend + 173, b_hp)},
          _ => {(hp, mp - 229, s_e, p_e, 5, m_spend + 229, b_hp)},
    
    };
    if hp - bdmg <= 0 {
        return 0;
    }
}


fn main() {
    0;
}
