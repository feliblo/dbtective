## v0.1.24 (2025-12-11)

### Fix

- **logs**: remove verbose logs from catalog - ([64d912f](https://github.com/feliblo/dbtective/commit/64d912f0b15da57a6935c4c92735a579b7973976)) - feliblo
- **table**: try another windows prefix fix in hyperlinks - ([a667eaa](https://github.com/feliblo/dbtective/commit/a667eaaa50c6950a86357113e8d7ea67d08b637f)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.23 (2025-12-11)

### Feat

- **table**: order the table by severity, object_type and rule - ([2e0e72b](https://github.com/feliblo/dbtective/commit/2e0e72bd32cefc551beb7be3077eac4c61988688)) - feliblo

### Fix

- **logs**: remove even more verbose logging - ([fc3a0ee](https://github.com/feliblo/dbtective/commit/fc3a0ee2e069c698d8155c883ba84c1c95bc5843)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.22 (2025-12-11)

### Feat

- **rules**: add has_contract_enforced rule - ([da1536e](https://github.com/feliblo/dbtective/commit/da1536ef2a14964f9614804f2cc0d1b1e43cf1c4)) - Felix Blom

### Fix

- **table**: fix windows hyperlinks in table - ([b0ee40d](https://github.com/feliblo/dbtective/commit/b0ee40d1f6ee97e7d9f12b2b38633d2d980d0762)) - feliblo
- **logs**: remove overly verbose logging - ([51bdf8f](https://github.com/feliblo/dbtective/commit/51bdf8f38698fedef43822a15ca516977a524e4f)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo), [@feliblo](https://github.com/feliblo)

## v0.1.21 (2025-12-10)

### Fix

- **bug**: fix column constraint parsing - ([b9e4421](https://github.com/feliblo/dbtective/commit/b9e4421754980b535d0dc112555d1fdf2fa58026)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.20 (2025-12-09)

### Fix

- **release**: add --no-verify flag to changelog amend commit - ([05f1edb](https://github.com/feliblo/dbtective/commit/05f1edbbd693e7538dac012659a7b3226d48ab01)) - Felix Blom
- **docs**: cargo dist needs h1's as changelog headers - ([a3db336](https://github.com/feliblo/dbtective/commit/a3db336462ea40bdbc314b8ae3878df4413f8718)) - Felix Blom
- **docs**: fix changelog formatting - retry auto-release-description - ([54835b7](https://github.com/feliblo/dbtective/commit/54835b7d43ba04b9c009cfb3fb498a9a7f1f34c8)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.17 (2025-12-09)

### Feat

- **ci**: tryout git cliff for changelog generation - ([c9bc801](https://github.com/feliblo/dbtective/commit/c9bc8017a5e96a1a1291a2767cb90690382a1bde)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.15 (2025-12-09)

### Feat

- **config**: accept v20 manifest.json (it's identical to v12) - ([17fff29](https://github.com/feliblo/dbtective/commit/17fff29b41fb18a9d2bc1a62e02efb3cefa75e92)) - Felix Blom

### Fix

- **docs**: fix documentation references to other pages - ([938c6ee](https://github.com/feliblo/dbtective/commit/938c6ee93a90425b9ddbb7f807f0c8d8d494a97a)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.14 (2025-12-09)

### Feat

- **config**: add dbtective.toml and pyproject.toml support - ([5a55aa4](https://github.com/feliblo/dbtective/commit/5a55aa493b3df5158b9fe2aaaaa3b21edd125fec)) - Felix Blom
- **checks**: add columns_have_descriptions check - ([f6e2542](https://github.com/feliblo/dbtective/commit/f6e254223f8390f384eb55046c896aa8749cdd80)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.13 (2025-12-07)

### Feat

- **checks**: create has_unique_test - ([08c1065](https://github.com/feliblo/dbtective/commit/08c1065b335b156ba9acd9c5ae6d9e079e42a695)) - Felix Blom
- **checks**: add 'is_not_orphaned' check & refactor tests into multiple files - ([68e0360](https://github.com/feliblo/dbtective/commit/68e0360223f0fe4c0df61e0d99fe556c89680e90)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.12 (2025-12-06)

### Feat

- **table**: fit table to terminal & add --disable-hyperlinks option - ([b23ad4b](https://github.com/feliblo/dbtective/commit/b23ad4b2391cac2f8d146b7fbbb5a6f39381b838)) - Felix Blom
- **actions**: add github actions runner - ([364b435](https://github.com/feliblo/dbtective/commit/364b43506f173d6958c2ed7bab59daf397658caf)) - Felix Blom
- **checks**: add all_columns_are_documented for nodes - ([d228c92](https://github.com/feliblo/dbtective/commit/d228c92c689f535cd701950f6288d5bea135f394)) - Felix Blom
- **cfg**: add catalog tests to config - ([e0bc29c](https://github.com/feliblo/dbtective/commit/e0bc29cb6b0f9f725c07ff0db649188f276fc967)) - Felix Blom
- **checks**: run checks on all (data-related) manifest objects - ([67c5ebe](https://github.com/feliblo/dbtective/commit/67c5ebe9e6165313f409eb5502317f2bc2499d0b)) - Felix Blom
- **applies_to**: add more possible targets to applies_to - ([cae2641](https://github.com/feliblo/dbtective/commit/cae2641e92a8c24e209192f8e2f1a763f4fa971e)) - Felix Blom
- **cli**: implement catalog into cli - ([d09c0ae](https://github.com/feliblo/dbtective/commit/d09c0ae2243d82a764c683bae84ed279f2d71702)) - feliblo
- **catalog**: add catalog parser - ([890a550](https://github.com/feliblo/dbtective/commit/890a550c1311ef997ad3af5af81b3e7a083c39ec)) - feliblo

### Fix

- **checks**: fix applies_to for columns_are_documented and clippy warnings - ([9d237ec](https://github.com/feliblo/dbtective/commit/9d237ecfb35e305df374ef905baaa1119c0b530d)) - Felix Blom

### Refactor

- **checks**: change filename to other_manifest_object_checks.rs - ([cbd6081](https://github.com/feliblo/dbtective/commit/cbd608145b050aec5c90e9999af31bc0d6f8640f)) - Felix Blom
- **catalog**: preparation for catalog based tests - ([c6c27a5](https://github.com/feliblo/dbtective/commit/c6c27a5fcd7f7017f30af04f53f0cb5cd3b559a0)) - Felix Blom
- **run**: refactor run to use unwrap_or_exit helper - ([1bed05a](https://github.com/feliblo/dbtective/commit/1bed05a986aef295d551f1e019d6ecdb915439fa)) - Felix Blom
- **rules**: change ruletypes and applies_to setup for manifest & catalog checks - ([d0c0a47](https://github.com/feliblo/dbtective/commit/d0c0a478bcbddda7a566e023239971468131ddbe)) - Felix Blom
- **manifest**: change dbt_objects module into manifests - ([bf71136](https://github.com/feliblo/dbtective/commit/bf71136c006954e21ff27f09bc6d6836b9f67ac1)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo), [@feliblo](https://github.com/feliblo)

## v0.1.5 (2025-11-30)

### Feat

- **pypi**: add pypi release pipeline - ([e4a96be](https://github.com/feliblo/dbtective/commit/e4a96befebfabda0cdcc3ace749832129df12c6d)) - [#32](https://github.com/feliblo/dbtective/pull/32) - [@feliblo](https://github.com/feliblo)
- **table**: make table messages clickable - ([4db6d0c](https://github.com/feliblo/dbtective/commit/4db6d0c3e007845c611d310082719432dfd27283)) - feliblo
- **checks**: add naming convention check - ([48ff150](https://github.com/feliblo/dbtective/commit/48ff1508419e4d3b96ffeea0812c0bd12b60fa75)) - feliblo
- **cli**: make table clickable to go to files - ([419b318](https://github.com/feliblo/dbtective/commit/419b318f9c95c5c21b8ed6904c45359e937788b4)) - feliblo
- **config**: implement includes/excludes arguments - ([45d5dd2](https://github.com/feliblo/dbtective/commit/45d5dd23eecb4159b5a4b985ec15aee1f5408de6)) - Felix Blom
- **rules**: add includes/excludes for rule paths - ([71a76b7](https://github.com/feliblo/dbtective/commit/71a76b7fd1c17a1fead8d16b8ad87c2c28e22c1f)) - Felix Blom
- **applies_to**: Add apply_source_tests using applies_to - ([4eac04c](https://github.com/feliblo/dbtective/commit/4eac04cc258c77dde9bc97dea06f8e0d2a95a71c)) - Felix Blom
- **config**: handle valid applies to - ([bc619a7](https://github.com/feliblo/dbtective/commit/bc619a79f502d335d562e8b08574b81dd38f6295)) - Felix Blom
- **config**: intialize config rule hints - ([ba165f4](https://github.com/feliblo/dbtective/commit/ba165f48a2184011311fe72c70510c95284d8b9b)) - Felix Blom

### Fix

- **ci**: enable homebrew publishing - ([cff763b](https://github.com/feliblo/dbtective/commit/cff763bbd76c90a36219d3d2d72ec7fe6bc829cc)) - feliblo
- **cli**: show warnings in output table - ([fa1c7fa](https://github.com/feliblo/dbtective/commit/fa1c7fabb571ff781a76cc14a1d13a66a195eb42)) - Felix Blom

### Refactor

- **release**: use cargo-dist for release pipeline - ([ec2a269](https://github.com/feliblo/dbtective/commit/ec2a269e3ac4bfd113595f70d2425bace7bf4539)) - [@feliblo](https://github.com/feliblo)
- **anyhow**: propagate errors & impove/introduce (integration) testing - ([ef9289a](https://github.com/feliblo/dbtective/commit/ef9289a40b500286e887a96d8cda539e012a336d)) - Felix Blom
- **AppliesTo**: change appliesto to work on all manifest objects - ([8aa2e67](https://github.com/feliblo/dbtective/commit/8aa2e67c4c7231a640aa4d86ca52e953a7a2ab5f)) - Felix Blom
- **config**: config module refactor into components - ([680bdd7](https://github.com/feliblo/dbtective/commit/680bdd706407c7d307b41a6dec9249d9869a23a9)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo), [@feliblo](https://github.com/feliblo)

## v0.1.0-alpha (2025-11-25)

### Fix

- fix output styling - ([8a9c1d4](https://github.com/feliblo/dbtective/commit/8a9c1d428eb1757cba5dd5bb49f1fbe21e313121)) - Felix Blom

### Refactor

- **node**: remove copying around data in descriptable naming - ([935b81f](https://github.com/feliblo/dbtective/commit/935b81fb0d04c3b6059efef86443b24ffcfd85a6)) - Felix Blom
- refactor rule messaging (free memory) - ([a74a7bd](https://github.com/feliblo/dbtective/commit/a74a7bdb08801083f3612e21642087507332f4c7)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo)
