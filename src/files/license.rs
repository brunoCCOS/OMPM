use std::path::Path;
use std::fs;
use clap::ValueEnum;

#[derive(Debug,Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum License {
    /// License type
    MIT,
    Apache2_0,
    GnuGplV3,
    Bsd3Clause,
    CC0_1_0,
}

impl ToString for License {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub fn create_license(path: &Path, license_type: License) {
    let license_text = match license_type {
        License::MIT => "MIT License\n\nPermission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.",
        License::Apache2_0 => "Apache License\nVersion 2.0, January 2004\nhttp://www.apache.org/licenses/\n\nTERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION\n\n1. Definitions.\n\n...\n\nEND OF TERMS AND CONDITIONS\n\nAPPENDIX: How to apply the Apache License to your work.",
        License::GnuGplV3 => "GNU GENERAL PUBLIC LICENSE\nVersion 3, 29 June 2007\n\n...\n\nEND OF TERMS AND CONDITIONS",
        License::Bsd3Clause => "BSD 3-Clause License\n\nRedistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:\n\n...\n\nTHIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.",
        License::CC0_1_0 => "This is free and unencumbered software released into the public domain.\n\n...\n\nFor more information, please refer to <http://unlicense.org/>",
    };

    let license_path = path.join("LICENSE.md");
    fs::write(&license_path, license_text).expect("Failed to create LICENSE.md");
}
