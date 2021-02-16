#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

extern crate log;
extern crate simple_logger;

use serde_xml_rs::from_str;
use simple_logger::SimpleLogger;

fn init_logger() {
    let _ = SimpleLogger::new().init();
}

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[test]
fn simple_struct_from_attributes() {
    init_logger();

    let s = r##"
        <item name="hello" source="world.rs" />
    "##;

    let item: Item = from_str(s).unwrap();

    assert_eq!(
        item,
        Item {
            name: "hello".to_string(),
            source: "world.rs".to_string(),
        }
    );
}

#[test]
fn multiple_roots_attributes() {
    init_logger();

    let s = r##"
        <item name="hello" source="world.rs" />
        <item name="hello" source="world.rs" />
    "##;

    let item: Vec<Item> = from_str(s).unwrap();

    assert_eq!(
        item,
        vec![
            Item {
                name: "hello".to_string(),
                source: "world.rs".to_string(),
            },
            Item {
                name: "hello".to_string(),
                source: "world.rs".to_string(),
            },
        ]
    );
}

#[test]
fn simple_struct_from_attribute_and_child() {
    init_logger();

    let s = r##"
        <item name="hello">
            <source>world.rs</source>
        </item>
    "##;

    let item: Item = from_str(s).unwrap();

    assert_eq!(
        item,
        Item {
            name: "hello".to_string(),
            source: "world.rs".to_string(),
        }
    );
}

#[derive(Debug, Deserialize, PartialEq)]
struct Project {
    name: String,

    #[serde(rename = "item", default)]
    items: Vec<Item>,
}

#[test]
fn nested_collection() {
    init_logger();

    let s = r##"
        <project name="my_project">
            <item name="hello1" source="world1.rs" />
            <item name="hello2" source="world2.rs" />
        </project>
    "##;

    let project: Project = from_str(s).unwrap();

    assert_eq!(
        project,
        Project {
            name: "my_project".to_string(),
            items: vec![
                Item {
                    name: "hello1".to_string(),
                    source: "world1.rs".to_string(),
                },
                Item {
                    name: "hello2".to_string(),
                    source: "world2.rs".to_string(),
                },
            ],
        }
    );
}

#[derive(Debug, Deserialize, PartialEq)]
enum MyEnum {
    A(String),
    B { name: String, flag: bool },
    C,
}

#[derive(Debug, Deserialize, PartialEq)]
struct MyEnums {
    #[serde(rename = "$value")]
    items: Vec<MyEnum>,
}

#[test]
fn collection_of_enums() {
    init_logger();

    let s = r##"
        <enums>
            <A>test</A>
            <B name="hello" flag="true" />
            <C />
        </enums>
    "##;

    let project: MyEnums = from_str(s).unwrap();

    assert_eq!(
        project,
        MyEnums {
            items: vec![
                MyEnum::A("test".to_string()),
                MyEnum::B {
                    name: "hello".to_string(),
                    flag: true,
                },
                MyEnum::C,
            ],
        }
    );
}

#[test]
fn struct_with_flattened_untagged_enum() {
    init_logger();

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(untagged)]
    enum Enum {
        A { field_a: i32 },
        B { field_b: i32 },
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Struct {
        common_int: i32,
        #[serde(flatten)]
        the_enum: Enum,
    }

    let s = r##"
        <struct>
          <common_int>123</common_int>
          <field_a>456</field_a>
        </struct>
    "##;

    let actual: Struct = from_str(s).unwrap();

    assert_eq!(
        actual,
        Struct {
            common_int: 123,
            the_enum: Enum::A { field_a: 456 },
        }
    );
}

#[test]
fn struct_with_flattened_internally_tagged_enum() {
    init_logger();

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(tag = "which")]
    enum Enum {
        A { field_a: i32 },
        B { field_b: i32 },
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Struct {
        common_int: i32,
        #[serde(flatten)]
        the_enum: Enum,
    }

    let s = r##"
        <struct>
          <common_int>123</common_int>
          <which>A</which>
          <field_a>456</field_a>
        </struct>
    "##;

    let actual: Struct = from_str(s).unwrap();

    assert_eq!(
        actual,
        Struct {
            common_int: 123,
            the_enum: Enum::A { field_a: 456 },
        }
    );
}
