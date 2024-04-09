#![no_std]
use gstd::{msg, exec};
use pebbles_game_io::*;

static mut PEBBLES: Option<Pebbles> = None;

#[no_mangle]
extern fn init() {
    let name: String = msg::load().expect("Failed to decode Pebbles");
    let age = exec::block_timestamp();

    let tmg = Pebbles {
        name, 
        age,
    };

    unsafe {
        PEBBLES = Some(tmg);
    }
}

#[no_mangle]
extern "C" fn handle() {
    let action: Action = msg::load().expect("Unable to decode `Action`");
    let peb = unsafe { PEBBLES.get_or_insert(Default::default()) };
    match action {
        Action::Name => {
            msg::reply(PebEvent::Name(peb.name.clone()), 0)
                .expect("Error in a reply `TmgEvent::Name`");
        }
        PebAction::Age => {
            let age = exec::block_timestamp() - peb.age;
            msg::reply(PebEvent::Age(age), 0)
                .expect("Error in a reply `TmgEvent::Age`");
        }
    };
}

#[no_mangle]
extern fn state() {
    let peb = unsafe { PEBBLES.take().expect("Error in taking current state") };
    msg::reply(peb, 0).expect("Failed to reply state");
}
