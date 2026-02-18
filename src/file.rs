#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Fasta {
    pub fastafile: String,
    pub kmer: Option<String>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Fastakmer {
    pub fastafile: String,
    pub kmer: String,
    pub step: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Neighbors {
    pub neighboursfile: String,
    pub valuefile: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MachineMinimizers {
    pub pathfile: String,
    pub value_k: String,
    pub value_w: String,
    pub can: bool,
}
