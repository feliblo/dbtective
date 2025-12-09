# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---

## [0.1.15](https://github.com/feliblo/dbtective/compare/v0.1.14..v0.1.15) - 2025-12-09

### Bug Fixes

- **(docs)** fix documentation references to other pages - ([938c6ee](https://github.com/feliblo/dbtective/commit/938c6ee93a90425b9ddbb7f807f0c8d8d494a97a)) - Felix Blom

### Features

- **(config)** accept v20 manifest.json (it's identical to v12) - ([17fff29](https://github.com/feliblo/dbtective/commit/17fff29b41fb18a9d2bc1a62e02efb3cefa75e92)) - Felix Blom

### Bump

- version 0.1.14 → 0.1.15 - ([9406102](https://github.com/feliblo/dbtective/commit/94061022d5da9b1acacc4af6cebe46234917a423)) - Felix Blom

---

## [0.1.14](https://github.com/feliblo/dbtective/compare/v0.1.13..v0.1.14) - 2025-12-09

### Features

- **(checks)** add columns_have_descriptions check - ([f6e2542](https://github.com/feliblo/dbtective/commit/f6e254223f8390f384eb55046c896aa8749cdd80)) - Felix Blom
- **(config)** add dbtective.toml and pyproject.toml support - ([5a55aa4](https://github.com/feliblo/dbtective/commit/5a55aa493b3df5158b9fe2aaaaa3b21edd125fec)) - Felix Blom

### Bump

- version 0.1.13 → 0.1.14 - ([9bee196](https://github.com/feliblo/dbtective/commit/9bee196606c9b552658b39d27cf223c43305a65c)) - Felix Blom

### Ci

- **(compile)** check if dbtective compiles & runs on test project in every pr - ([2f2e431](https://github.com/feliblo/dbtective/commit/2f2e4316e9a6de957ac02fb8e562c1d0c48a216a)) - Felix Blom

---

## [0.1.13](https://github.com/feliblo/dbtective/compare/v0.1.12..v0.1.13) - 2025-12-07

### Documentation

- **(checks)** add documentation for has_unique_test - ([a1f1c30](https://github.com/feliblo/dbtective/commit/a1f1c30b9b002521a864544ef43a9519824d88dd)) - Felix Blom

### Features

- **(checks)** add 'is_not_orphaned' check & refactor tests into multiple files - ([68e0360](https://github.com/feliblo/dbtective/commit/68e0360223f0fe4c0df61e0d99fe556c89680e90)) - Felix Blom
- **(checks)** create has_unique_test - ([08c1065](https://github.com/feliblo/dbtective/commit/08c1065b335b156ba9acd9c5ae6d9e079e42a695)) - Felix Blom

### Bump

- version 0.1.12 → 0.1.13 - ([08b7c3f](https://github.com/feliblo/dbtective/commit/08b7c3fdcc1335f5cd04915a66a896e26ff1878d)) - Felix Blom

---

## [0.1.12](https://github.com/feliblo/dbtective/compare/v0.1.5..v0.1.12) - 2025-12-06

### Bug Fixes

- **(checks)** fix applies_to for columns_are_documented and clippy warnings - ([9d237ec](https://github.com/feliblo/dbtective/commit/9d237ecfb35e305df374ef905baaa1119c0b530d)) - Felix Blom

### Documentation

- **(checks)** improve has_description example - ([587e7e6](https://github.com/feliblo/dbtective/commit/587e7e6c7c0684c9ecd904cf1afb4d258c05a19d)) - feliblo
- **(checks)** change the applies_to section in all current checks - ([5310762](https://github.com/feliblo/dbtective/commit/5310762b7ab632f7fdc6c3d27fb5272d715c2f2f)) - Felix Blom
- **(checks)** columns_all_documented docs added and generally docs improved - ([943ec9c](https://github.com/feliblo/dbtective/commit/943ec9cd26a1f162a2dbd86385988cae43186edb)) - Felix Blom
- **(name)** remove old name from dbt_project - ([09f32b5](https://github.com/feliblo/dbtective/commit/09f32b54191f2b10f013da130ffc847fac14763d)) - Felix Blom
- **(readme)** add prek/pre-commit to installs and auto-update version - ([b625887](https://github.com/feliblo/dbtective/commit/b6258875ba95ef8da8395b553cef7a205e593bc6)) - Felix Blom

### Features

- **(actions)** add github actions runner - ([364b435](https://github.com/feliblo/dbtective/commit/364b43506f173d6958c2ed7bab59daf397658caf)) - Felix Blom
- **(applies_to)** add more possible targets to applies_to - ([cae2641](https://github.com/feliblo/dbtective/commit/cae2641e92a8c24e209192f8e2f1a763f4fa971e)) - Felix Blom
- **(catalog)** add catalog parser - ([890a550](https://github.com/feliblo/dbtective/commit/890a550c1311ef997ad3af5af81b3e7a083c39ec)) - feliblo
- **(cfg)** add catalog tests to config - ([e0bc29c](https://github.com/feliblo/dbtective/commit/e0bc29cb6b0f9f725c07ff0db649188f276fc967)) - Felix Blom
- **(checks)** run checks on all (data-related) manifest objects - ([67c5ebe](https://github.com/feliblo/dbtective/commit/67c5ebe9e6165313f409eb5502317f2bc2499d0b)) - Felix Blom
- **(checks)** add all_columns_are_documented for nodes - ([d228c92](https://github.com/feliblo/dbtective/commit/d228c92c689f535cd701950f6288d5bea135f394)) - Felix Blom
- **(cli)** implement catalog into cli - ([d09c0ae](https://github.com/feliblo/dbtective/commit/d09c0ae2243d82a764c683bae84ed279f2d71702)) - feliblo
- **(table)** fit table to terminal & add --disable-hyperlinks option - ([b23ad4b](https://github.com/feliblo/dbtective/commit/b23ad4b2391cac2f8d146b7fbbb5a6f39381b838)) - Felix Blom

### Refactoring

- **(catalog)** preparation for catalog based tests - ([c6c27a5](https://github.com/feliblo/dbtective/commit/c6c27a5fcd7f7017f30af04f53f0cb5cd3b559a0)) - Felix Blom
- **(checks)** change filename to other_manifest_object_checks.rs - ([cbd6081](https://github.com/feliblo/dbtective/commit/cbd608145b050aec5c90e9999af31bc0d6f8640f)) - Felix Blom
- **(manifest)** change dbt_objects module into manifests - ([bf71136](https://github.com/feliblo/dbtective/commit/bf71136c006954e21ff27f09bc6d6836b9f67ac1)) - feliblo
- **(rules)** change ruletypes and applies_to setup for manifest & catalog checks - ([d0c0a47](https://github.com/feliblo/dbtective/commit/d0c0a478bcbddda7a566e023239971468131ddbe)) - Felix Blom
- **(run)** refactor run to use unwrap_or_exit helper - ([1bed05a](https://github.com/feliblo/dbtective/commit/1bed05a986aef295d551f1e019d6ecdb915439fa)) - Felix Blom

### Style

- **(run)** refactor execution of checks using flat_map instead of loops - ([f5b6401](https://github.com/feliblo/dbtective/commit/f5b64011b30207ca0c46197357ff3a2307acfe53)) - Felix Blom

### Tests

- **(cfg)** add config parsing tests - ([5b2b13f](https://github.com/feliblo/dbtective/commit/5b2b13f197f754ffb50f0888991c97c1394c767e)) - Felix Blom
- **(checks)** add tests for all_columns_documented - ([0b8f2d6](https://github.com/feliblo/dbtective/commit/0b8f2d66716abad5c1b5dd820ab97870ec495b43)) - Felix Blom

### Bump

- version 0.1.5 → 0.1.6 - ([c8478cd](https://github.com/feliblo/dbtective/commit/c8478cd67387a80727ee57554287063fed982a63)) - feliblo
- version 0.1.6 → 0.1.7 - ([89e4a0a](https://github.com/feliblo/dbtective/commit/89e4a0abf45eaab1efc05a54220b63104809f6b5)) - feliblo
- version 0.1.7 → 0.1.8 - ([5416f9e](https://github.com/feliblo/dbtective/commit/5416f9ecb119c0234414cca911ecbb9d03fc20e9)) - feliblo
- version 0.1.8 → 0.1.9 - ([d1eb17e](https://github.com/feliblo/dbtective/commit/d1eb17e2e36891999055c23950d0081a98c531a3)) - Felix Blom
- version 0.1.9 → 0.1.10 - ([10906d3](https://github.com/feliblo/dbtective/commit/10906d33c12179446f0d9717a685ec478fef7989)) - Felix Blom
- version 0.1.10 → 0.1.11 - ([7945f9f](https://github.com/feliblo/dbtective/commit/7945f9fef7c680a52bec7aba91f0c48c9f683e32)) - Felix Blom
- version 0.1.11 → 0.1.12 - ([236318f](https://github.com/feliblo/dbtective/commit/236318f5566764fff2c9da6c189c299960d7a96c)) - Felix Blom

### Ci

- **(prek)** add pre-commit/prek hook - ([5f13fd3](https://github.com/feliblo/dbtective/commit/5f13fd3b99f58035f1ad1178d7c1283a1064bdeb)) - Felix Blom

---

## [0.1.5](https://github.com/feliblo/dbtective/compare/v0.1.0-alpha..v0.1.5) - 2025-11-30

### Bug Fixes

- **(ci)** enable homebrew publishing - ([cff763b](https://github.com/feliblo/dbtective/commit/cff763bbd76c90a36219d3d2d72ec7fe6bc829cc)) - feliblo
- **(cli)** show warnings in output table - ([fa1c7fa](https://github.com/feliblo/dbtective/commit/fa1c7fabb571ff781a76cc14a1d13a66a195eb42)) - Felix Blom

### Documentation

- **(checks)** naming convention documentation - ([575fe21](https://github.com/feliblo/dbtective/commit/575fe213ae49010c89feb23eb6344a3ae50a71b2)) - feliblo
- **(config)** update documentaton for includes/excludes - ([2610fd3](https://github.com/feliblo/dbtective/commit/2610fd357bc6f74ec4a107b524d64b8cf67ae1a3)) - Felix Blom
- **(name_convention)** change description of name convention - ([13d6272](https://github.com/feliblo/dbtective/commit/13d6272af9df18a39f6e04a55379a4f4b3907e7e)) - Felix Blom

### Features

- **(applies_to)** Add apply_source_tests using applies_to - ([4eac04c](https://github.com/feliblo/dbtective/commit/4eac04cc258c77dde9bc97dea06f8e0d2a95a71c)) - Felix Blom
- **(checks)** add naming convention check - ([48ff150](https://github.com/feliblo/dbtective/commit/48ff1508419e4d3b96ffeea0812c0bd12b60fa75)) - feliblo
- **(cli)** make table clickable to go to files - ([419b318](https://github.com/feliblo/dbtective/commit/419b318f9c95c5c21b8ed6904c45359e937788b4)) - feliblo
- **(config)** intialize config rule hints - ([ba165f4](https://github.com/feliblo/dbtective/commit/ba165f48a2184011311fe72c70510c95284d8b9b)) - Felix Blom
- **(config)** handle valid applies to - ([bc619a7](https://github.com/feliblo/dbtective/commit/bc619a79f502d335d562e8b08574b81dd38f6295)) - Felix Blom
- **(config)** implement includes/excludes arguments - ([45d5dd2](https://github.com/feliblo/dbtective/commit/45d5dd23eecb4159b5a4b985ec15aee1f5408de6)) - Felix Blom
- **(pypi)** add pypi release pipeline ([#32](https://github.com/feliblo/dbtective/pull/32)) - ([e4a96be](https://github.com/feliblo/dbtective/commit/e4a96befebfabda0cdcc3ace749832129df12c6d)) - Felix Blom
- **(rules)** add includes/excludes for rule paths - ([71a76b7](https://github.com/feliblo/dbtective/commit/71a76b7fd1c17a1fead8d16b8ad87c2c28e22c1f)) - Felix Blom
- **(table)** make table messages clickable - ([4db6d0c](https://github.com/feliblo/dbtective/commit/4db6d0c3e007845c611d310082719432dfd27283)) - feliblo

### Miscellaneous Chores

- bump version to 0.2.0 - ([d59489d](https://github.com/feliblo/dbtective/commit/d59489df67bb298286b4b8fa2893449869cfcf78)) - Felix Blom

### Refactoring

- **(AppliesTo)** change appliesto to work on all manifest objects - ([8aa2e67](https://github.com/feliblo/dbtective/commit/8aa2e67c4c7231a640aa4d86ca52e953a7a2ab5f)) - Felix Blom
- **(anyhow)** propagate errors & impove/introduce (integration) testing - ([ef9289a](https://github.com/feliblo/dbtective/commit/ef9289a40b500286e887a96d8cda539e012a336d)) - Felix Blom
- **(config)** config module refactor into components - ([680bdd7](https://github.com/feliblo/dbtective/commit/680bdd706407c7d307b41a6dec9249d9869a23a9)) - Felix Blom
- **(release)** use cargo-dist for release pipeline - ([ec2a269](https://github.com/feliblo/dbtective/commit/ec2a269e3ac4bfd113595f70d2425bace7bf4539)) - Felix Blom

### Style

- **(just)** Add release command in just - ([a893bf8](https://github.com/feliblo/dbtective/commit/a893bf8fd24cfe6a117aa6da7ae1212450a47b10)) - Felix Blom
- **(just)** fix just command - ([97762d2](https://github.com/feliblo/dbtective/commit/97762d285af04955f9adc8180bd69c95b97eaa3a)) - Felix Blom
- **(prek)** install prek and run initial lints - ([7ccef4c](https://github.com/feliblo/dbtective/commit/7ccef4cd41e047f4ba5564b79a5c65eb39ce1ce2)) - Felix Blom
- **(print)** remove info logs - ([fbaa6f2](https://github.com/feliblo/dbtective/commit/fbaa6f2181be6ed29c91f210669ecc85e9d67a1b)) - feliblo
- **(refactor)** refactor to a run function - ([9ef696e](https://github.com/feliblo/dbtective/commit/9ef696e3f11dd1ba9fb854e1697c173467a9a8a0)) - Felix Blom
- **(table)** add name of the check to the table - ([0bdd6f4](https://github.com/feliblo/dbtective/commit/0bdd6f49a20035a78fa45870ed436a1a03c288dd)) - Felix Blom

### Build

- **(homebrew)** add homebrew download option - ([1a85fc8](https://github.com/feliblo/dbtective/commit/1a85fc884808d45e599f04be3b5eafaaecf8aea9)) - feliblo
- **(just)** add install command - ([33225b4](https://github.com/feliblo/dbtective/commit/33225b4f17923cbc7372b9eeb2f2813b4c3ac681)) - Felix Blom

### Bump

- version 0.1.0 → 0.1.1 - ([717d886](https://github.com/feliblo/dbtective/commit/717d886f4d5c275d3b0ea187a915e815ae3b8f2c)) - feliblo
- retry version 0.1.0 → 0.1.1 - ([b734e14](https://github.com/feliblo/dbtective/commit/b734e14507a235e1b3497ff11bda3a36b49dba73)) - feliblo
- version 0.1.1 → 0.1.2 - ([8ecec19](https://github.com/feliblo/dbtective/commit/8ecec19e7ba2d4eccf75c5a233d11d64e6c3f5a6)) - feliblo
- version 0.1.2 → 0.1.3 - ([614a139](https://github.com/feliblo/dbtective/commit/614a1394b35edac6963425a0e211b7d108d38d7c)) - feliblo
- version 0.1.3 → 0.1.4 - ([54ab758](https://github.com/feliblo/dbtective/commit/54ab7583b06b46587c21ed258d550d0ad293bab3)) - feliblo
- version 0.1.4 → 0.1.5 - ([aca87b2](https://github.com/feliblo/dbtective/commit/aca87b268e34c1ec00bc4b177ce16d0a4b33aa3f)) - feliblo

---

## [0.1.0-alpha] - 2025-11-25

### Bug Fixes

- fix output styling - ([8a9c1d4](https://github.com/feliblo/dbtective/commit/8a9c1d428eb1757cba5dd5bb49f1fbe21e313121)) - Felix Blom

### Documentation

- **(pages)** update pages and update readme with documentation links - ([f382115](https://github.com/feliblo/dbtective/commit/f38211523e1c4aaaad67736c61173e87c498f7d6)) - Felix Blom
- **(user)** remove references to old username - ([f99f3ee](https://github.com/feliblo/dbtective/commit/f99f3ee19eb3a39e9a79b3c4d67dedac15aa1425)) - Felix Blom
- **(web)** intialize web documentation - ([dabca7d](https://github.com/feliblo/dbtective/commit/dabca7dfa9cfa4ceb5b813e337d5d760f9872396)) - Felix Blom
- change default entrypoint & justfile - ([4dcadcf](https://github.com/feliblo/dbtective/commit/4dcadcfdc32d2d3cbad3fb6ebb1e2d005ddff849)) - Felix Blom
- github setup & inital documentation website - ([2e7c41e](https://github.com/feliblo/dbtective/commit/2e7c41e08a3704e74e7054baef3c4ea0e986a984)) - Felix Blom
- try out github pages hugo - ([93dd5aa](https://github.com/feliblo/dbtective/commit/93dd5aac91976bd593d93fe72bda786a53482002)) - Felix Blom

### Refactoring

- **(node)** remove copying around data in descriptable naming - ([935b81f](https://github.com/feliblo/dbtective/commit/935b81fb0d04c3b6059efef86443b24ffcfd85a6)) - Felix Blom
- refactor rule messaging (free memory) - ([a74a7bd](https://github.com/feliblo/dbtective/commit/a74a7bdb08801083f3612e21642087507332f4c7)) - Felix Blom

### Style

- **(shear)** Remove unused packages - ([345232e](https://github.com/feliblo/dbtective/commit/345232ea3ee6c0d95ac3d6978333587f5ec08aa9)) - Felix Blom
- **(shear)** remove console (unused) - ([f7b305a](https://github.com/feliblo/dbtective/commit/f7b305a9b32565abe7573c4a8257c41782ecc855)) - Felix Blom
- fix clippy lints - ([e797751](https://github.com/feliblo/dbtective/commit/e797751815216d577f178829f5d6d4d9f5427be9)) - Felix Blom

### Tests

- **(config)** fix config unit tests - ([bc74bb9](https://github.com/feliblo/dbtective/commit/bc74bb9e60cb255911de9241fee8e6ca7d029c67)) - Felix Blom
- **(has_description)** add test for valid description - ([cd632b2](https://github.com/feliblo/dbtective/commit/cd632b2979888182bd767bd306f7a496d624eca0)) - Felix Blom

### Build

- **(release)** Initial release pipeline - ([dedcf38](https://github.com/feliblo/dbtective/commit/dedcf38674e0f8b9bffdbc5f14a8e0e338aecd30)) - Felix Blom

<!-- generated by git-cliff -->
