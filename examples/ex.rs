// based on webmidikit demo
use wmidi_apple::prelude::*;

struct ReceiverEvent {}
// unsafe impl Send for ReceiverEvent { }
// unsafe impl Sync for ReceiverEvent { }

struct MIDIReceiver {
    pub rx: std::sync::mpsc::Receiver<ReceiverEvent>,
}

unsafe impl Send for MIDIReceiver {}
unsafe impl Sync for MIDIReceiver {}

impl MIDIInputObserver for MIDIReceiver {
    fn receive(&mut self, packet_list: &coremidi::PacketList) {
        println!("received packet");
        // todo!()
    }
}
fn main() {
    let mut access = MIDIAccess::new("wmidi");

    // for (_, p) in access.inputs().iter() {
    //     println!("input: {:?}", p);
    // }

    // for (_, p) in access.outputs().iter() {
    //     println!("outputs: {:?}", p);
    // }

    // let noteOn: [UInt8] = [0x90, 0x40, 0x7f]
    // let noteOff: [UInt8] = [0x80, 0x40, 0]
    // let (tx, rx) = std::sync::mpsc::channel();
    // let observer = Box::new(MIDIReceiver { rx });

    // {
    //     let input = access.inputs_mut().prompt_mut().unwrap();
    //     input.set_input_observer(observer);
    // }
    // access.outputs()
    // std::thread::
    // println!("selected {:?}", z);
    let output = access.outputs().prompt();
}
