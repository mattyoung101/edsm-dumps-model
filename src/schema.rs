pub mod criteria;
pub mod types;

use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, Write};

use serde_json::Value;

use criteria::Criteria;
use types::{ObjectScheme, Type, Types};

#[derive(Debug, Clone)]
pub struct SchemaGenerator {
    criteria: Criteria,
    types: Types,
}

impl SchemaGenerator {
    pub fn new(criteria: Criteria) -> SchemaGenerator {
        SchemaGenerator {
            criteria,
            types: Types::empty(),
        }
    }

    pub fn add_value(&mut self, val: Value) {
        let typ = Type::from_value(&self.criteria, val);
        self.types.add(typ);
    }

    pub fn build(self) -> Schema {
        let mut builder = SchemaBuilder::new();
        let root = SchemaType::parse(&mut builder, self.types);
        builder.build(root)
    }
}

#[derive(Debug, Clone)]
pub struct Schema {
    root: SchemaType,
    structs: BTreeMap<u64, Struct>,
    enums: BTreeMap<u64, Enum>,
}

impl Schema {
    pub fn print<W: Write>(&self, mut w: W) -> io::Result<()> {
        if self.root.is_struct() {
            let root = self.structs.get(&0).unwrap();
            root.print(&mut w, "Root")?;
        } else if self.root.is_enum() {
            let root = self.enums.get(&0).unwrap();
            root.print(&mut w, "Root")?;
        } else {
            writeln!(w, "pub struct Root({})", self.root)?;
            return Ok(());
        }

        for (id, s) in self.structs.iter() {
            writeln!(w)?;
            s.print(&mut w, &format!("Struct{}", id))?;
        }

        for (id, e) in self.enums.iter() {
            writeln!(w)?;
            e.print(&mut w, &format!("Enum{}", id))?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct SchemaBuilder {
    structs: BTreeMap<u64, Struct>,
    enums: BTreeMap<u64, Enum>,
    id: u64,
}

impl SchemaBuilder {
    fn new() -> SchemaBuilder {
        SchemaBuilder {
            structs: BTreeMap::new(),
            enums: BTreeMap::new(),
            id: 0,
        }
    }

    fn add_struct(&mut self, id: u64, st: Struct) {
        self.structs.insert(id, st);
    }

    fn add_enum(&mut self, id: u64, e: Enum) {
        self.enums.insert(id, e);
    }

    fn next_id(&mut self) -> u64 {
        let id = self.id;
        self.id += 1;
        id
    }

    fn build(self, root: SchemaType) -> Schema {
        Schema {
            root,
            structs: self.structs,
            enums: self.enums,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaType {
    is_nullable: bool,
    typ: SchemaTypes,
}

impl SchemaType {
    fn parse(builder: &mut SchemaBuilder, types: Types) -> SchemaType {
        let is_nullable = types.is_nullable();

        match types.variants_count() {
            0 => SchemaType {
                is_nullable,
                typ: SchemaTypes::Unit,
            },
            1 => {
                let t = types
                    .into_iter()
                    .filter(|t| *t != Type::Null)
                    .nth(0)
                    .unwrap();
                let typ = SchemaTypes::parse(builder, t);

                SchemaType { is_nullable, typ }
            }
            _ => {
                let mut e = Enum::new();
                let id = builder.next_id();

                for t in types.into_iter().filter(|t| *t != Type::Null) {
                    let typ = SchemaTypes::parse(builder, t);
                    let v = typ.varinat();
                    e.add(v, typ);
                }

                builder.add_enum(id, e);
                SchemaType {
                    is_nullable,
                    typ: SchemaTypes::Enum(id),
                }
            }
        }
    }

    fn is_struct(&self) -> bool {
        if let SchemaTypes::Struct(_) = self.typ {
            true
        } else {
            false
        }
    }

    fn is_enum(&self) -> bool {
        if let SchemaTypes::Enum(_) = self.typ {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for SchemaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_nullable {
            write!(f, "Option<{}>", self.typ)
        } else {
            write!(f, "{}", self.typ)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SchemaTypes {
    Unit,
    Bool,
    U64,
    I64,
    Float,
    String,
    Array(Box<SchemaType>),
    Struct(u64),
    Enum(u64),
}

impl SchemaTypes {
    fn parse(builder: &mut SchemaBuilder, t: Type) -> SchemaTypes {
        match t {
            Type::Null => unreachable!(),
            Type::Bool => SchemaTypes::Bool,
            Type::U64 => SchemaTypes::U64,
            Type::I64 => SchemaTypes::I64,
            Type::Float => SchemaTypes::Float,
            Type::String => SchemaTypes::String,
            Type::Array(ts) => {
                let t = SchemaType::parse(builder, ts);
                SchemaTypes::Array(Box::new(t))
            }
            Type::Object(_, o) => {
                let id = builder.next_id();
                let obj = Struct::parse(builder, o);
                builder.add_struct(id, obj);
                SchemaTypes::Struct(id)
            }
        }
    }

    fn varinat(&self) -> Variant {
        match self {
            SchemaTypes::Unit => Variant::primitive("Unit"),
            SchemaTypes::Bool => Variant::primitive("Bool"),
            SchemaTypes::U64 => Variant::primitive("U64"),
            SchemaTypes::I64 => Variant::primitive("I64"),
            SchemaTypes::Float => Variant::primitive("Float"),
            SchemaTypes::String => Variant::primitive("String"),
            SchemaTypes::Array(_) => Variant::primitive("Array"),
            SchemaTypes::Struct(id) => Variant::Struct(*id),
            SchemaTypes::Enum(id) => Variant::Enum(*id),
        }
    }
}

impl fmt::Display for SchemaTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchemaTypes::Unit => write!(f, "()"),
            SchemaTypes::Bool => write!(f, "bool"),
            SchemaTypes::U64 => write!(f, "u64"),
            SchemaTypes::I64 => write!(f, "i64"),
            SchemaTypes::Float => write!(f, "f64"),
            SchemaTypes::String => write!(f, "String"),
            SchemaTypes::Array(typ) => write!(f, "Vec<{}>", typ),
            SchemaTypes::Struct(id) => write!(f, "Struct{}", id),
            SchemaTypes::Enum(id) => write!(f, "Enum{}", id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Struct(BTreeMap<String, SchemaType>);

impl Struct {
    fn parse(builder: &mut SchemaBuilder, obj: ObjectScheme) -> Struct {
        let mut fields = BTreeMap::new();

        for (k, ts) in obj.into_iter() {
            let t = SchemaType::parse(builder, ts);
            fields.insert(k, t);
        }

        Struct(fields)
    }

    pub fn print<W: Write>(&self, mut w: W, type_name: &str) -> io::Result<()> {
        writeln!(w, "pub struct {} {{", type_name)?;
        for (k, t) in self.0.iter() {
            writeln!(w, "    \"{}\": {},", k, t)?;
        }
        writeln!(w, "}}")?;

        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Enum(BTreeMap<Variant, SchemaTypes>);

impl Enum {
    fn new() -> Enum {
        Enum(BTreeMap::new())
    }

    fn add(&mut self, variant: Variant, ty: SchemaTypes) {
        self.0.insert(variant, ty);
    }

    pub fn print<W: Write>(&self, mut w: W, type_name: &str) -> io::Result<()> {
        writeln!(w, "pub enum {} {{", type_name)?;
        for (k, t) in self.0.iter() {
            writeln!(w, "    {}({}),", k, t)?;
        }
        writeln!(w, "}}")?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Variant {
    Primitive(String),
    Struct(u64),
    Enum(u64),
}

impl Variant {
    fn primitive(s: &str) -> Variant {
        Variant::Primitive(s.to_owned())
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Variant::Primitive(s) => write!(f, "{}", s),
            Variant::Struct(id) => write!(f, "Struct{}", id),
            Variant::Enum(id) => write!(f, "Enum{}", id),
        }
    }
}
