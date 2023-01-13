use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct LightOfFoliarIncisionEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for LightOfFoliarIncisionEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::CriticalBase, "裁叶萃光被动", 0.01 * refine + 0.03);

        let rate = self.rate;
        attribute.add_edge1(
            AttributeName::ElementalMastery,
            AttributeName::ExtraDmgNormalAttack,
            Box::new(move |em, _| em * (0.3 * refine + 0.9) * rate),
            Box::new(|grad, em, _| (0.0, 0.0)),
            "裁叶萃光被动等效"
        );
    }
}

pub struct LightOfFoliarIncision;

impl WeaponTrait for LightOfFoliarIncision {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::LightOfFoliarIncision,
        internal_name: "todo", // todo
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: None,
        #[cfg(not(target_family = "wasm"))]
        chs: ""
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01,
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::LightOfFoliarIncision { rate } => rate,
            _ => 0.0
        };
        Some(Box::new(LightOfFoliarIncisionEffect {
            rate
        }))
    }
}
