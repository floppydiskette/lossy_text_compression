use crate::{CompressionTypes, Compressor};
use crate::common::*;

impl Compressor {
    pub fn new(compression_types: CompressionTypes) -> Compressor {
        Compressor {
            compression_types,
        }
    }

    pub fn compress(&self, input: impl AsRef<str>) -> String {
        let mut output = String::new();

        if self.compression_types.thesaurus {
            output = thesaurus::compress(input);
        }

        if self.compression_types.vowel_removal {
            output = vowels::compress(output, false);
        }

        if self.compression_types.intense_vowel_removal {
            output = vowels::compress(output, true);
        }


        if self.compression_types.punctuation_removal {
            output = punctuation::compress(output);
        }

        // note: whitespace removal must be last
        if self.compression_types.whitespace_removal {
            output = whitespace::compress(output);
        }

        output
    }
}