mod util;
use util::intcode::*;
use itertools::Itertools;

struct Computer {
    cpu: IntcodeCPU,
    queue: Vec<Message>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            cpu: IntcodeCPU::new(program_from_file("input.txt")),
            queue: vec![]
        }
    }

    fn get_messages(&mut self) -> Vec<Message> {
        let mut res = vec![];
        for i in (0..self.cpu.output.len()).step_by(3) {
            res.push(Message::new(
                self.cpu.output[i],
                self.cpu.output[i+1],
                self.cpu.output[i+2],
            ));
        }
        self.cpu.output.clear();
        res
    }

    fn receive(&mut self, message: Message) {
        self.cpu.input.push(message.x);
        self.cpu.input.push(message.y);
    }

    fn receive_nothing(&mut self) {
        self.cpu.input.push(-1);
    }

    fn process_queue(&mut self) -> i64 {
        if self.queue.len() > 0 {
            for message in self.queue.clone() {
                self.receive(message);
            }
            let res = self.queue.len() / 3;
            self.queue.clear();
            res as i64
        } else {
            self.receive_nothing();
            0
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Message {
    destination: i64,
    x: i64,
    y: i64
}

impl Message {
    fn new(destination: i64, x: i64, y: i64) -> Message {
        Message {
            destination,
            x,
            y
        }
    }
}

struct NAT {
    message: Option<Message>
}

impl NAT {
    fn new() -> NAT {
        NAT {
            message: None
        }
    }

    fn receive(&mut self, message: Message) {
        self.message = Some(message);
    }

    fn send(&mut self) -> Message {
        let m = self.message.unwrap();
        self.message = None;
        m
    }
}

fn main() {
    let mut pcs = vec![];
    for i in 0..50 {
        let mut pc = Computer::new();
        pc.cpu.input.push(i);
        pcs.push(pc);
    }

    let mut nat = NAT::new();
    let mut prev_y = None;

    loop {
        for i in 0..50 {
            let pc = &mut pcs[i];

            pc.process_queue();

            pc.cpu.resume_no_input();

            for message in pc.get_messages() {
                if message.destination == 255 {
                    dbg!(message);
                    nat.receive(message);
                } else {
                    pcs[message.destination as usize].queue.push(message);
                }
            }
        }

        let num_idle_pcs =
            pcs.iter().filter(|pc| pc.queue.len() == 0).count();

        if num_idle_pcs == pcs.len() {
            let message = nat.send();
            pcs[0].queue.push(message);

            if Some(message.y) == prev_y {
                dbg!(message.y);
                break;
            } else {
                prev_y = Some(message.y);
            }
        }
    }
}
