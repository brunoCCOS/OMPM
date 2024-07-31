use std::fs;
use std::path::Path;
pub mod license;
use license::License;

pub fn create_readme(path: &Path, name: &str) {
    // Create a README file
    let readme_path = path.join("README.md");
    fs::write(&readme_path,
        format!("
            # {}\n
            This is a readme file for {}.
        ", name, name)
    ).expect("Failed to create README.md");
}

pub fn create_gitignore(path: &Path, folder: &str) {
    // Create a .gitignore file
    let gitignore_path = path.join(".gitignore");
    fs::write(&gitignore_path, format!("{}/\n", folder)).expect("Failed to create .gitignore");
}

pub fn create_contribute(path: &Path, name: &str) {
    // Create a CONTRIBUTING file
    let contribute_path = path.join("CONTRIBUTING.md");
    fs::write(&contribute_path,
        format!(
            "
            # Contributing to {0}

            Thank you for considering contributing to {0}! Here are some guidelines to help you get started.

            ## How to Contribute

            1. Fork the repository.
            2. Clone your forked repository:
                ```sh
                git clone https://github.com/your-username/{0}.git
                ```
            3. Create a new branch for your feature or bugfix:
                ```sh
                git checkout -b feature-name
                ```
            4. Make your changes.
            5. Commit your changes with a descriptive commit message:
                ```sh
                git commit -m \"Description of the feature or fix\"
                ```
            6. Push your branch to your forked repository:
                ```sh
                git push origin feature-name
                ```
            7. Open a Pull Request (PR) on the main repository.

            ## Reporting Issues

            If you find any bugs or have feature requests, please [open an issue](https://github.com/your-username/{0}/issues).

            ## Code of Conduct

            Please adhere to our [Code of Conduct](CODE_OF_CONDUCT.md).
            ", name)
    ).expect("Failed to create CONTRIBUTING.md");
}

pub fn create_code_of_conduct(path: &Path) {
    // Create a CODE_OF_CONDUCT file
    let code_of_conduct_path = path.join("CODE_OF_CONDUCT.md");
    fs::write(&code_of_conduct_path,
        "
        # Code of Conduct

        ## Our Pledge

        We as members, contributors, and leaders pledge to make participation in our community a harassment-free experience for everyone, regardless of age, body size, visible or invisible disability, ethnicity, sex characteristics, gender identity and expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

        We pledge to act and interact in ways that contribute to an open, welcoming, diverse, inclusive, and healthy community.

        ## Our Standards

        Examples of behavior that contributes to a positive environment for our community include:

        - Demonstrating empathy and kindness toward other people
        - Being respectful of differing opinions, viewpoints, and experiences
        - Giving and gracefully accepting constructive feedback
        - Accepting responsibility and apologizing to those affected by our mistakes, and learning from the experience
        - Focusing on what is best not just for us as individuals, but for the overall community

        Examples of unacceptable behavior include:

        - The use of sexualized language or imagery, and sexual attention or advances of any kind
        - Trolling, insulting or derogatory comments, and personal or political attacks
        - Public or private harassment
        - Publishing others' private information, such as a physical or email address, without their explicit permission
        - Other conduct which could reasonably be considered inappropriate in a professional setting

        ## Enforcement Responsibilities

        Community leaders are responsible for clarifying and enforcing our standards of acceptable behavior and will take appropriate and fair corrective action in response to any behavior that they deem inappropriate, threatening, offensive, or harmful.

        Community leaders have the right and responsibility to remove, edit, or reject comments, commits, code, wiki edits, issues, and other contributions that are not aligned with this Code of Conduct, and will communicate reasons for moderation decisions when appropriate.

        ## Scope

        This Code of Conduct applies within all community spaces, and also applies when an individual is officially representing the community in public spaces. Examples of representing our community include using an official e-mail address, posting via an official social media account, or acting as an appointed representative at an online or offline event.

        ## Enforcement

        Instances of abusive, harassing, or otherwise unacceptable behavior may be reported to the community leaders responsible for enforcement at [INSERT CONTACT METHOD]. All complaints will be reviewed and investigated promptly and fairly.

        All community leaders are obligated to respect the privacy and security of the reporter of any incident.

        ## Enforcement Guidelines

        Community leaders will follow these Community Impact Guidelines in determining the consequences for any action they deem in violation of this Code of Conduct:

        ### 1. Correction

        **Community Impact**: Use of inappropriate language or other behavior deemed unprofessional or unwelcome in the community.

        **Consequence**: A private, written warning from community leaders, providing clarity around the nature of the violation and an explanation of why the behavior was inappropriate. A public apology may be requested.

        ### 2. Warning

        **Community Impact**: A violation through a single incident or series of actions.

        **Consequence**: A warning with consequences for continued behavior. No interaction with the people involved, including unsolicited interaction with those enforcing the Code of Conduct, for a specified period of time. This includes avoiding interactions in community spaces as well as external channels like social media. Violating these terms may lead to a temporary or permanent ban.

        ### 3. Temporary Ban

        **Community Impact**: A serious violation of community standards, including sustained inappropriate behavior.

        **Consequence**: A temporary ban from any sort of interaction or public communication with the community for a specified period of time. No public or private interaction with the people involved, including unsolicited interaction with those enforcing the Code of Conduct, is allowed during this period. Violating these terms may lead to a permanent ban.

        ### 4. Permanent Ban

        **Community Impact**: Demonstrating a pattern of violation of community standards, including sustained inappropriate behavior, harassment of an individual, or aggression toward or disparagement of classes of individuals.

        **Consequence**: A permanent ban from any sort of public interaction within the community.

        ## Attribution

        This Code of Conduct is adapted from the [Contributor Covenant](https://www.contributor-covenant.org), version 2.0, available at [https://www.contributor-covenant.org/version/2/0/code_of_conduct.html].

        Community Impact Guidelines were inspired by [Mozilla's code of conduct enforcement ladder](https://github.com/mozilla/diversity).

        For answers to common questions about this code of conduct, see the FAQ at [https://www.contributor-covenant.org/faq]. Translations are available at [https://www.contributor-covenant.org/translations].
        "
    ).expect("Failed to create CODE_OF_CONDUCT.md");
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
