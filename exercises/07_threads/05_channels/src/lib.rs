use std::sync::mpsc::{Receiver, Sender};

use data::TicketDraft;
use store::TicketStore;

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server the thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();

    while let Ok(cmd) = receiver.recv() {
        match cmd {
            Command::Insert(ticket_draft) => {
                store.add_ticket(ticket_draft);
            }
        }
    }
}
