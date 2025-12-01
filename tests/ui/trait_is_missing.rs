use dbstruct::dbstruct;

struct CustomType {
    field: u8,
}

#[dbstruct(db=btreemap)]
struct Test {
    #[dbstruct(Default)]
    field: CustomType,
}

fn main() {}
