use yew::prelude::*;
use crate::block::{Block, Transaction};
use crate::blockchain::Blockchain;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
struct AppState {
    blockchain: Rc<Blockchain>,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| AppState {
        blockchain: Rc::new(Blockchain::new()),
    });

    let add_transaction = {
        let state = state.clone();
        Callback::from(move |(sender, receiver, amount): (String, String, f64)| {
            let mut blockchain = (*state.blockchain).clone();
            blockchain.add_transaction(Transaction { sender, receiver, amount });
            state.set(AppState { blockchain: Rc::new(blockchain) });
        })
    };

    let mine_block = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut blockchain = (*state.blockchain).clone();
            blockchain.add_block();
            state.set(AppState { blockchain: Rc::new(blockchain) });
        })
    };

    html! {
        <div>
            <h1>{ "Mi Blockchain" }</h1>
            <TransactionForm on_submit={add_transaction} />
            <button onclick={mine_block}>{ "Minar Bloque" }</button>
            <BlockchainDisplay blockchain={(*state.blockchain).clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TransactionFormProps {
    on_submit: Callback<(String, String, f64)>,
}

#[function_component(TransactionForm)]
fn transaction_form(props: &TransactionFormProps) -> Html {
    let sender = use_state(|| String::new());
    let receiver = use_state(|| String::new());
    let amount = use_state(|| String::new());

    let on_submit = {
        let sender = sender.clone();
        let receiver = receiver.clone();
        let amount = amount.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Ok(amount) = (*amount).parse::<f64>() {
                on_submit.emit(((*sender).clone(), (*receiver).clone(), amount));
                sender.set(String::new());
                receiver.set(String::new());
                amount.set(String::new());
            }
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <input
                type="text"
                placeholder="Sender"
                value={(*sender).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    sender.set(input.value());
                })}
            />
            <input
                type="text"
                placeholder="Receiver"
                value={(*receiver).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    receiver.set(input.value());
                })}
            />
            <input
                type="number"
                placeholder="Amount"
                value={(*amount).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    amount.set(input.value());
                })}
            />
            <button type="submit">{ "Agregar Transacción" }</button>
        </form>
    }
}

#[derive(Properties, PartialEq)]
struct BlockchainDisplayProps {
    blockchain: Blockchain,
}

#[function_component(BlockchainDisplay)]
fn blockchain_display(props: &BlockchainDisplayProps) -> Html {
    let is_valid = props.blockchain.is_chain_valid();
    let blocks: Vec<Html> = props.blockchain.chain.iter().enumerate().map(|(i, block)| {
        html! {
            <div>
                <h3>{ format!("Block #{}", i) }</h3>
                <p>{ format!("Timestamp: {}", block.timestamp) }</p>
                <p>{ format!("Transactions: {:?}", block.transactions) }</p>
                <p>{ format!("Previous Hash: {}", block.previous_hash) }</p>
                <p>{ format!("Hash: {}", block.hash) }</p>
                <p>{ format!("Nonce: {}", block.nonce) }</p>
            </div>
        }
    }).collect();

    html! {
        <div>
            <h2>{ format!("Is blockchain valid? {}", is_valid) }</h2>
            { blocks }
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}