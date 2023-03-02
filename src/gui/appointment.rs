
struct Service {
    description: String,
    time: DateTime,
    prep: Duration,
    prep_provider: String,
    service: Duration,
    service_provider: String,
    process: Duration,
    process_provider: String
}

impl Into<(Block, Block, Block)> for Service {
    fn into(self) -> (Block, Block, Block) {
        (
            Block {
                color: RGBA::new(1.0,1.0,1.0,1.0),
                time: self.time,
                duration: self.prep,
                properties: vec![(String::from("recipient"), String::from("Jaden Carver")]
            },
            Block {
                color: RGBA::new(1.0,1.0,1.0,1.0),
                time: self.time,
                duration: self.service,
                properties: vec![Property::Recipient(String::from("Jaden Carver"))]
            },
            Block {
                color: RGBA::new(1.0,1.0,1.0,1.0),
                time: self.time,
                duration: self.process,
                properties: vec![(String::from("recipient"), String::from("Jaden Carver")]
            }
        )
    }
}

struct Appointment {
    // "Jaden Carver <tel:(917) 484-0438>"
    // "Jaden Carver <jaden.carver@gmail.com>"
    contact: String,
    date: Date,
    services: Vec<Service>
}

