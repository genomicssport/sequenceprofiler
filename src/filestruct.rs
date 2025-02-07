#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]

pub struct Genomeiter {
    pub header: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct Collector {
    pub name: String,
    pub id: String,
    pub count: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct ProfileKmer {
    pub name: String,
    pub sequence: Vec<String>,
    pub count: usize,
    pub shared: usize,
    pub ratio: usize,
}
