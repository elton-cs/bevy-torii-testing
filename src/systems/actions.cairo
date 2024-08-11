// use dojo_starter::models::Direction;
// use dojo_starter::models::Position;

// define the interface
#[dojo::interface]
trait IActions {
    fn spawn_move(ref world: IWorldDispatcher);
    fn incr(ref world: IWorldDispatcher);
    fn decr(ref world: IWorldDispatcher);
    // fn move(ref world: IWorldDispatcher, direction: Direction);
}

// dojo decorator
#[dojo::contract]
mod actions {
    use super::{IActions};
    use starknet::{ContractAddress, get_caller_address};
    // use dojo_starter::models::{Position, Vec2, Moves, Direction, DirectionsAvailable};
    use dojo_starter::models::{Moves};


    #[abi(embed_v0)]
    impl ActionsImpl of IActions<ContractState> {
        fn spawn_move(ref world: IWorldDispatcher) {
            let caller = get_caller_address();
            let moves = Moves { player: caller, remaining: 100, can_move: false };
            set!(world, (moves));
        }

        fn incr(ref world: IWorldDispatcher) {
            let caller = get_caller_address();

            let moves = get!(world, caller, Moves);
            let new_moves = Moves {
                player: moves.player, remaining: moves.remaining + 5, can_move: moves.can_move
            };
            set!(world, (new_moves));
        }

        fn decr(ref world: IWorldDispatcher) {
            let caller = get_caller_address();

            let moves = get!(world, caller, Moves);
            let new_moves = Moves {
                player: moves.player, remaining: moves.remaining - 5, can_move: moves.can_move
            };
            set!(world, (new_moves));
        }
    }
}
