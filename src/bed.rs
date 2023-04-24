use crate::{
    interval::{find_in_lapper, get_lapper_hashmap, Region},
    parser::Strand,
    utils::{get_file_reader, get_output_writer},
};
use csv::{DeserializeRecordsIter, ReaderBuilder};
use log::{info, warn};
use std::{
    fmt,
    io::{self, Write},
};

#[derive(Debug)]
pub struct BedReader<R: io::Read> {
    inner: csv::Reader<R>,
}

impl<R: io::Read> BedReader<R> {
    /// Read from a given reader.
    pub fn new(reader: R) -> Self {
        BedReader {
            inner: ReaderBuilder::new()
                .delimiter(b'\t')
                .has_headers(false)
                .comment(Some(b'#'))
                .from_reader(reader),
        }
    }

    /// Iterate over all records.
    pub fn bedrecords(&mut self) -> BedRecords<'_, R> {
        BedRecords {
            inner: self.inner.deserialize(),
        }
    }
}

/// An iterator over the records of a BED file.
pub struct BedRecords<'a, R: io::Read> {
    inner: DeserializeRecordsIter<'a, R, BedRecord>,
}

impl<'a, R: io::Read> Iterator for BedRecords<'a, R> {
    type Item = csv::Result<BedRecord>;

    fn next(&mut self) -> Option<csv::Result<BedRecord>> {
        self.inner.next()
    }
}

#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct BedRecord {
    pub chrom: String,
    pub start: usize,
    pub end: usize,
    #[serde(default)]
    aux: Vec<String>,
}

impl BedRecord {
    /// Get the strand flexible.
    pub fn strand(&self) -> Strand {
        match self.aux(5) {
            Some("+") => Strand::Positive,
            Some("-") => Strand::Negative,
            _ => {
                warn!(
                    "BED record: {{{}}} column 6 should be +/-, but it is not! Force to be positive",
                    &self
                );
                Strand::Positive
            }
        }
    }

    /// Access auxiliary fields after the strand field by index
    /// (counting first field (chromosome) as 0).
    pub fn aux(&self, i: usize) -> Option<&str> {
        let j = i - 3;
        if j < self.aux.len() {
            Some(&self.aux[j])
        } else {
            None
        }
    }
    pub fn into_region(&self) -> Region {
        Region {
            chrom: &self.chrom,
            start: self.start,
            end: self.end,
            strand: self.strand(),
        }
    }
}

impl fmt::Display for BedRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aux_string = self
            .aux
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\t");
        write!(
            f,
            "{}\t{}\t{}\t{}",
            self.chrom, self.start, self.end, aux_string
        )
    }
}

pub fn cross_bed(
    bed_file: &String,
    input_chain: &Option<String>,
    output_bed: &Option<String>,
    unmaped_bed: &Option<String>,
    rewrite: bool,
) {
    let lapper_hashmap = get_lapper_hashmap(input_chain);
    info!("get lapper hashmap done!");
    let bed_file = get_file_reader(bed_file).unwrap();
    let (mut output_file, stdout_mode) = get_output_writer(output_bed, rewrite);
    let mut unmaped_file = match stdout_mode {
        true => Box::new(io::stdout()),
        false => get_output_writer(unmaped_bed, rewrite).0,
    };

    let mut bed_reder = BedReader::new(bed_file);
    for bed_line in bed_reder.bedrecords() {
        let bed_rcd = match bed_line {
            Ok(bed_rcd) => {
                // info!("capture a bed record!");
                bed_rcd
            }
            Err(e) => {
                warn!("SKIP: Error parsing BED record: {}", e);
                continue;
            }
        };
        let bed_region = bed_rcd.into_region();
        if bed_region.start > bed_region.end {
            warn!("SKIP: Start larger end in BED record: {}", bed_rcd);
            continue;
        }
        let matches = find_in_lapper(&lapper_hashmap, &bed_region);
        match matches {
            None => {
                unmaped_file
                    .write_all(format!("{}\tUNMAP\n", bed_rcd).as_bytes())
                    .unwrap();
                continue;
            }
            Some(matches) => {
                let match_len = matches.len();
                let mut count = 0;
                for j in (1..match_len).step_by(2) {
                    count += 1;
                    let current_match_region = &matches[j];
                    let hit_multi = match_len > 2;
                    let hit_info = match hit_multi {
                        true => format!("(split.{}:{})", count, &matches[j - 1]).replace('\t', "@"),
                        false => "->".to_string(),
                    };
                    if stdout_mode {
                        output_file
                            .write_all(
                                format!("{}\t{}\t{}\n", bed_rcd, hit_info, current_match_region)
                                    .as_bytes(),
                            )
                            .unwrap();
                    } else {
                        output_file
                            .write_all(format!("{}\n", current_match_region).as_bytes())
                            .unwrap();
                    }
                }
            }
        }
    }
}
