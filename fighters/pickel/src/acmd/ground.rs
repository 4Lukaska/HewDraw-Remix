use super::*;

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
    let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
    let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
    let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
    let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            let mut damage = 10.0; // default damage, used for wood and gold
            let mut kbg = 66; //default knockback growth, used for wood, stone, and iron
            if material_kind == stone {
                damage = 11.0; // damage for stone
            } else if material_kind == iron {
                damage = 13.0; // damage for iron
            } else if material_kind == gold {
                kbg = 76; // kbg for gold
            } else if material_kind == diamond {
                damage = 14.0; // damage for diamond
                kbg = 71; // kbg for diamond
            }
            ATTACK(agent, 0, 0, Hash40::new("top"), damage, 50, kbg, 0, 68, 5.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), damage, 50, kbg, 0, 68, 5.8, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 4.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
            wait(lua_state, 3.0);
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), (damage - 2.0), 50, (kbg - 8), 0, 66, 5.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 0, Hash40::new("top"), (damage - 2.0), 50, (kbg - 8), 0, 66, 5.2, 0.0, 8.8, 9.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
        else {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 66, 0, 48, 5.8, 0.0, 8.8, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
            wait(lua_state, 3.0);
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 58, 0, 46, 5.2, 0.0, 8.8, 3.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        if material_kind == gold {
            if is_excute(agent) {
                MotionModule::set_rate(boma, 1.2);
                MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.2);
            }
        }
    }
    frame(lua_state, 32.0);
            if is_excute(agent) {
            MotionModule::set_rate(boma, 1.0);
            MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
            }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackdash", game_attackdash, Priority::Low);
}
