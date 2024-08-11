use async_channel::{unbounded, Receiver};
#[cfg(target_arch = "wasm32")]
use bevy::tasks::IoTaskPool;
use bevy::{prelude::*, tasks::futures_lite::StreamExt};
use cainome::cairo_serde::ContractAddress;
use starknet_crypto::Felt;
use torii_client::client::Client;
use torii_grpc::{
    client::EntityUpdateStreaming,
    types::{schema::Entity as DojoEntity, EntityKeysClause, KeysClause},
};

use crate::bindgen::bevy::components::moves::Moves;

pub struct ToriiPlugin;
impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_torii_client);
        app.add_systems(Update, (parse_dojo_entity, spawn_or_update).chain());
    }
}

#[derive(Resource)]
pub struct ToriiClient {
    pub entity_rx: Receiver<DojoEntity>,
    #[cfg(not(target_arch = "wasm32"))]
    pub runtime: tokio::runtime::Runtime,
}

fn setup_torii_client(mut commands: Commands) {
    let torii_url = "http://0.0.0.0:8080".to_string();
    let rpc_url = "http://0.0.0.0:5050".to_string();
    let relay_url = "/ip4/127.0.0.1/tcp/9090".to_string();
    let world = Felt::from_hex_unchecked(
        "0x5d97c46d046f442f125b6cc83057e97ee6e848c4921126acd8ae9d17b55b369",
    );
    let (tx, rx) = unbounded::<DojoEntity>();

    #[cfg(target_arch = "wasm32")]
    let pool = IoTaskPool::get();

    #[cfg(target_arch = "wasm32")]
    pool.spawn(async move {
        info!("Setting up Torii client");
        let client = Client::new(torii_url, rpc_url, relay_url, world)
            .await
            .unwrap();
        let mut rcv = client
            .on_entity_updated(vec![EntityKeysClause::Keys(KeysClause {
                keys: vec![],
                pattern_matching: torii_grpc::types::PatternMatching::VariableLen,
                models: vec![],
            })])
            .await
            .unwrap();

        info!("Torii client setup");
        while let Some(Ok((_, entity))) = rcv.next().await {
            // info!("Received Dojo entity: {:?}", entity);
            tx.send(entity).await.unwrap();
        }
    })
    .detach();

    #[cfg(not(target_arch = "wasm32"))]
    let runtime = tokio::runtime::Runtime::new().unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    runtime.spawn(async move {
        info!("Setting up Torii client");
        let client = Client::new(torii_url, rpc_url, relay_url, world)
            .await
            .unwrap();
        let mut rcv: EntityUpdateStreaming = client
            .on_entity_updated(vec![EntityKeysClause::Keys(KeysClause {
                keys: vec![],
                pattern_matching: torii_grpc::types::PatternMatching::VariableLen,
                models: vec![],
            })])
            .await
            .unwrap();

        info!("Torii client setup");
        while let Some(Ok((_, entity))) = rcv.next().await {
            // info!("Received Dojo entity: {:?}", entity);
            tx.send(entity).await.unwrap();
        }
    });

    info!("Torii client setup task spawned");

    commands.insert_resource(ToriiClient {
        entity_rx: rx,
        #[cfg(not(target_arch = "wasm32"))]
        runtime: runtime,
    });
}

#[derive(Debug, Component)]
struct TempDojoEntity {
    moves: Moves,
}

fn parse_dojo_entity(torii_client: Res<ToriiClient>, mut commands: Commands) {
    match torii_client.entity_rx.try_recv() {
        Ok(dojo_entity) => {
            info!("Received Dojo entity: {:?}", dojo_entity);
            let length = dojo_entity.models.len();
            if length == 1 {
                if let Some(moves) = dojo_to_bevy(&dojo_entity) {
                    info!(
                        "Spawned Bevy equivalent entity from Dojo entity: {:?}",
                        moves
                    );
                    let dojo_entity = TempDojoEntity { moves };
                    commands.spawn(dojo_entity);
                }
            }
        }
        Err(err) => {
            if err != async_channel::TryRecvError::Empty {
                error!("Error receiving dojo entity: {:?}", err);
            }
        }
    }
}

// this function assumes that the dojo entity is a Moves entity
fn dojo_to_bevy(dojo_entity: &DojoEntity) -> Option<Moves> {
    let model = &dojo_entity.models[0];
    let name = model.name.split('-').last().unwrap();
    info!("Model name: {:?}", model.name);
    match name {
        "Moves" => {
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

            Some(Moves {
                player,
                remaining,
                can_move,
            })
        }
        _ => None,
    }
}

fn spawn_or_update(
    mut commands: Commands,
    mut query_dojo_entity: Query<(Entity, &mut TempDojoEntity)>,
    mut query_bevy_entity: Query<&mut Moves>,
) {
    for (id, dojo_entity) in query_dojo_entity.iter_mut() {
        let player = dojo_entity.moves.player.clone();
        let new_remaining = dojo_entity.moves.remaining.clone();
        let new_can_move = dojo_entity.moves.can_move.clone();

        let mut is_new = true;

        for mut existing_moves in query_bevy_entity.iter_mut() {
            if existing_moves.player == player {
                existing_moves.remaining = new_remaining;
                is_new = false;
            }
        }
        if is_new {
            commands.spawn(Moves {
                player,
                remaining: new_remaining,
                can_move: new_can_move,
            });
        }

        commands.entity(id).despawn();
    }
}
