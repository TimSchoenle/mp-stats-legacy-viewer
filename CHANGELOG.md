# Changelog

## [0.10.0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.9.0...v0.10.0) (2026-02-28)


### Features

* add a prototype warning on the home page ([f0907ce](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f0907cea86155f137fba7e6dfd3c0baecfaa2fe5))
* add compressed data set ([5d22c09](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/5d22c09ce83abec96e15e1ac71f2934993cd7f7c))
* add early prototype ([64f98a9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/64f98a92d3eea0ec51cbb3ea49fa962759b3d428))
* add propper liveness checks for server startup ([#26](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/26)) ([ec93562](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ec93562f5b4a655884788a12a68801d02668a7a2))
* **Data:** improve leaderboard meta ([2087ee3](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2087ee375998cdf9176e93b149475e5f704ece4f))
* **Data:** unify bedrock and java export data ([fea906b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/fea906b800aefba0608392b9bb101c2762228c3f))
* filter historic data to reduce amount ([4b57a2a](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/4b57a2abe305fd25f6c168d1f605fa65420d88ef))
* **Frontend:** add custom score formatter ([#55](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/55)) ([30f36e8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/30f36e898473fa8cba718781ae81417f14d5a7b3))
* **Frontend:** add name search feature ([#36](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/36)) ([6620c6c](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/6620c6cd311200feefc48825ecc846a91d697215))
* **Frontend:** add propper caching layer ([#50](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/50)) ([6d2074b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/6d2074b6ecc5f5938bdda5fef27a7eecef941c45))
* **Frontend:** add propper home page description section ([#34](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/34)) ([00655ef](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/00655ef23c754447b5c4b3c5c11bb90f0d717afe))
* **Frontend:** cleanup Trunk usage ([a383b79](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/a383b79bfe64706db3324591bd7e049c5c2cbaf8))
* **Frontend:** correctly format snapshot dates ([#48](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/48)) ([0e49f0f](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/0e49f0f07a7b14c428811eadb4aab8927ed591c4))
* **Frontend:** limit player profile to all data ([#54](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/54)) ([d346774](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/d3467748bce6190de3b3dca68040a66cb9ea7bae))
* **Frontend:** simplify api ([#30](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/30)) ([46794c8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/46794c8f03c111bff44eb730273a6006f5797968))
* fully disable SSR ([4c2e1c2](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/4c2e1c2e171bad976639696ac0c36382a1a868a7))
* increase data compression preset to 9 ([cdb2e85](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/cdb2e855923b5a8f1e8f674dc601940ef86c5958))
* standardize UI  ([#39](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/39)) ([1865a59](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1865a5982eee02663b33a13cbacf79cc8a17a402))
* unify leaderboard logic and implement bedrock loading ([c07268e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/c07268ecc6932f6db556b66419fe413c81535060))


### Bug Fixes

* **Frontend:** correctly sort snapshots ([#44](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/44)) ([b9af9b4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b9af9b4c450886e39aa0d743e9173bb1873b861f))
* **Frontend:** improve player page ([#37](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/37)) ([2c194b9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2c194b9d9b94391430b7f7ba17f1ac6a6611d224))
* **Frontend:** total pages calculation ([#49](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/49)) ([9311373](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9311373fd2b483740ab7e8de8c3cb1dac0c07570))
* incorrect docker repo ([42ce970](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/42ce9708628f45b8452ea6344270c7ce97f65fcc))
* **Leaderboard:** latest total entry calculations ([f794a4f](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f794a4f3c7e98142181e2f3a554048001ac97b09))
* model parser behaviour ([#42](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/42)) ([a97ed6d](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/a97ed6dc02d330dd9aed95806d883b449e40c4f6))
* **Player:** invalid direct link to leaderboards page ([#40](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/40)) ([41d4455](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/41d4455d7d162a859fca12e79fcd27021774c068))
* remove dashmap ([#52](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/52)) ([83bad42](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/83bad426b0a5646da8d592fd41b9a85b0c6cc2c0))
* restore npm dependencies for trunk ([3c2a3b2](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/3c2a3b26c5910cda1fabb6e45933a210e4622c5b))
* show the uuid as the player name if no player name is found ([ef0e204](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ef0e2049dd4b29b7903c0a28a57e50ffcde56624))
* trunk tailwind setup ([43aff28](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/43aff2869d68f85f86de0958f0372bffe9bc616a))


### Code Refactoring

* cleanup models ([#31](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/31)) ([02b031d](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/02b031d39f0d45aab92a4a035ffd7468ec070c5b))
* **Frontend:** split components into sub components and update h… ([#32](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/32)) ([07cad60](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/07cad60cfc387753d98ac1f4dbdaddf62df4b020))
* replace dead lzma-rs with lzma-rust2 & remove deprecated zlib ([7317d78](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7317d78f589a177ccd80ed066bf13342c381c574))
* start cleanup ([ce7a5f3](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ce7a5f318176b7b40c02d2b344b5da2c317a0f1e))
* start splitting leaderboard components and logic ([af84cb3](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/af84cb3813d0b640d84b2757b42e87b87460e007))


### Styles

* auto-format rust code ([89ea026](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/89ea02687b2176f679dffd76cb61aa388c6fc568))
* auto-format rust code ([7ed77c9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7ed77c9b81a61aa751ba44f8a237709aab6aee82))
* auto-format rust code ([cbd0fd2](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/cbd0fd25582b0d78914454d32f2e29082bf28204))


### Build System

* add basic build check action ([ff846eb](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ff846eb20f0f71304dd5e54cba17f1a45270aae8))
* add cargo lock update action ([#57](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/57)) ([f315495](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f3154955ad05e6766b0fd98b614195cc16b76f10))
* add CI automation actions ([#60](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/60)) ([061e8af](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/061e8af009865b35073579a0af277514199ee41b))
* add propper docker build test ([ef60c66](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ef60c66fb2194eb83e644e444729983db1edbe98))
* add release please ([fac09e9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/fac09e95d1053e698f52804617bc28bcb424e0a7))
* add renovate ([1294b35](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1294b356a304048a2bba7c71c941aa6d7cd0cebf))
* centralize version across all crates ([2f30f2e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2f30f2e2588da991a1fc81a69b39a12c4063eacb))
* cleanup unused dependencies and update existing ones ([49dff78](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/49dff78594c14d13e8b694fa89caa40a12b61937))
* **Data:** add data-test sub set of the global data for testing purpose ([525f7ef](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/525f7eff414ac260f7468008eab77b5c44491d7b))
* **Data:** use higher compression levels ([1b52479](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1b52479c40bd9281c27ffb4512d3990790300142))
* **Docker:** add DATA_INPUT_DIRECTORY to select data input directory ([8adac9b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/8adac9b209a19752047af8ee10521f82b4bd0b66))
* maybe this will fix release please ([3b21a46](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/3b21a4688b9c2f42575d7be80dff9e6e2fdcc06b))
* maybe this will fix release please2 ([74fa019](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/74fa019126663a5896325ed4b137de5b65c44dc0))
* maybe this will fix release please3 ([20488a1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/20488a14536198a5b29f7d9211c7f27549fcdaa6))
* maybe this will fix release please4 ([01f2d93](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/01f2d93770227ecd0df10bdeb5e3c8adea7e5bf9))
* maybe this will fix release please5 ([3bb55a1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/3bb55a10929540cd40050853983c438b12e6b945))
* maybe this will fix release please6 ([dde743b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/dde743badea39b54ef65d2bbcdb5d292c6b95407))
* maybe this will fix release please7 ([dd8b0ee](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/dd8b0ee74b4ffd8556b19ace4a9b1d8c28837269))
* maybe this will fix release please8 ([f191454](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f191454bb3751f7d7f0e8c292b3a373e54668a45))
* never cancel release please action ([f3614eb](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f3614ebedbf2bc253d013a418ef12f15f7351b11))
* update data sources ([deb2900](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/deb2900f1f9af720e6a5b13840e641f4495794bc))
* update Dockerfile to scratch image with cargo cheff ([#28](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/28)) ([544e511](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/544e511353efda1efb387e2b41d505e91380ea51))
* update lock ([b3d0650](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b3d065001b46e661455bf8fb2e1d9188443abe15))
* update lock ([724108e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/724108e32b125350c5e5c73834e14d6f31367e0d))
* update lock ([dfadf08](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/dfadf085b9eb5a30c33b49a7ddb30f9759448062))
* update version lock ([e7d8c44](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/e7d8c440d02db23eb5b296781f5bb89078e49e68))
* update version lock ([4608892](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/46088924e039bc35fa8379511105a7cdc4ba2c8a))


### Miscellaneous

* **deps:** pin dependencies ([#4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/4)) ([b44644c](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b44644c3986e79fc1be2ff9816149da0df05ed60))
* **deps:** update all non-major action updates ([#19](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/19)) ([26c3095](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/26c3095528f11d0d203cc719e66f291a2cff260d))
* **deps:** update all non-major action updates ([#61](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/61)) ([a2295b9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/a2295b9c60cd7b837594cd0ba37aa75668197e15))
* **deps:** update dependency tailwindcss to v4 ([#15](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/15)) ([beac15c](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/beac15c51f667e135d5d620cbddf9ad2c29cefbf))
* **deps:** update dependency tailwindcss to v4.2.0 ([#17](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/17)) ([29dde49](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/29dde4982ea93846be878cf215cc7f0ad64ebfc8))
* **deps:** update dependency tailwindcss to v4.2.1 ([#56](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/56)) ([9f0a2dc](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9f0a2dcbfb08d1a51fe9d26550147c97f1f1eb20))
* **deps:** update github/codeql-action action to v4.32.4 ([#25](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/25)) ([aff8eca](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/aff8ecac08f6f53355e2d1d11bb8545b05983815))
* **deps:** update rust crate anyhow to v1.0.102 ([#22](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/22)) ([ce67a0e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ce67a0e491fd325da8b4db23fbc686ec1fc74574))
* **deps:** update rust crate axum to 0.8 ([#8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/8)) ([67de303](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/67de303a1a963e23575529fe534e36643fa470a4))
* **deps:** update rust crate clap to v4.5.59 ([#5](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/5)) ([b91fd17](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b91fd17dcf71bc67d097d1895b505cdcd4421a23))
* **deps:** update rust crate clap to v4.5.60 ([#21](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/21)) ([e077291](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/e077291a8305ea5fb63db7880a5b46c15976f041))
* **deps:** update rust crate futures to v0.3.32 ([#6](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/6)) ([0a666a0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/0a666a02ea02d5965cb86cac7a24701b7eaf37f7))
* **deps:** update rust crate indicatif to 0.18 ([#9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/9)) ([7108848](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7108848c379b99645b89a6dab0447fed2980fb38))
* **deps:** update rust crate smol_str to 0.3 ([#10](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/10)) ([7b06542](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7b0654261163f302b85e61d8a1028626bf9ff641))
* **deps:** update rust crate tower to 0.5 ([#12](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/12)) ([04c6c07](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/04c6c0718a759ef3b7913f099156e4ddf10b8fc7))
* **deps:** update rust crate tower-http to 0.6 ([#13](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/13)) ([452c197](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/452c1977df412977536b6fc1bc89622990ad90dc))
* **deps:** update rust docker tag to v1.93 ([#14](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/14)) ([a032ba5](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/a032ba5e680eda5b89ac46b23fb9f6d3adbd8dc5))
* **deps:** update rust:1.93-slim docker digest to 7e6fa79 ([#46](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/46)) ([81ee9e2](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/81ee9e2d2b614bfe6c8322efad4fc4ab580b1e54))
* **deps:** update rust:1.93-slim docker digest to c0a38f5 ([#62](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/62)) ([4689cd9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/4689cd9304b81da875bf6bf8ba9f2be55f98d42d))
* **deps:** update step-security/harden-runner action to v2.15.0 ([#47](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/47)) ([73b764c](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/73b764c1fdda48e0f0890d7926406b9b54981ea1))
* **deps:** update step-security/harden-runner action to v2.15.0 ([#59](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/59)) ([caae6af](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/caae6af07db01912ed55b274c869ea6e99a3e703))
* **deps:** update to vactions-helm-update-chart-version-v1.5.1 ([#11](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/11)) ([c0cfc09](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/c0cfc096de774ca6f74dec74f6831d606dbebc07))
* **deps:** update to vactions-helm-update-chart-version-v1.5.2 ([#38](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/38)) ([e534447](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/e534447761b06f6f443bcfce853a53ce61fc644a))
* fix test failure ([95b443a](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/95b443a46dcb531cd701c13014805b13ccf117c0))
* **main:** release 0.2.0 ([#1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/1)) ([2f00edb](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2f00edbc70c18cab448d36968a7d4475a61217f1))
* **main:** release 0.2.1 ([#2](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/2)) ([6da4997](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/6da4997c9c1fbf03322b165d890ee40541d5561f))
* **main:** release 0.3.0 ([#3](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/3)) ([6758bcc](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/6758bcc0aa86c0aca7e7990090537d5cd8fb8d99))
* **main:** release 0.3.1 ([#20](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/20)) ([8f632f4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/8f632f4dbcf0a1d96912423bfc71c5d878296c1e))
* **main:** release 0.3.2 ([#23](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/23)) ([d3e211a](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/d3e211a233a3e69ac3b9c13b51887a0026550e7c))
* **main:** release 0.4.0 ([#24](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/24)) ([2a2b1c4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2a2b1c4b4a8699f9a95300eb67fb7e84458bf458))
* **main:** release 0.5.0 ([#27](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/27)) ([3aeef17](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/3aeef17ab22dd05571362e1709bb81663f2148d9))
* **main:** release 0.6.0 ([#29](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/29)) ([b6327b1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b6327b1115d94a0e3a85e353470239f473a369d7))
* **main:** release 0.7.0 ([#33](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/33)) ([b720844](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b72084485a3dfaf041ba75e1148f4215cd106790))
* **main:** release 0.7.1 ([#41](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/41)) ([0c25949](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/0c2594921fd28b7f587c0528e8414dbf466ce48e))
* **main:** release 0.7.2 ([#43](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/43)) ([ce8fe8e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ce8fe8e87f556ecb8c39ef935ab8a01f4f42f5d0))
* **main:** release 0.8.0 ([#45](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/45)) ([cfef3e8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/cfef3e8cda7632aa052fabeccccee20f78241c0b))
* **main:** release 0.8.1 ([#51](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/51)) ([9f630fd](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9f630fd00886be7781c876c2b891f169a1fa9c6d))
* **main:** release 0.9.0 ([#53](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/53)) ([13d085c](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/13d085cfb0607c8aa427db857818155452a8a112))
* update cargo lock ([359ecf9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/359ecf99f79af1a8054e7de589b799e4611c3a4a))
* update lock ([9a7adcd](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9a7adcd7c3ae86b3434873a292d07d3690202a6f))


### Dependencies

* **deps:** lock file maintenance ([#16](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/16)) ([61663c6](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/61663c60e47709546ca0ec0fb3cd118d3d32ad91))
* **deps:** lock file maintenance ([#35](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/35)) ([347b52d](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/347b52d67dc1d4f51df42f410fc079567f0d8e61))

## [0.9.0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.8.1...v0.9.0) (2026-02-25)


### Features

* **Frontend:** add custom score formatter ([#55](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/55)) ([9fdd3aa](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9fdd3aae8dcc052e159e038569c7507c715ceefe))
* **Frontend:** limit player profile to all data ([#54](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/54)) ([4a0831b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/4a0831b21ea150ef5328797e9493dd71502da120))


### Build System

* update version lock ([f76ca22](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f76ca22e4773cea7792af7b1b6dd72e1b1e51ee5))

## [0.8.1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.8.0...v0.8.1) (2026-02-25)


### Bug Fixes

* remove dashmap ([#52](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/52)) ([9564596](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/956459663b3593cfe371dceaea807043541906a1))


### Build System

* update version lock ([0114066](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/0114066f16e341f233fb59c0e28de4dc28fe2ef2))

## [0.8.0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.7.2...v0.8.0) (2026-02-25)


### Features

* **Frontend:** add propper caching layer ([#50](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/50)) ([ca9690e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ca9690ef85b6caa6a00c0a242d1bf635c9baba63))
* **Frontend:** correctly format snapshot dates ([#48](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/48)) ([2e847e2](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2e847e2d75cf05ab0fc1c40d57b9ec782faaedbc))


### Bug Fixes

* **Frontend:** correctly sort snapshots ([#44](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/44)) ([88098c9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/88098c927fea155f1b624eacfaa415d8b23ab650))
* **Frontend:** total pages calculation ([#49](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/49)) ([2463945](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2463945523669093b5c369a8087869f375af966d))


### Miscellaneous

* **deps:** update rust:1.93-slim docker digest to 7e6fa79 ([#46](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/46)) ([2d53385](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2d53385e8d6417902531647d309612be9156103d))
* **deps:** update step-security/harden-runner action to v2.15.0 ([#47](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/47)) ([7475f63](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7475f6300571540116759abd911ff1ea30059ca3))

## [0.7.2](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.7.1...v0.7.2) (2026-02-24)


### Bug Fixes

* model parser behaviour ([#42](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/42)) ([bf54a3e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/bf54a3ef4ac8cb29afaa6429ce93db66200827bd))

## [0.7.1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.7.0...v0.7.1) (2026-02-24)


### Bug Fixes

* **Player:** invalid direct link to leaderboards page ([#40](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/40)) ([729a43f](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/729a43ff27d74ead680b5ea928ca4224061aaa7d))

## [0.7.0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.6.0...v0.7.0) (2026-02-24)


### Features

* **Frontend:** add name search feature ([#36](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/36)) ([02779ea](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/02779ea5ac0c41e971c876c2b1194fcd7e3cec88))
* **Frontend:** add propper home page description section ([#34](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/34)) ([4cf506d](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/4cf506d9eaccadbaae4aec7bd9986202331b403c))
* standardize UI  ([#39](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/39)) ([2b0c46c](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2b0c46cdc298a09f739def9d68c881ded727076f))


### Bug Fixes

* **Frontend:** improve player page ([#37](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/37)) ([b7a80e8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b7a80e8e94d3407e1ca49cf452d1ac3ea24355a9))


### Code Refactoring

* **Frontend:** split components into sub components and update h… ([#32](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/32)) ([0368922](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/0368922e2a96151a0847b71a63eccc164edae873))


### Miscellaneous

* **deps:** update to vactions-helm-update-chart-version-v1.5.2 ([#38](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/38)) ([64cfd15](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/64cfd15ec693e071e259e7616bc1fc1d9b038949))


### Dependencies

* **deps:** lock file maintenance ([#35](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/35)) ([5622b2b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/5622b2b589418684a4358b2ce7455dc19ed9cb04))

## [0.6.0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.5.0...v0.6.0) (2026-02-22)


### Features

* **Frontend:** simplify api ([#30](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/30)) ([0750213](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/0750213c8023053b3a9193921e045b4303ab18c9))


### Code Refactoring

* cleanup models ([#31](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/31)) ([d28789e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/d28789e1fe30e530643675fe462011c94adef77e))


### Build System

* cleanup unused dependencies and update existing ones ([7c59897](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7c598978a403b517d485ea1f8a689fba16506722))
* never cancel release please action ([1414dc6](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1414dc6146422da89f9c02f4596eae22dbac723d))
* update lock ([bf6a411](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/bf6a4116361a68ef8c52e7a970a438956a4bc5d7))


### Miscellaneous

* **deps:** update github/codeql-action action to v4.32.4 ([#25](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/25)) ([81afbae](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/81afbae7f0fb75a83f1a7c800d83993d1173f21d))
* **deps:** update rust crate anyhow to v1.0.102 ([#22](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/22)) ([0f2ee3a](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/0f2ee3aac6490b36c39b6fe4a2eef1540d48b09f))
* **deps:** update rust crate clap to v4.5.60 ([#21](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/21)) ([2d1f77f](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2d1f77f834a1f41490d2440f8da2a7ad69edeef1))
* fix test failure ([552a90c](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/552a90c1ace40f099898fb6dcaab3feb983d59b0))

## [0.5.0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.4.0...v0.5.0) (2026-02-20)


### Features

* add propper liveness checks for server startup ([#26](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/26)) ([1cf7f16](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1cf7f164d7c55ea5da8b02a1f316090e797311e7))


### Build System

* update Dockerfile to scratch image with cargo cheff ([#28](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/28)) ([4d7853d](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/4d7853d93244ed01b0a2ba425cdb7fb751d54707))
* update lock ([02b2515](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/02b25159b4ceeceda8ddbffa769d46d5adee62b4))

## [0.4.0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.3.2...v0.4.0) (2026-02-20)


### Features

* increase data compression preset to 9 ([1dc341e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1dc341e95944bc1bb55063ecc8ad1b4ffa530aba))


### Build System

* add propper docker build test ([270a9f1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/270a9f157b88bf45ab50fb0261a3506cc31500a4))
* **Data:** add data-test sub set of the global data for testing purpose ([1a3e7f6](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1a3e7f637b013951e71f804370e9552852698177))
* **Data:** use higher compression levels ([25452a6](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/25452a6fcff2c35e93025510cac8a0855de8810e))
* **Docker:** add DATA_INPUT_DIRECTORY to select data input directory ([dbd9855](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/dbd98550044f80ae5dab16238234b9bb862e0938))
* update lock ([13ec6bd](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/13ec6bdfa0a072f23bc4cd93afadfcb3528fad9f))

## [0.3.2](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.3.1...v0.3.2) (2026-02-20)


### Bug Fixes

* restore npm dependencies for trunk ([1c7f5cd](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1c7f5cd17298688e0d0c1e41132c6f1cfc8849d3))

## [0.3.1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.3.0...v0.3.1) (2026-02-19)


### Bug Fixes

* trunk tailwind setup ([fb311a5](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/fb311a5be743552554a3b1d1536feb434f4f7329))


### Miscellaneous

* update lock ([4e72dbb](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/4e72dbbe23e153d4648cb60d9e9a129db1f06dbf))

## [0.3.0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.2.1...v0.3.0) (2026-02-19)


### Features

* **Data:** unify bedrock and java export data ([b8c1518](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b8c1518cc2b0968ee950017b68387040d56d7397))
* **Frontend:** cleanup Trunk usage ([ac7af7a](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ac7af7ac5307c1b901b246e865e6aa3c6c93ee3f))
* unify leaderboard logic and implement bedrock loading ([e34cd60](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/e34cd608052e74ab6b0315b681dba005aa8efe8f))


### Code Refactoring

* replace dead lzma-rs with lzma-rust2 & remove deprecated zlib ([91d38fc](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/91d38fcca8b652acbcee33ab894684a3b041cb57))
* start cleanup ([ddeedb0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ddeedb0b966abe52ef389641226fb5b752fe31e8))
* start splitting leaderboard components and logic ([6e41251](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/6e41251e5796db47a5714328c68e2d69ecd94f44))


### Styles

* auto-format rust code ([f6dbb56](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f6dbb56bffb7769cf20d75cc894edcfaf9ad9347))
* auto-format rust code ([7401777](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/74017778c0d7c061963719dfc1fd993ea8bd1db4))
* auto-format rust code ([7ec4c9a](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7ec4c9a2432fddc1d58415cec237213deca8c813))


### Build System

* add basic build check action ([2f8a35a](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2f8a35a2d7c2eb0c0d40e478b4f064977625761b))
* add renovate ([ed7c1a6](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ed7c1a696a0329726c74d7b37213b2f06ef31d0a))


### Miscellaneous

* **deps:** pin dependencies ([#4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/4)) ([1523e5a](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1523e5af96532dc362e0c84d811691ab9ed5803c))
* **deps:** update all non-major action updates ([#19](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/19)) ([91cfd7e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/91cfd7e1b844fe24e641b96944a2edf29c33a08f))
* **deps:** update dependency tailwindcss to v4 ([#15](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/15)) ([19842ea](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/19842ea6a6fe1d3bb9947cb92772775b7fa33c2a))
* **deps:** update dependency tailwindcss to v4.2.0 ([#17](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/17)) ([11a5627](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/11a5627a6ea43552f7ec8ef4065d4697882f626a))
* **deps:** update rust crate axum to 0.8 ([#8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/8)) ([25f4097](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/25f409778366def558986ba79a0e074bb348f53b))
* **deps:** update rust crate clap to v4.5.59 ([#5](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/5)) ([346bfbe](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/346bfbe486b98669c8f83db1151d454ff9b38717))
* **deps:** update rust crate futures to v0.3.32 ([#6](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/6)) ([244d512](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/244d512b5345e7ac63ea67b43d2a2eee4cae7b4d))
* **deps:** update rust crate indicatif to 0.18 ([#9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/9)) ([4eec899](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/4eec8995408f0b1a14d1888a1a11df0271129e55))
* **deps:** update rust crate smol_str to 0.3 ([#10](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/10)) ([7b5c1e7](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7b5c1e7f2b27446aad2783bcebdd07f9f175d81d))
* **deps:** update rust crate tower to 0.5 ([#12](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/12)) ([446e251](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/446e25137653de7c580df7401b8996098870f00e))
* **deps:** update rust crate tower-http to 0.6 ([#13](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/13)) ([9113967](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/91139673852c41e6c5d9f2a8503e77d8121614a8))
* **deps:** update rust docker tag to v1.93 ([#14](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/14)) ([972981b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/972981b5bd55d86a91cf8f2e57ed293768fa2288))
* **deps:** update to vactions-helm-update-chart-version-v1.5.1 ([#11](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/11)) ([7130d1f](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7130d1ffed481a17a23e2d4e7c86ad4da90ac213))
* update cargo lock ([12ce439](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/12ce4392e3555ea8b87f93eabb23be68a6fb15ad))


### Dependencies

* **deps:** lock file maintenance ([#16](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/16)) ([db03ca8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/db03ca8829f89a8f3e49551aed1724ecf1029b05))
* **deps:** lock file maintenance ([#18](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/18)) ([8331749](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/8331749f06a54976698094b6d2ec5ceb003d8717))

## [0.2.1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.2.0...v0.2.1) (2026-02-18)


### Bug Fixes

* incorrect docker repo ([385c056](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/385c056e02450479705e02d4598f001a0094bde5))

## [0.2.0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.1.0...v0.2.0) (2026-02-18)


### Features

* add a prototype warning on the home page ([98e3d61](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/98e3d613f0c161ae0d99a96c80923413830d6dc0))
* add compressed data set ([2573c0b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2573c0b640a20717634f4b5d26eecbbd159932c0))
* add early prototype ([1a79ac8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1a79ac8a7138c26d8458d85454ea480d2c06e013))
* **Data:** improve leaderboard meta ([7d13f5c](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7d13f5ce4208d106d19ad870b7217a7bbda25b7d))
* filter historic data to reduce amount ([8da4417](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/8da44173b23391c0e6a6edd5653b45eef4b049b8))
* fully disable SSR ([a49162b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/a49162ba56be1dbec7162151410e91f61f6ff8cd))


### Bug Fixes

* **Leaderboard:** latest total entry calculations ([5168616](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/5168616d07b1e660a463a10a8dc405949e676088))
* show the uuid as the player name if no player name is found ([c73e212](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/c73e21291f4a1175d6580c96851f2739ac1ab7fc))


### Build System

* add release please ([f8bf312](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f8bf312f209c11abbcb411f5e97167ef0d41933b))
* centralize version across all crates ([58990cd](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/58990cd9c0e6ed17f627fc7cf6d73636e9240fb4))
* maybe this will fix release please ([cf3ad78](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/cf3ad78c9725cb8070954b649a769a6b5c1b107c))
* maybe this will fix release please2 ([e628dff](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/e628dffc5e9dc5bf86913a9949dd0389af8a1d0e))
* maybe this will fix release please3 ([bfae8f8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/bfae8f80178f7feb79700f7e85335c3805bbcba3))
* maybe this will fix release please4 ([66ecf48](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/66ecf482e17b0babbf3b38b66245952bef758d37))
* maybe this will fix release please5 ([b9ace5f](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b9ace5fecfd60009e347aaa59f611b5fc137371b))
* maybe this will fix release please6 ([2bfcac9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2bfcac95e8cc18e9991b4bd25aecb0a411fe167a))
* maybe this will fix release please7 ([d2dddf8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/d2dddf8a2d4a906013e5e490dcdc2717f743e33f))
* maybe this will fix release please8 ([5b96e2d](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/5b96e2d09d9f32cf93a3f1fb84738278133891b4))
