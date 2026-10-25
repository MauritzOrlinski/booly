use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use crate::cnf::clause::Clause;
use crate::cnf::literals::to_lit;

pub type Literal = i32;

pub struct ProofClause {
    pub lits: Vec<Literal>,
}

impl From<&Clause> for ProofClause {
    fn from(value: &Clause) -> Self {
        ProofClause {
            lits: value.literals.iter().map(|x| to_lit(&x)).collect(),
        }
    }
}
impl From<&mut Clause> for ProofClause {
    fn from(value: &mut Clause) -> Self {
        ProofClause {
            lits: value.literals.iter().map(|x| to_lit(&x)).collect(),
        }
    }
}

impl ProofClause {
    pub fn new(lits: Vec<Literal>) -> ProofClause {
        ProofClause { lits: lits }
    }
}

pub enum ProofStep {
    Comment(String),
    Clause(ProofClause),
    Delete(ProofClause),
}

pub struct ProofLogger<W>
where
    W: Write,
{
    log: BufWriter<W>,
}

impl<W: Write> std::fmt::Debug for ProofLogger<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProofLogger").finish_non_exhaustive()
    }
}

impl<W> ProofLogger<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        ProofLogger {
            log: BufWriter::new(writer),
        }
    }

    pub fn log_clause(&mut self, clause: ProofClause) -> io::Result<()> {
        for lit in clause.lits {
            write!(self.log, "{lit} ")?;
        }
        self.log.write_all(b"0\n")
    }

    pub fn log_delete(&mut self, clause: ProofClause) -> io::Result<()> {
        write!(self.log, "d ")?;
        for lit in clause.lits {
            write!(self.log, "{lit} ")?;
        }
        self.log.write_all(b"0\n")
    }

    pub fn log_comment(&mut self, comment: String) -> io::Result<()> {
        write!(self.log, "c {comment}")?;
        self.log.write_all(b"\n")
    }

    pub fn flush_to_file(&mut self) -> io::Result<()> {
        self.log.flush()
    }
}

impl ProofLogger<File> {
    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self::new(File::create(path)?))
    }
}

impl ProofLogger<Box<dyn Write>> {
    /// Like [`ProofLogger::create`], but with the writer type the solver expects.
    pub fn create_boxed(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self::new(Box::new(File::create(path)?)))
    }
}
