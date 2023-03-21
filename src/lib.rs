pub mod compressor;
pub mod decompressor;
pub(crate) mod common;

pub const COMPRESSOR_LEVEL_1: Compressor = Compressor {
    compression_types: CompressionTypes {
        thesaurus: true,
        punctuation_removal: false,
        vowel_removal: false,
        intense_vowel_removal: false,
        whitespace_removal: false,
    },
};

pub const COMPRESSOR_LEVEL_2: Compressor = Compressor {
    compression_types: CompressionTypes {
        thesaurus: true,
        punctuation_removal: true,
        vowel_removal: false,
        intense_vowel_removal: false,
        whitespace_removal: false,
    },
};

pub const COMPRESSOR_LEVEL_3: Compressor = Compressor {
    compression_types: CompressionTypes {
        thesaurus: true,
        punctuation_removal: true,
        vowel_removal: true,
        intense_vowel_removal: false,
        whitespace_removal: false,
    },
};

pub const COMPRESSOR_LEVEL_4: Compressor = Compressor {
    compression_types: CompressionTypes {
        thesaurus: true,
        punctuation_removal: true,
        vowel_removal: true,
        intense_vowel_removal: true,
        whitespace_removal: false,
    },
};

pub const COMPRESSOR_LEVEL_5: Compressor = Compressor {
    compression_types: CompressionTypes {
        thesaurus: true,
        punctuation_removal: true,
        vowel_removal: true,
        intense_vowel_removal: true,
        whitespace_removal: true,
    },
};

pub const DECOMPRESSOR_LEVEL_1: Decompressor = Decompressor {
    decompression_types: DecompressionTypes {
        vowel_injection: false,
        punctuation_injection: false,
        thesaurus: true,
        whitespace_injection: false,
    },
};

pub const DECOMPRESSOR_LEVEL_2: Decompressor = Decompressor {
    decompression_types: DecompressionTypes {
        vowel_injection: false,
        punctuation_injection: true,
        thesaurus: true,
        whitespace_injection: false,
    },
};

pub const DECOMPRESSOR_LEVEL_3: Decompressor = Decompressor {
    decompression_types: DecompressionTypes {
        vowel_injection: true,
        punctuation_injection: true,
        thesaurus: true,
        whitespace_injection: false,
    },
};

pub const DECOMPRESSOR_LEVEL_4: Decompressor = Decompressor {
    decompression_types: DecompressionTypes {
        vowel_injection: true,
        punctuation_injection: true,
        thesaurus: true,
        whitespace_injection: true,
    },
};

#[derive(Clone, Debug)]
pub struct CompressionTypes {
    /// finds the shortest synonym for a word and replaces it
    pub thesaurus: bool,
    /// removes all punctuation
    pub punctuation_removal: bool,
    /// removes all vowels unless they are the first or last letter
    pub vowel_removal: bool,
    /// removes all vowels no matter what
    pub intense_vowel_removal: bool,
    /// removes all whitespace
    pub whitespace_removal: bool,
}

#[derive(Clone, Debug)]
pub struct Compressor {
    pub compression_types: CompressionTypes,
}

#[derive(Clone, Debug)]
pub struct DecompressionTypes {
    /// attempts to increase readability by adding vowels
    pub vowel_injection: bool,
    /// attempts to add punctuation back in
    pub punctuation_injection: bool,
    /// tries to find the longest synonym for a word and replace it
    pub thesaurus: bool,
    /// attempts to add whitespace back in
    pub whitespace_injection: bool,
}

#[derive(Clone, Debug)]
pub struct Decompressor {
    pub decompression_types: DecompressionTypes,
}