pub mod agents;
use agents::*;
use crossbeam::crossbeam_channel::bounded;
use std::thread;

fn main() {
    let (s_enter, r_enter): Channel = bounded(0);
    let (s_leave, r_leave): Channel = bounded(0);
    let room = Room::new(r_enter, r_leave);

    let (s_on, r_on): Channel = bounded(0);
    let (s_off, r_off): Channel = bounded(0);
    let light = Light::new(r_on, r_off);

    let (s_once, r_once): Channel = bounded(0);
    let (s_chance, r_chance): Channel = bounded(0);

    let num_prisoners = 100;

    let counter_prisoner = CounterPrisoner::new(
        num_prisoners,
        s_enter.clone(),
        s_leave.clone(),
        s_off.clone(),
        s_on.clone(),
        s_once.clone(),
        s_chance.clone(),
    );
    let warden = Warden::new(num_prisoners, r_once, r_chance);

    let mut thread_handles = vec![];

    for id in 1..=num_prisoners - 1 {
        let prisoner = Prisoner::new(
            id,
            s_enter.clone(),
            s_leave.clone(),
            s_off.clone(),
            s_on.clone(),
            s_once.clone(),
        );

        thread_handles.push(thread::spawn(move || {
            prisoner.run();
        }));
    }

    thread_handles.push(thread::spawn(move || {
        counter_prisoner.run();
    }));

    thread::spawn(move || {
        light.run();
    });

    thread::spawn(move || {
        room.run();
    });

    thread_handles.push(thread::spawn(move || {
        warden.run();
    }));

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
