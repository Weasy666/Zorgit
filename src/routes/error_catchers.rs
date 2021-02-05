use rocket::{Catcher, Request};
use crate::models::{Config, User};
use crate::models::template::error::*;


pub fn get_catchers() -> Vec<Catcher> {
    catchers![
        unauthorized,
        forbidden,
        not_found,
        unprocessable_entity,
        internal_server_error,
    ]
}

#[catch(401)]
fn unauthorized(req: &Request<'_>) -> Error401Template {
    let logged_user = req.guard::<User>().succeeded();

    Error401Template {
        user: logged_user,
        debug: Config::global().debug,
    }
}

#[catch(403)]
fn forbidden(req: &Request<'_>) -> Error403Template {
    let logged_user = req.guard::<User>().succeeded();

    Error403Template {
        user: logged_user,
        debug: Config::global().debug,
    }
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Error404Template {
    let logged_user = req.guard::<User>().succeeded();

    Error404Template {
        user: logged_user,
        debug: Config::global().debug,
    }
}

#[catch(422)]
fn unprocessable_entity(req: &Request<'_>) -> Error422Template {
    let logged_user = req.guard::<User>().succeeded();

    Error422Template {
        user: logged_user,
        debug: Config::global().debug,
    }
}

#[catch(500)]
fn internal_server_error(req: &Request<'_>) -> Error500Template {
    let logged_user = req.guard::<User>().succeeded();

    Error500Template {
        user: logged_user,
        debug: Config::global().debug,
    }
}