#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]

pub struct Genomeiter {
    pub header: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct CollectIter {
    pub name: String,
    pub namenext: String,
    pub id: String,
    pub idnext: String,
    pub count: usize,
    pub shared: usize,
}
