use crate::bindgen::bevy::components::moves::Moves;
use bevy::prelude::*;
use cainome::cairo_serde::ContractAddress;
use dojo_types::schema::Struct as DojoStruct;
use torii_grpc::types::schema::Entity as DojoEntity;

pub trait ToriiToBevy<T> {
    fn dojo_model_to_bevy_component(model: &DojoStruct) -> T;
}

impl ToriiToBevy<Moves> for Moves {
    fn dojo_model_to_bevy_component(model: &DojoStruct) -> Moves {
        let player = model.children[0]
            .ty
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();

        let player = ContractAddress::from(player);

        let remaining = model.children[1]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();

        let can_move = model.children[2]
            .ty
            .as_primitive()
            .unwrap()
            .as_bool()
            .unwrap();

        Moves {
            player,
            remaining,
            can_move,
        }
    }
}
