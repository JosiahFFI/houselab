use std::collections::HashMap;

use houselab_core::{
    Comment, List, Severity,
    inspection::Inspection,
    person::{Client, Inspector, Person, Realtor},
    template::{Id, Section, Template},
};

pub fn inspection() -> Inspection {
    let comments = vec![
        Comment {
            base: "Basic Comment".into(),
            applied: false,
            lists: HashMap::new(),
            entries: HashMap::new(),
            severity: Severity::General,
            summary: false,
        },
        Comment {
            base: "Lists Comment: {{list}}".into(),
            applied: false,
            lists: {
                let mut h = HashMap::new();
                h.insert(
                    "list".into(),
                    List {
                        items: vec!["AB".into(), "XY".into(), "12".into()],
                        selected: 0,
                    },
                );
                h
            },
            entries: HashMap::new(),
            severity: Severity::Functionality,
            summary: false,
        },
        Comment {
            base: "Entry Comment: {{entry}}".into(),
            applied: false,
            lists: HashMap::new(),
            entries: {
                let mut h = HashMap::new();
                h.insert("entry".into(), "foobar".into());
                h
            },
            severity: Severity::Safety,
            summary: false,
        },
        Comment {
            base: "Complex Comment: entry {{entry}} AND list {{list}} in one comment!".into(),
            applied: false,
            lists: {
                let mut h = HashMap::new();
                h.insert(
                    "list".into(),
                    List {
                        items: vec!["AB".into(), "XY".into(), "12".into()],
                        selected: 0,
                    },
                );
                h
            },
            entries: {
                let mut h = HashMap::new();
                h.insert("entry".into(), "foobar".into());
                h
            },
            severity: Severity::Safety,
            summary: false,
        },
    ];

    let id_overview = Id::root(1);

    let mut id_exterior = Id::root(2);
    let id_siding = id_exterior.next().unwrap();

    let mut id_interior = Id::root(3);
    let id_living_room = id_interior.next().unwrap();

    let mut id_kitchen = id_interior.next().unwrap();
    let mut id_appliances = id_kitchen.next().unwrap();
    let id_oven = id_appliances.next().unwrap();
    let id_microwave = id_appliances.next().unwrap();
    let id_range = id_appliances.next().unwrap();

    let sections = vec![
        Section {
            name: "Overview".into(),
            description: Some("An overview of the basic inspection details.".into()),
            inline: false,
            comments: comments.clone(),
            observations: comments.clone(),
            id: id_overview,
            children: vec![],
        },
        Section {
            name: "Exterior".into(),
            description: None,
            inline: false,
            comments: comments.clone(),
            observations: comments.clone(),
            id: id_exterior,
            children: vec![Section {
                name: "Siding".into(),
                description: None,
                inline: false,
                comments: comments.clone(),
                observations: comments.clone(),
                id: id_siding,
                children: vec![],
            }],
        },
        Section {
            name: "Interior".into(),
            description: None,
            inline: false,
            comments: comments.clone(),
            observations: comments.clone(),
            id: id_interior,
            children: vec![
                Section {
                    name: "Living Room".into(),
                    description: None,
                    inline: false,
                    comments: comments.clone(),
                    observations: comments.clone(),
                    id: id_living_room,
                    children: vec![],
                },
                Section {
                    name: "Kitchen".into(),
                    description: None,
                    inline: false,
                    comments: comments.clone(),
                    observations: comments.clone(),
                    id: id_kitchen,
                    children: vec![Section {
                        name: "Appliances".into(),
                        description: None,
                        inline: false,
                        comments: comments.clone(),
                        observations: comments.clone(),
                        id: id_appliances,
                        children: vec![
                            Section {
                                name: "Oven".into(),
                                description: None,
                                inline: true,
                                comments: comments.clone(),
                                observations: comments.clone(),
                                id: id_oven,
                                children: vec![],
                            },
                            Section {
                                name: "Microwave".into(),
                                description: None,
                                inline: true,
                                comments: comments.clone(),
                                observations: comments.clone(),
                                id: id_microwave,
                                children: vec![],
                            },
                            Section {
                                name: "Range".into(),
                                description: None,
                                inline: true,
                                comments: comments.clone(),
                                observations: comments.clone(),
                                id: id_range,
                                children: vec![],
                            },
                        ],
                    }],
                },
            ],
        },
    ];

    let template = Template {
        name: "Test Template".into(),
        description: Some("A hard-coded testing template.".into()),
        last_modified: jiff::Timestamp::now(),
        sections,
        colors: HashMap::new(),
        id_gen: 3,
    };

    let inspector1 = Inspector {
        info: Person {
            name: "Josiah Torgerson".into(),
            phone: Some("509-242-6279".into()),
            email: Some("josiah@firesidefamilyinspections.com".into()),
        },
        licenses: vec![
            ("License 1".into(), "123456".into()),
            ("License 2".into(), "abcdef".into()),
        ],
    };
    let inspector2 = Inspector {
        info: Person {
            name: "Derek Torgerson".into(),
            phone: None,
            email: None,
        },
        licenses: vec![],
    };

    let client = Client {
        info: Person {
            name: "John Doe".into(),
            phone: None,
            email: None,
        },
        realtor: None,
    };

    let dt = jiff::Zoned::now();
    Inspection {
        name: "Test Inspection".into(),
        address: "123 Lullaby Ln, Spokane, WA, 99224".into(),
        date: dt.date(),
        time: dt.time(),
        template,
        inspectors: vec![inspector1, inspector2],
        client,
        seller: None,
        images: HashMap::new(),
    }
}

pub fn people() -> crate::people::People {
    let inspectors = vec![
        Inspector {
            info: Person {
                name: "Josiah Torgerson".into(),
                phone: Some("509-242-6279".into()),
                email: Some("josiah@firesidefamilyinspections.com".into()),
            },
            licenses: vec![
                ("License 1".into(), "123456".into()),
                ("License 2".into(), "abcdef".into()),
            ],
        },
        Inspector {
            info: Person {
                name: "Derek Torgerson".into(),
                phone: None,
                email: None,
            },
            licenses: vec![],
        },
    ];

    let realtors = vec![
        Realtor {
            info: Person {
                name: "Jane Doe".into(),
                phone: Some("123-456-7890".into()),
                email: Some("janed@example.com".into()),
            },
            firm: "Green Fields Realty".into(),
        },
        Realtor {
            info: Person {
                name: "Bob Smith".into(),
                phone: Some("987-654-3210".into()),
                email: Some("bsmith@example.net".into()),
            },
            firm: "Smith Real Estate".into(),
        },
    ];

    crate::people::People {
        inspectors,
        realtors,
    }
}
