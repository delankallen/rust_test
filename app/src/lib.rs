use seed::prelude::*;
use seed::*;
use seed::browser::fetch as fetch;

const ENTER_KEY: &str = "Enter";

#[derive(Default)]
struct Model {
    items: Vec<Item>,
    new_item: String,
    error: Option<String>,
    response_data: Option<String>
}

struct Item {
    id: usize,
    title: String
}

enum Msg {
    FetchedItems(fetch::Result<Vec<String>>),
    Fetched(fetch::Result<String>),
    PostItem(),
    RemoveItem(usize),
    ClearAll(),
    ItemTitleChanged(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        //---- API Response ----
        FetchedItems(resp) => match resp {
            Ok(items) => model.items = items.iter().enumerate().map(|(i, title)| {                
                Item {
                    id: i,
                    title: title.to_string()
                }                
            }).collect(),
            Err(e) => model.error = Some(format!("{:?}", e)),
        }

        Fetched(resp) => match resp {
            Ok(response_data) => model.response_data = Some(response_data),
            Err(e) => model.error = Some(format!("{:?}", e)),
        }

        //---- API Requests ----

        PostItem() => {
            let item = model.new_item.trim();
            if not(item.is_empty()) {
                orders.perform_cmd({
                    let item = model.new_item.clone();
                    async { 
                        Msg::Fetched(post_todo_item(item).await);
                        Msg::FetchedItems(get_todo_items().await)
                    }
                });
                model.new_item.clear();
            }
        }

        RemoveItem(index) => {
            orders.perform_cmd({
                async move { 
                    Msg::Fetched(remove_todo_item(index).await);
                    Msg::FetchedItems(get_todo_items().await)
                }
            });            
        }

        ClearAll() => {
            orders.perform_cmd({                
                model.items.clear();
                async {
                    Msg::Fetched(clear_all_items().await);
                }
            });
        }
        
        //---- Input ----

        ItemTitleChanged(item_title) => {
            model.new_item = item_title;
        },
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            C!["card"],
            div![
                C!["main"],
                view_header(),
                view_todo_input(&model.new_item),
            ],
        ],

        view_items(&model.items)
    ]
}

fn view_header() -> Node<Msg> {
    header![
        C!["header"],
        a![
            attrs!{At::Href => "https://www.lvt.com/", At::Target => "_blank"},
            img![
                attrs!{At::Alt => "LiveView logo", At::Src => "https://cameras.liveviewtech.com/img/LVLogo_small.png"}
            ]
        ],
        h1!["ToDo List"]
    ]
}

fn view_todo_input(new_item: &str) -> Node<Msg> {
    div![
        C!["container"],
        input![
            C!["todo-input"],
            attrs!{At::Placeholder => "What to do?", At::AutoFocus => AtValue::None, At::Value => new_item },
            input_ev(Ev::Input, Msg::ItemTitleChanged),
            keyboard_ev(Ev::KeyDown, |keyboard_event| {
                IF!(keyboard_event.key() == ENTER_KEY => Msg::PostItem())
            })
        ],
        div![
            button![
                C!["btn"],
                ev(Ev::Click, |_| Msg::PostItem()),
                "Save"
            ],
            button![
                C!["btn clear-btn"],
                ev(Ev::Click, |_| Msg::ClearAll()),
                "Clear"
            ],
        ]
        
    ]
}

fn view_items(items: &Vec<Item>) -> Node<Msg> {
    ul![attrs!{At::Id => "item-list"},
    items.iter().map(|item| {
        let id = item.id.clone();
        div![
            li![
                &item.title,
                span![C!["remove-item"],
                    "x",
                    ev(Ev::Click, move |_| Msg::RemoveItem(id)),
                ]
            ]
        ]
    })
]
}

async fn get_todo_items() -> fetch::Result<Vec<String>> {
    Request::new("/api/todo")
        .method(fetch::Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn post_todo_item(todo: String) -> fetch::Result<String> {
    Request::new("/api/newtodo")
        .method(fetch::Method::Post)
        .json(&todo)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn remove_todo_item(index: usize) -> fetch::Result<String> {
    Request::new("/api/removetodo")
        .method(fetch::Method::Delete)
        .json(&index.to_string())?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn clear_all_items() -> fetch::Result<String> {
    Request::new("/api/clearall")
        .method(fetch::Method::Delete)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::FetchedItems(get_todo_items().await) });
    Model::default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
