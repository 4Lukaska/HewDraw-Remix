use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
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
        WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    FT_MOTION_RATE(agent, 1.4);
    if material_kind == gold {
        FT_MOTION_RATE(agent, 0.8);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        FT_MOTION_RATE(agent, 1.0);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            let mut damage = 16.0; // default damage, used for wood and gold
            let mut kbg = 85; //default knockback growth, used for wood, stone, and iron
            if material_kind == stone {
                damage = 17.0; // damage for stone
            } else if material_kind == iron {
                damage = 19.0; // damage for iron
            } else if material_kind == gold {
                kbg = 95; // kbg for gold
            } else if material_kind == diamond {
                damage = 20.0; // damage for diamond
                kbg = 90; // kbg for diamond
            }
            let mut sfx = *COLLISION_SOUND_ATTR_PUNCH; // collision sound
            if [iron, gold, diamond].contains(&material_kind) {
                sfx = *COLLISION_SOUND_ATTR_CUTUP;
            }
            ATTACK(agent, 0, 0, Hash40::new("top"), damage, 42, kbg, 0, 34, 3.8, 0.0, 8.0, 6.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, sfx, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), damage, 42, kbg, 0, 34, 4.4, 0.0, 8.0, 10.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, sfx, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("top"), damage, 42, kbg, 0, 34, 4.6, 0.0, 8.0, 14.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, sfx, *ATTACK_REGION_SWORD);
            WorkModule::set_float(boma, 14.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        else {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 85, 0, 20, 3.8, 0.0, 8.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 361, 85, 0, 20, 4.4, 0.0, 8.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        if material_kind == gold {
            if is_excute(agent) {
                FT_MOTION_RATE(agent, 0.8);
            }
        }
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
    let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
    let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
    let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
    let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, false, -1);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 105, 100, 100, 0, 3.0, 0.0, 3.0, 8.0, Some(0.0), Some(3.0), Some(1.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 10.0);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 80, 90, 0, 47, 8.0, 0.0, 26.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
        let mut damage = 6.5; // default damage, used for wood and gold
        let mut kbg = 82; //default knockback growth, used for wood, stone, and iron
        if material_kind == stone {
            damage = 7.5; // damage for stone
        } else if material_kind == iron {
            damage = 9.5; // damage for iron
        } else if material_kind == gold {
            kbg = 92; // kbg for gold
        } else if material_kind == diamond {
            damage = 10.5; // damage for diamond
            kbg = 87; // kbg for diamond
        }
        ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 52, kbg, 0, 56, 3.5, 4.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        WorkModule::set_float(boma, 4.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        AttackModule::clear_all(boma);
    }
}

// shrinks the magma block effects to match its reduced size

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -6, 9, -6, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pickel_block_magma_heat"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("null"), Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        EFFECT_OFF_KIND(agent, Hash40::new("pickel_block_magma_heat"), false, true);
        EFFECT(agent, Hash40::new("pickel_block_break_magma"), Hash40::new("top"), 0, 25, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);
    
    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    agent.acmd("effect_attackhi4", effect_attackhi4, Priority::Low);
}
