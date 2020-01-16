extern crate fsevent;
use std::sync::mpsc::channel;
use std::thread;

extern crate ical;

use std::fs::File;
use std::io::BufReader;

fn main() {
    println!("Hairbraint Service");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("\nStarting...");

    //// working with vcards
    //let buf = BufReader::new(File::open("../data/Contacts/Mitchell Beck.vcf").unwrap());
    //let reader = ical::VcardParser::new(buf);
    //for line in reader {
    //    if let Ok(contact) = line {
    //        let iter = contact.properties.iter();
    //        let mut name_property = iter.filter(|p| p.name == "FN");
    //        if let Some(name) = name_property.next() {
    //            if let Some(value) = &name.value {
    //                println!("Name: {}", value);
    //            }
    //        }
    //    } else {
    //        println!("{:?}", line);
    //    }
    //}

    //// working with ical
    //let buf = BufReader::new(File::open("../data/Calendar/Work.ics").unwrap());

    //let reader = ical::IcalParser::new(buf);

    //for line in reader {
    //    if let Ok(cal) = line {
    //        //println!("{:?}", event);
    //        for event in cal.events.iter() {
    //            println!("---");
    //            let iter = event.properties.iter();
    //            let mut name_property = iter.filter(|p| p.name == "SUMMARY");
    //            if let Some(name) = name_property.next() {
    //                if let Some(value) = &name.value {
    //                    println!("Name: {}", value);
    //                }
    //            }
    //        }
    //    }
    //}


    let (sender, receiver) = channel();

    let _t = thread::spawn(move || {
        let fsevent = fsevent::FsEvent::new(vec!["../data".to_string()]);
        let handle = fsevent.observe_async(sender).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(15)); // sleep five seconds
        fsevent.shutdown_observe(handle);
    });

    loop {
        let duration = std::time::Duration::from_secs(1);
        match receiver.recv_timeout(duration) {
            Ok(val) => println!("{:?}", val),
            Err(e) => match e {
                std::sync::mpsc::RecvTimeoutError::Disconnected => break,
                _ => {} // This is the case where nothing entered the channel buffer (no file mods).
            }
        }
    }
}
