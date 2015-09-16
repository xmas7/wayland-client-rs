
#[derive(Debug)]
pub struct Protocol {
    pub name: String,
    pub copyright: Option<String>,
    pub interfaces: Vec<Interface>
}

impl Protocol {
    pub fn new(name: String) -> Protocol {
        Protocol {
            name: name,
            copyright: None,
            interfaces: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub version: u32,
    pub description: Option<(String, String)>,
    pub requests: Vec<Request>,
    pub events: Vec<Event>,
    pub enums: Vec<Enum>
}

impl Interface {
    pub fn new() -> Interface {
        Interface {
            name: String::new(),
            version: 1,
            description: None,
            requests: Vec::new(),
            events: Vec::new(),
            enums: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub name: String,
    pub typ: Option<Type>,
    pub since: u16,
    pub description: Option<(String, String)>,
    pub args: Vec<Arg>,
    pub type_index: usize
}

impl Request {
    pub fn new() -> Request {
        Request {
            name: String::new(),
            typ: None,
            since: 1,
            description: None,
            args: Vec::new(),
            type_index: 0
        }
    }

    pub fn all_null(&self) -> bool {
        self.args.iter().all(|a| !((a.typ == Type::Object || a.typ == Type::NewId) && a.interface.is_some()))
    }
}

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub since: u16,
    pub description: Option<(String, String)>,
    pub args: Vec<Arg>,
    pub type_index: usize
}

impl Event {
    pub fn new() -> Event {
        Event {
            name: String::new(),
            since: 1,
            description: None,
            args: Vec::new(),
            type_index: 0
        }
    }

    pub fn all_null(&self) -> bool {
        self.args.iter().all(|a| !((a.typ == Type::Object || a.typ == Type::NewId) && a.interface.is_some()))
    }
}

#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub typ: Type,
    pub interface: Option<String>,
    pub summary: Option<String>,
    pub description: Option<(String, String)>,
    pub allow_null: bool
}

impl Arg {
    pub fn new() -> Arg {
        Arg {
            name: String::new(),
            typ: Type::Object,
            interface: None,
            summary: None,
            description: None,
            allow_null: false
        }
    }
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub since: u16,
    pub description: Option<(String, String)>,
    pub entries: Vec<Entry>
}

impl Enum {
    pub fn new() -> Enum {
        Enum {
            name: String::new(),
            since: 1,
            description: None,
            entries: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub value: String,
    pub since: u16,
    pub description: Option<(String, String)>,
    pub summary: Option<String>
}

impl Entry {
    pub fn new() -> Entry {
        Entry {
            name: String::new(),
            value: "0".to_owned(),
            since: 1,
            description: None,
            summary: None,
        }
    }
}

#[derive(Debug,PartialEq,Eq,Copy,Clone)]
pub enum Type {
    Int,
    Uint,
    Fixed,
    String,
    Object,
    NewId,
    Array,
    Fd,
    Destructor
}

impl Type {
    pub fn nullable(&self) -> bool {
        match *self {
            Type::String | Type::Object | Type::NewId | Type::Array => true,
            _ => false
        }
    }

    pub fn rust_type(&self) -> &'static str {
        match *self {
            Type::Int => "i32",
            Type::Uint => "u32",
            Type::Fixed => "f32",
            Type::Array => "wl_array",
            Type::Fd => "::std::os::unix::io::RawFd",
            Type::String => "String",
            _ => "()" // FIXME
        }
    }
}
