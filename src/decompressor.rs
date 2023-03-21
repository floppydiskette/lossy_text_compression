use crate::{DecompressionTypes, Decompressor};
use crate::common::*;

impl Decompressor {
    pub fn new(decompression_types: DecompressionTypes) -> Decompressor {
        Decompressor {
            decompression_types,
        }
    }

    pub fn decompress(&self, input: impl AsRef<str>) -> String {
        let mut output = String::new();

        if self.decompression_types.whitespace_injection {
            output = whitespace::decompress(input);
        }

        if self.decompression_types.punctuation_injection {
            output = punctuation::decompress(output);
        }

        if self.decompression_types.vowel_injection {
            output = vowels::decompress(output);
        }

        if self.decompression_types.thesaurus {
            output = thesaurus::decompress(output);
        }


        output
    }
}