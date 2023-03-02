enum Property {
    Recipient<String>
}

struct Block {
    time: DateTime,
    length: Duration,
    color: RGBA,
    properties: Vec<Property>,
    available: bool,
}
