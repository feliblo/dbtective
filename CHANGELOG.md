## v0.3.3 (2026-05-05)

### Feat

- **rules**: add node_dependency rule - ([8d32dc9](https://github.com/feliblo/dbtective/commit/8d32dc9f38e45bdf881e95f05c620fc2508e260a)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.3.2 (2026-04-06)

### Feat

- **rules**: add 2 dag position materialization rules - ([b8014ca](https://github.com/feliblo/dbtective/commit/b8014ca32cd56e4ba1db2edb4d43a86c9cddbc11)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.3.1 (2026-03-25)

### Feat

- **rules**: add property_file_colocation rule - ([ae7c4f6](https://github.com/feliblo/dbtective/commit/ae7c4f64eb8caf0a761cb25406e8ffb0f792430a)) - feliblo
- **rules**: add has_required_tests - ([45b5886](https://github.com/feliblo/dbtective/commit/45b588645b4a06ed91f72c9d92e447dcfcb53dab)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.3.0 (2026-03-01)

### Feat

- **udf**: add support for udfs - ([d0a88eb](https://github.com/feliblo/dbtective/commit/d0a88ebcaa138967b2449f8e322de41bf68d4158)) - feliblo
- **udf**: add parsing for udfs - ([7b94b4d](https://github.com/feliblo/dbtective/commit/7b94b4da9e0f6df2a0c42aba8a8e24911d553188)) - feliblo
- **init**: add modelling/layers: inmon, kimball, datavault - ([3caabad](https://github.com/feliblo/dbtective/commit/3caabad073c79e65a9bbaaac15668335ace75b45)) - feliblo
- **rules**: add code_no_hardcoded_refs rule - ([c0aa268](https://github.com/feliblo/dbtective/commit/c0aa26816b9117350726e8fb0cc0473cc25afec1)) - feliblo
- **bin**: strip debug info for smaller binary size - ([539fbac](https://github.com/feliblo/dbtective/commit/539fbacadb8d5475323c064ca8a631eefa10c7bf)) - feliblo

### Fix

- **init**: enhance modelling/layers logic - ([a8a476d](https://github.com/feliblo/dbtective/commit/a8a476d6d33e41b27b6912abb681f950cb4c3259)) - feliblo
- **clippy**: clippy warning fix - ([529b381](https://github.com/feliblo/dbtective/commit/529b3810806a0d836e5aa65bc1e4fc59402bbbb9)) - feliblo
- **tests**: add more tests for code_no hardcoded_refs - ([332ea9f](https://github.com/feliblo/dbtective/commit/332ea9f6f3094832c9f388019fb78b5df1ceb252)) - feliblo

### Refactor

- **rules**: rename code rules - ([74df105](https://github.com/feliblo/dbtective/commit/74df1059ce16ddf12431c22ed0a8dcd31ec531c6)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.2.10 (2026-02-20)

### Feat

- **config**: add name: and tag: to includes/excludes - ([d824625](https://github.com/feliblo/dbtective/commit/d82462583279a55b261238603631a272beba5daa)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.2.9 (2026-02-20)

### Feat

- **rules**: add rule categories shown in structured output - ([051316c](https://github.com/feliblo/dbtective/commit/051316c715a0883b900c81f9e31f739b512a6ee0)) - feliblo
- **performance**: add cargo bloat on pull-request - ([52e81e4](https://github.com/feliblo/dbtective/commit/52e81e4881fcb2d2407d92c3394874a6fdc1e845)) - feliblo

### Fix

- **init**: fix init output config format not working - ([3f4ec10](https://github.com/feliblo/dbtective/commit/3f4ec101ac0859264b5d2f08836c344fb1cb0e4c)) - feliblo
- **ci**: fix codecov report - ([5f706ae](https://github.com/feliblo/dbtective/commit/5f706ae5e73187bcb0a178866046491469f6503c)) - feliblo

### Refactor

- **enums**: force sorting for specified enums using remain - ([e0ba7e8](https://github.com/feliblo/dbtective/commit/e0ba7e87643988da77c9b2bda7313c5da04eb69c)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.2.8 (2026-02-20)

### Feat

- **cli**: add --auto-parse option to run - ([82946c0](https://github.com/feliblo/dbtective/commit/82946c06d70b1ead1cf6f6b998d5cdaeb91828ab)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.2.7 (2026-02-19)

### Feat

- **rules**: create max_downstream_dependencies & max_downstream_dependencies - ([b0c349e](https://github.com/feliblo/dbtective/commit/b0c349ed0b8f0686d0532d60c5e704f494708d9e)) - feliblo
- **rules**: add use_database_columns to columns_name_convention rule - ([f04fdf2](https://github.com/feliblo/dbtective/commit/f04fdf297754f9ec144eafc7616173266b99afa5)) - feliblo
- **rules**: create 'max_joins' rule - ([71a4534](https://github.com/feliblo/dbtective/commit/71a4534397b9d62f57288eff3b8aab170fa254ad)) - feliblo
- **rules**: add code_contains_refs rule - ([9a1e4c2](https://github.com/feliblo/dbtective/commit/9a1e4c2edaf6076aa1ba93bc9a5ec0b3f5a267b1)) - feliblo

### Fix

- **rules**: fix return message of has_unique_test - ([c55c8b9](https://github.com/feliblo/dbtective/commit/c55c8b9c4961d904d990418a85c440dcae164986)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.2.6 (2026-02-17)

### Fix

- **rules**: fallback on depends_on.nodes for deciding rule parent - ([4037b41](https://github.com/feliblo/dbtective/commit/4037b41925613a2a1cd1a090b850ff074558119b)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.2.5 (2026-02-17)

### Fix

- **bug**: fix namespace issue in test metadata - ([86f21fd](https://github.com/feliblo/dbtective/commit/86f21fdd24a55b86ef5f6c446c6c548916ec0a57)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.2.4 (2026-02-17)

### Feat

- **table**: add hyperlink prefer_sql argument - ([c442058](https://github.com/feliblo/dbtective/commit/c44205812a7e99f8067d73388f4278c36becc2bb)) - feliblo

### Fix

- **rules**: remove exposure from has_refs rule - ([159f6b5](https://github.com/feliblo/dbtective/commit/159f6b5334d387406bb8c48ee3ca955dd5fa5b6a)) - feliblo
- **bug**: fix bug where includes/excludes didn't get picked up - ([042e4fb](https://github.com/feliblo/dbtective/commit/042e4fb6290352bc021c202fb5999a621faaddc1)) - feliblo
- **bug**: fix message in is_not_orphaned - ([ad98d19](https://github.com/feliblo/dbtective/commit/ad98d19be720bb62b11080f1fd0a56b8baf5115e)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.2.3 (2026-02-16)

### Feat

- **output**: add json, csv and ndjson output options & add output-file option - ([03f0085](https://github.com/feliblo/dbtective/commit/03f0085c10b024fc14b0064022a4bfee4733e099)) - feliblo
- **rules**: add columns_have_data_type rule - ([5313d43](https://github.com/feliblo/dbtective/commit/5313d4394428a5a2b1d694f5f7801ee4fa56b86c)) - feliblo
- **rules**: add has_forbidden_code rule - ([3e458c9](https://github.com/feliblo/dbtective/commit/3e458c94e81c7a7e166091fde02f0330db24a339)) - feliblo
- **init**: add marts/gold models should be exposed to init command - ([a2e6d82](https://github.com/feliblo/dbtective/commit/a2e6d82d0afa6c2475fb1a42b4590ce8e862b38a)) - feliblo

### Fix

- **cli**: show better skipped tests and manifest fallback tests - ([57b766b](https://github.com/feliblo/dbtective/commit/57b766bbbe0db2ea9d44368086aeb11f40e33c65)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.2.2 (2026-02-13)

### Feat

- **rules**: add min_length and forbidden substrings to has_description rule - ([71b2930](https://github.com/feliblo/dbtective/commit/71b29300ce4c837da8ac3de64721bb29acf0d9b5)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.2.1 (2026-02-11)

### Feat

- **rules**: add manifest fallback for eligible rules - ([c605eff](https://github.com/feliblo/dbtective/commit/c605effcb77c957375998af293f1abbe191d25ce)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.2.0 (2026-02-06)

### Feat

- **rules**: add sources_have_freshness rule - ([9ebc6ca](https://github.com/feliblo/dbtective/commit/9ebc6ca9b9fd5662356544a217a40e2a4312fd0a)) - feliblo
- **rules**: add sources_have_loader rule - ([94f3e62](https://github.com/feliblo/dbtective/commit/94f3e628d86584e023826d882a14d87f591cdc39)) - feliblo
- **rules**: enhance contract enforced with access level and add to init command - ([ed05c25](https://github.com/feliblo/dbtective/commit/ed05c25f2f18573ae43086ecfbc5e691fae0171d)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.33 (2026-02-05)

### Fix

- change catalog warning to stdout so it shows in prek/pre-commit - ([27ec13f](https://github.com/feliblo/dbtective/commit/27ec13f277056acd7d5220a384e8bbe3856dcc1d)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.32 (2026-02-05)

### Feat

- **table**: point to yaml file first in hyperlink if available (over sql) - ([2d09eb7](https://github.com/feliblo/dbtective/commit/2d09eb772ae56d24b0e5279d52cc64405896f7c8)) - Felix Blom
- **catalog**: add warning on how to fix catalog mismatches & fix pre-commit - ([522632b](https://github.com/feliblo/dbtective/commit/522632ba9a50f3ac72bf6d0bb1b7d9308f39dd83)) - Felix Blom
- **init**: change init to be a questionaire - ([4d49799](https://github.com/feliblo/dbtective/commit/4d49799408cd9f8eda0123e0d32507ff588bb450)) - Felix Blom
- **rules**: add allowed_subfolders_rule - ([a6c4cec](https://github.com/feliblo/dbtective/commit/a6c4cec858f1be67bf012c9f444a343bb3d11d60)) - Felix Blom

### Refactor

- **impls**: remove unneeded impls for references - ([bad7a4e](https://github.com/feliblo/dbtective/commit/bad7a4e92bbae92b99d2e58dcbc935ba3c81f004)) - feliblo
- **nodes**: delegate methods to subobjects for compiler messages - ([678ef6c](https://github.com/feliblo/dbtective/commit/678ef6cc42e60e7034b85b8c8b1ff20c1f5333ca)) - feliblo
- **impls**: add common Identiable trait for objects representation - ([77fd01b](https://github.com/feliblo/dbtective/commit/77fd01bb53ab065de99b746b8fe2215c21d75ecd)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo), [@feliblo](https://github.com/feliblo)

## v0.1.31 (2026-01-03)

### Feat

- **windows**: add windows test and improve includes_excludes behaviour - ([6cc36e8](https://github.com/feliblo/dbtective/commit/6cc36e83eda3d15ff528c6e12ee4a4dfdcb7faa5)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.30 (2025-12-31)

### Feat

- **cli**: add --hide-warnings flag & fix exit codes - ([8753bc9](https://github.com/feliblo/dbtective/commit/8753bc9a2f22dc66a33b5d3460bbe2a3515706b0)) - feliblo
- **rules**: add columns_canonical_name rule - ([5edf237](https://github.com/feliblo/dbtective/commit/5edf23758f413d277253675d4e1781da5efcff9d)) - feliblo
- **dbt**: refactor to dbt_artifact_parser crate - ([c494c5b](https://github.com/feliblo/dbtective/commit/c494c5b4f7bfc95ba2dafe7b89869a7baabda0b7)) - feliblo
- **filters**: add model materialization filter - ([e7ef995](https://github.com/feliblo/dbtective/commit/e7ef9959f09ab8b5666b0248d3f197c4062a90a0)) - feliblo

### Fix

- **rules**: add exceptions to columns_canonical_name - ([ec6bee4](https://github.com/feliblo/dbtective/commit/ec6bee41991a126d4d8956e3b6ade5be8ca17296)) - feliblo

### Refactor

- **tests**: refactor test folders - ([73bd775](https://github.com/feliblo/dbtective/commit/73bd77533274b217142095ce367657ed2e451c03)) - feliblo
- **test**: add rule::from_specific_rule test setup - ([c0dbad6](https://github.com/feliblo/dbtective/commit/c0dbad6f0c2eeacb7864970684548ccba1e6166a)) - feliblo
- **regex**: refactor regex to parse within config - ([8c3dcad](https://github.com/feliblo/dbtective/commit/8c3dcadb98921d69abfca0216d90b895d28dd746)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.29 (2025-12-24)

### Feat

- **rules**: add data type filter to column naming conventions - ([9ce56b1](https://github.com/feliblo/dbtective/commit/9ce56b19e283b34b6883306f7ff1393c76b2e2df)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.28 (2025-12-17)

### Feat

- **rules**: add max_code_lines rule - ([b26d293](https://github.com/feliblo/dbtective/commit/b26d293992a619fe275402384f090e0d525656c9)) - Felix Blom
- **init**: add column_name_convention to init - ([d0a5d73](https://github.com/feliblo/dbtective/commit/d0a5d73f3efba8a61c160722f963c97c49564e76)) - Felix Blom

### Refactor

- **rename**: massive renaming of all occurances of checks -> rules - ([b326876](https://github.com/feliblo/dbtective/commit/b326876f1b42ff76bdaea9f659b1b3680cd97726)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.27 (2025-12-16)

### Feat

- **rules**: add columns_name_convention rule - ([aa80ee7](https://github.com/feliblo/dbtective/commit/aa80ee78122f3fe3c7288837475da14462f58217)) - Felix Blom
- **rules**: add 'has_refs' rule - ([5e2a87d](https://github.com/feliblo/dbtective/commit/5e2a87df56d681c2a58c535d3131ef88d8d9ad8c)) - Felix Blom

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.26 (2025-12-14)

### Feat

- **cli**: add dbtective init command - ([5f2f50a](https://github.com/feliblo/dbtective/commit/5f2f50a4d4d6943d77ae932286c3e0c8127509c7)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

## v0.1.25 (2025-12-12)

### Feat

- **rules**: add has_metadata_keys_rule - ([74f74bb](https://github.com/feliblo/dbtective/commit/74f74bbf025adfeaba8af35630d0a1469371b5cd)) - feliblo

### Fix

- **macro**: add relative path to descriptable trait - ([53fdb48](https://github.com/feliblo/dbtective/commit/53fdb4857f896d9a5731ac9240d606bb0aa0e864)) - feliblo
- **config**: pyproject.toml is only a valid config if it contains a dbtective section - ([b5358d8](https://github.com/feliblo/dbtective/commit/b5358d87df11c67829f3fb6c9da9ad4302ffa220)) - feliblo
- **catalog**: set byte description to option (it can be null) - ([0d44475](https://github.com/feliblo/dbtective/commit/0d44475b480d7fec6bf61234f2f705ac59338907)) - feliblo

### Contributors

[@feliblo](https://github.com/feliblo)

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
