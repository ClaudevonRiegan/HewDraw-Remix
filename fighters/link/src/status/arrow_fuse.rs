use super::*;
use globals::*;
// status script import


pub fn install() {
    install_status_scripts!(
        arrow_haved_main,
        special_n_exit,
        arrow_fly_end,
        arrow_fly_init,
    );
}

unsafe fn set_fuse_params(weapon: &mut L2CFighterBase) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_id);
    let kind = ItemModule::get_have_item_kind(owner_module_accessor,0);
    let link_object = utils::util::get_battle_object_from_id(owner_id);
    if kind == *ITEM_KIND_BOMBER {
        VarModule::set_int(link_object,vars::link::instance::FUSE_TYPE,0);
        VarModule::set_int(link_object,vars::link::instance::FUSED_ITEM_POST_STATUS,*ITEM_BOMBER_STATUS_KIND_BORN2);
    }
    if kind == *ITEM_KIND_LINKBOMB {
        VarModule::set_int(link_object,vars::link::instance::FUSE_TYPE,0);
        VarModule::set_int(link_object,vars::link::instance::FUSED_ITEM_POST_STATUS,*ITEM_STATUS_KIND_BORN);
    }
    else {
        if kind == *ITEM_KIND_KILLER
        || kind == *ITEM_KIND_BANANAGUN
        || kind == *ITEM_KIND_DOLPHINBOMB {
            VarModule::set_int(link_object,vars::link::instance::FUSE_TYPE,1);
        }
        else {
            VarModule::set_int(link_object,vars::link::instance::FUSE_TYPE,0);
        }
        VarModule::set_int(link_object,vars::link::instance::FUSED_ITEM_POST_STATUS,*ITEM_STATUS_KIND_THROW);
    }
    let item_id = ItemModule::get_have_item_id(owner_module_accessor,0);
    VarModule::set_int(link_object,vars::link::instance::FUSED_ITEM_ID,item_id as i32);
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_HAVED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn arrow_haved_main(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_id);
    let link_object = utils::util::get_battle_object_from_id(owner_id);
    if ItemModule::is_have_item(owner_module_accessor,0) {
        set_fuse_params(weapon);
        VarModule::on_flag(link_object,vars::link::instance::IS_ARROW_FUSED);
    }
    else {
        VarModule::off_flag(link_object,vars::link::instance::IS_ARROW_FUSED);
    }
    if VarModule::is_flag(link_object,vars::link::instance::IS_ARROW_FUSED) {
        let item_id = VarModule::get_int(link_object,vars::link::instance::FUSED_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        LinkModule::remove_model_constraint(item_boma,true);
        if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_boma);
        }
        if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) == false {
            VisibilityModule::set_whole(item_boma,true);
            LinkModule::link(item_boma,*ITEM_LINK_NO_HAVE,(*(weapon.module_accessor)).battle_object_id);
            LinkModule::set_model_constraint_pos_ort(item_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
            PLAY_SE(weapon,Hash40::new("se_link_fuse"));
        }
    }
    if VarModule::is_flag(link_object,vars::link::instance::IS_ARROW_FUSED) {
        MotionModule::change_motion(weapon.module_accessor,Hash40::new("haved"),0.0,1.0,false,0.0,false,false);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor,Hash40::new("haved_hide"),0.0,1.0,false,0.0,false,false);
    }
    weapon.fastshift(L2CValue::Ptr(arrow_haved_main_loop as *const () as _))
}

unsafe fn arrow_haved_main_loop(_weapon: &mut L2CFighterBase) -> L2CValue {
    return L2CValue::I32(0)
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
pub unsafe fn special_n_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bow_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    ArticleModule::change_status_exist(fighter.module_accessor,bow_id,*WN_LINK_BOW_STATUS_KIND_BACK);
    if ArticleModule::is_exist(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        if VarModule::is_flag(fighter.battle_object, vars::link::instance::IS_ARROW_FUSED) {
            let item_id = VarModule::get_int(fighter.battle_object,vars::link::instance::FUSED_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma,true);
            if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_THROW,false);
            }
        }
    }
    ArticleModule::remove_exist(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,ArticleOperationTarget(0));
    return L2CValue::I32(0)
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn arrow_fly_end(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let link_object = utils::util::get_battle_object_from_id(owner_id);
    if VarModule::is_flag(link_object,vars::link::instance::IS_ARROW_FUSED) {
        let item_id = VarModule::get_int(link_object,vars::link::instance::FUSED_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        LinkModule::remove_model_constraint(item_boma,true);
        if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_boma);
            let status = VarModule::get_int(link_object,vars::link::instance::FUSED_ITEM_POST_STATUS);
            StatusModule::change_status_request(item_boma,status,false);
        }
    }
    EffectModule::detach_all(weapon.module_accessor,5);
    return L2CValue::I32(0)
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn arrow_fly_init(weapon: &mut L2CFighterBase) -> L2CValue {
    original!(weapon);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let link_object = utils::util::get_battle_object_from_id(owner_id);
    if VarModule::is_flag(link_object,vars::link::instance::IS_ARROW_FUSED) {
        if VarModule::get_int(link_object,vars::link::instance::FUSE_TYPE) == 1 {
            let lr = PostureModule::lr(weapon.module_accessor);
            weapon.clear_lua_stack();
            lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,12.0*lr,0.0);
            sv_kinetic_energy::set_speed(weapon.lua_state_agent);
            weapon.clear_lua_stack();
            lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,0.0,0.0);
            sv_kinetic_energy::set_accel(weapon.lua_state_agent);
            KineticModule::enable_energy(weapon.module_accessor,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
            AttackModule::set_power_mul(weapon.module_accessor,2.5);
        }
    }
    return L2CValue::I32(0)
}