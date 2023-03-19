

use common::request;
use uuid::Uuid;
use yew::prelude::*;
use yew_hooks::use_async;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(Game)]
pub fn game(props: &Props) -> Html {
    let user_access = {
        let id = props.id.clone();
        use_async(async move {
            request::join_game(id)
                .await
                .map_err(|err| err.to_string())
        })
    };  

    {
        let user_access = user_access.clone();
        use_effect_with_deps(move |_| {
            user_access.run();
        }, ());
    }

    

    if !user_access.loading {
        if let Some(user_access) = &user_access.data {
            html! {
                <game>
                </game>
            }
        } else {
            if let Some(err) = &user_access.error {
                html! {
                    <error>
                        { format!("Error: {}", err) }
                    </error>
                }
            } else {
                html! {}
            }
        }
    } else {
        html! {
            <loading>
                {"Loading ..."}
            </loading>
        }
    }

    
}

