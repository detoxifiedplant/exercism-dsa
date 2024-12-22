#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.find(|c| !"GCTA".contains(c)) {
            Some(i) => Err(i),
            None => Ok(Dna {
                sequence: dna.into(),
            }),
        }
    }

    pub fn into_rna(self) -> Rna {
        let sequence: String = self
            .sequence
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect();
        Rna { sequence }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let _ = match rna.find(|c| !"AUGCU".contains(c)) {
            Some(e) => Err(e),
            None => Ok(Rna {
                sequence: rna.into(),
            }),
        };

        match rna.chars().position(|c| !"GCUA".contains(c)) {
            Some(i) => Err(i),
            None => Ok(Rna {
                sequence: rna.into(),
            }),
        }
    }
}
