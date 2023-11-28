use actix_web::web;
use super::game::*;
pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/api/games")
        .service(get_games)
        .service(get_game_by_id)
        .service(create_game)
        .service(update_game)
        .service(delete_game);

    conf.service(scope);
}
