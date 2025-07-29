pub mod block;
pub mod blockchain;

use yew::prelude::*;
use crate::block::Transaction;
use crate::blockchain::Blockchain;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, PartialEq)]
struct AppState {
    blockchain: Rc<Blockchain>,
    message: Option<String>, // Campo para mensajes de alerta
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| AppState {
        blockchain: Rc::new(Blockchain::new()),
        message: None,
    });

    let add_transaction = {
        let state = state.clone();
        Callback::from(move |(sender, receiver, amount): (String, String, String)| {
            let mut blockchain = (*state.blockchain).clone();
            if let Ok(amount) = amount.parse::<f64>() {
                blockchain.add_transaction(Transaction { sender, receiver, amount });
                state.set(AppState {
                    blockchain: Rc::new(blockchain),
                    message: Some("Transacción agregada!".to_string()),
                });
            } else {
                state.set(AppState {
                    blockchain: state.blockchain.clone(),
                    message: Some("Cantidad inválida".to_string()),
                });
            }
        })
    };

    let mine_block = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut blockchain = (*state.blockchain).clone();
            blockchain.add_block();
            state.set(AppState {
                blockchain: Rc::new(blockchain),
                message: Some("Bloque minado!".to_string()),
            });
        })
    };

    html! {
        <div>
            <h1>{ "Mi Blockchain" }</h1>
            if let Some(msg) = &state.message {
                <p style="color: green;">{ msg }</p>
            }
            <TransactionForm on_submit={add_transaction} />
            <button onclick={mine_block}>{ "Minar Bloque" }</button>
            <BlockchainDisplay blockchain={(*state.blockchain).clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TransactionFormProps {
    on_submit: Callback<(String, String, String)>,
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
            on_submit.emit(((*sender).clone(), (*receiver).clone(), (*amount).clone()));
            sender.set(String::new());
            receiver.set(String::new());
            amount.set(String::new());
        })
    };

    let on_sender_input = {
        let sender = sender.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            sender.set(input.value());
        })
    };

    let on_receiver_input = {
        let receiver = receiver.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            receiver.set(input.value());
        })
    };

    let on_amount_input = {
        let amount = amount.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            amount.set(input.value());
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <input
                type="text"
                placeholder="Sender"
                value={(*sender).clone()}
                oninput={on_sender_input}
            />
            <input
                type="text"
                placeholder="Receiver"
                value={(*receiver).clone()}
                oninput={on_receiver_input}
            />
            <input
                type="number"
                placeholder="Amount"
                value={(*amount).clone()}
                oninput={on_amount_input}
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
            <div class="block">
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