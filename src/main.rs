use gilrs::{Gilrs, Button, Event, EventType};
use enigo::{Enigo, Key, KeyboardControllable};
use std::collections::HashMap;
use std::env;

fn main() {
    let mut enigo = Enigo::new();
    let mut gilrs = Gilrs::new().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Needs at least 1 arg, no args were provided.");
        return;
    }
    let mode = args[1].to_lowercase();

    // South North Z            West
    // East  C     RightTrigger LeftTrigger

    let mut map = HashMap::new();

    match mode.as_ref() {
        "mpv" => {
            //map.insert(Button::DPadUp,     Key::UpArrow); // too easy to bump
            //map.insert(Button::DPadDown,   Key::DownArrow); // too easy to bump
            map.insert(Button::DPadLeft,     Key::LeftArrow);
            map.insert(Button::DPadRight,    Key::RightArrow);

            map.insert(Button::East,         Key::Layout(',')); // previous frame
            map.insert(Button::C,            Key::Layout('.')); // next frame
            map.insert(Button::RightTrigger, Key::Layout('b')); // previous subtitle, usually Ctrl-left but I also mapped to b in mpv
            map.insert(Button::LeftTrigger,  Key::Layout('n')); // next subtitle, usually Ctrl-right but I also mapped to n in mpv

            map.insert(Button::South,        Key::Space);
        }
        "emulator" => {
            map.insert(Button::DPadUp,       Key::UpArrow);
            map.insert(Button::DPadDown,     Key::DownArrow);
            map.insert(Button::DPadLeft,     Key::LeftArrow);
            map.insert(Button::DPadRight,    Key::RightArrow);

            map.insert(Button::East,         Key::Layout('a'));
            map.insert(Button::C,            Key::Layout('b'));
            map.insert(Button::RightTrigger, Key::Layout('x'));
            map.insert(Button::LeftTrigger,  Key::Layout('y'));

            map.insert(Button::South,        Key::Return); // start
            map.insert(Button::North,        Key::Shift);  // select
            map.insert(Button::Z,            Key::Layout('l'));
            map.insert(Button::West,         Key::Layout('r'));
        }
        "wanikani" => {
            map.insert(Button::DPadUp,       Key::UpArrow);
            map.insert(Button::DPadDown,     Key::DownArrow);
            map.insert(Button::DPadLeft,     Key::LeftArrow);
            map.insert(Button::DPadRight,    Key::RightArrow);

            map.insert(Button::East,         Key::Space);
            //map.insert(Button::C,            Key::Return);
            //map.insert(Button::RightTrigger, Key::Return);
            map.insert(Button::LeftTrigger,  Key::Return);
        }
        "music" => {
            map.insert(Button::DPadUp,       Key::UpArrow);
            map.insert(Button::DPadDown,     Key::DownArrow);
            map.insert(Button::DPadLeft,     Key::LeftArrow);
            map.insert(Button::DPadRight,    Key::RightArrow);

            map.insert(Button::East,         Key::Layout('b')); // seek back
            map.insert(Button::C,            Key::Layout('f')); // seek forward
            map.insert(Button::RightTrigger, Key::Meta); // use for switching between windows with the joystick
            map.insert(Button::LeftTrigger,  Key::Layout('p')); // pause
        }
        "key" => {
            map.insert(Button::DPadUp,       Key::UpArrow);
            map.insert(Button::DPadDown,     Key::DownArrow);
            map.insert(Button::DPadLeft,     Key::LeftArrow);
            map.insert(Button::DPadRight,    Key::RightArrow);

            let key_string = args[2].to_lowercase();
            if key_string.len() != 1 {
                println!("specified key needs to be exactly one character, multiple characters were provided.");
                return
            }
            let key = key_string.chars().next().unwrap();
            map.insert(Button::East,         Key::Layout(key));
            map.insert(Button::C,            Key::Layout(key));
            map.insert(Button::RightTrigger, Key::Layout(key));
            map.insert(Button::LeftTrigger,  Key::Layout(key));
        }
        _ => {
            println!("unknown mode: {}", mode);
            return;
        }
    }

    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed (button, _) => {
                    if let Some(key) = map.get(&button) {
                        enigo.key_down(key.clone());
                    }
                    else {
                        println!("{:?} Unhandled event from {}: {:?}", time, id, event);
                    }
                }
                EventType::ButtonReleased (button, _) => {
                    if let Some(key) = map.get(&button) {
                        enigo.key_up(key.clone());
                    }
                    else {
                        println!("{:?} Unhandled event from {}: {:?}", time, id, event);
                    }
                }
                _ => { }
            }
        }
    }
}
