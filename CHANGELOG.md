# Changelog

## [0.10.0](https://github.com/TimSchoenle/mp-stats-legacy-viewer/compare/v0.9.0...v0.10.0) (2026-02-28)


### Features

* add a prototype warning on the home page ([6d623ac](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/6d623ac287e8270cd48a9772786702522b9b109a))
* add compressed data set ([97bbd1b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/97bbd1b90e16493370a6f4657c0e8fbddf70dfb0))
* add early prototype ([08a9a44](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/08a9a4405d2c27eabb491c182f5136c6eb562ddf))
* add propper liveness checks for server startup ([#26](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/26)) ([f1a97cf](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f1a97cf87741d98b950be3ab92f3da8407b9e73b))
* **Data:** improve leaderboard meta ([a4944bf](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/a4944bf35fd93835d8a108f654b4947820a0c4b7))
* **Data:** unify bedrock and java export data ([a3c8ebb](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/a3c8ebb166f48bafd1ca333c491d530c3e6f0b62))
* filter historic data to reduce amount ([c8b800c](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/c8b800cc3951f56ab965cc931e9a118bf55bebcb))
* **Frontend:** add custom score formatter ([#55](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/55)) ([d546daf](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/d546daf8bb4cc28a3cb9ab01e98f82e9623ea89a))
* **Frontend:** add name search feature ([#36](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/36)) ([7134c19](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7134c199ae355637677ad8c288007c67ac320408))
* **Frontend:** add propper caching layer ([#50](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/50)) ([b6eccf3](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b6eccf3dd0f1d89bc28e2f8a57805df66495eb48))
* **Frontend:** add propper home page description section ([#34](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/34)) ([1dc8eeb](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1dc8eeb0e5b7cf1878cebd23fff4ed2443202e61))
* **Frontend:** cleanup Trunk usage ([7e335b7](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7e335b7423854177ad859a0e7e1c2bb2744cc9fe))
* **Frontend:** correctly format snapshot dates ([#48](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/48)) ([20921ad](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/20921ad1008f3a3fb01abe5374ffef27dceac891))
* **Frontend:** limit player profile to all data ([#54](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/54)) ([3cd9a1b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/3cd9a1b6f9dbe8327430447400b4c87f33117ed9))
* **Frontend:** simplify api ([#30](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/30)) ([fcc1ae6](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/fcc1ae6195c994bfe33d74ad0d4b4f91916eafb3))
* fully disable SSR ([071a154](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/071a154de47d1982f3685ecb52bb6ce629f231af))
* increase data compression preset to 9 ([b2be5ad](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b2be5ad83edbeffd07ad691844c2878adf5493ea))
* standardize UI  ([#39](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/39)) ([f7e9107](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f7e9107778566b8eaf86bc3ea3db5496a9cc9557))
* unify leaderboard logic and implement bedrock loading ([89867e4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/89867e4cae7c4623434dfa6c419c27e9bfac9561))


### Bug Fixes

* **Frontend:** correctly sort snapshots ([#44](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/44)) ([da8d2ed](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/da8d2ed472769e7b017b07e86c9376c916010b91))
* **Frontend:** improve player page ([#37](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/37)) ([829612f](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/829612f9a3cb0d584bbb74393518e1074bd7f7ed))
* **Frontend:** total pages calculation ([#49](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/49)) ([44d2d25](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/44d2d25ad1d9b838bf3c78a3c56b0e86ea235592))
* incorrect docker repo ([3bd569d](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/3bd569d9f570ba9f731030ace044da2711c28d1c))
* **Leaderboard:** latest total entry calculations ([28a6907](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/28a6907e1a28cb8e45b31b0d51b3f0bd63d91f2a))
* model parser behaviour ([#42](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/42)) ([a35757a](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/a35757abd38e800bc59b798aa98737a469de9089))
* **Player:** invalid direct link to leaderboards page ([#40](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/40)) ([8bec947](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/8bec94780ca2ca78cc059e854de92a79737d97f0))
* remove dashmap ([#52](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/52)) ([b381031](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b381031e263fc4c28b301ff62381757546ba505f))
* restore npm dependencies for trunk ([6128c40](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/6128c407a989cd5bfd6ba5020629c638ce7713da))
* show the uuid as the player name if no player name is found ([eaa5321](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/eaa53212bfead022e2694437c50d3be3208b610d))
* trunk tailwind setup ([7fd81ae](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7fd81aedfe7d0d99c2fe67a9d1009a1380fb859d))


### Code Refactoring

* cleanup models ([#31](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/31)) ([9ec63d3](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9ec63d364c321707f208411ac05437a6e6f43401))
* **Frontend:** split components into sub components and update h… ([#32](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/32)) ([b7948bf](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b7948bf2a34eb40550ae4de1cd195487740e2c3f))
* replace dead lzma-rs with lzma-rust2 & remove deprecated zlib ([42e92a5](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/42e92a56eff757da08602729a6702e146c80f730))
* start cleanup ([8f7ed99](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/8f7ed990c7b6efb011df16f709308f66f68f2f44))
* start splitting leaderboard components and logic ([f0d8d65](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f0d8d650cc5f99c7ce27906189d3ddfd1cd54afd))


### Styles

* auto-format rust code ([9879cd4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9879cd4f0cc5569714c4055c18ef45ffe7df2e28))
* auto-format rust code ([eaea152](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/eaea152ebb6cf1b43f35555b2e6cb1805623b38a))
* auto-format rust code ([7797d57](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7797d57128b52555032770ce6d1d4f6d3ab8d44c))


### Build System

* add basic build check action ([54ed8d4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/54ed8d4920c1eec6cb250bee3b3422436cd0085f))
* add cargo lock update action ([#57](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/57)) ([e2185d3](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/e2185d363106d44ef59fdbc8cb6cac70f2f9c08b))
* add CI automation actions ([#60](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/60)) ([26dfa9d](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/26dfa9df2f9a342602e50b60ea7fa4a8a70bc416))
* add propper docker build test ([d2a2405](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/d2a2405dc0defc82dea05395aebbad619be90a22))
* add release please ([f2dd5b4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f2dd5b4a314a8b0552091198e14ec3334ee89575))
* add renovate ([ebef276](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/ebef276dc903d2bc65c91455f670661ac1f90855))
* centralize version across all crates ([227bc2b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/227bc2b7e8de5e0659cbd896c3fb2a1fa17c688e))
* cleanup unused dependencies and update existing ones ([b4927e5](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/b4927e52882cfee1f96ce805dfb4f75765ca5bc6))
* **Data:** add data-test sub set of the global data for testing purpose ([f960737](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f960737297b3ccb5d92767c7d7eb91d226c6fd26))
* **Data:** use higher compression levels ([7900d5a](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/7900d5a13cf0c6887941e82319acfdabf6c4fbc7))
* **Docker:** add DATA_INPUT_DIRECTORY to select data input directory ([04da710](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/04da7103a56e203110f17aabdb36484a563b41d9))
* maybe this will fix release please ([67c38dd](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/67c38ddd5585abab315b8ac14f3bb5afd738ab49))
* maybe this will fix release please2 ([8ecb769](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/8ecb769b78a4ef3e292f697f593bbcdcb746e923))
* maybe this will fix release please3 ([48d942b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/48d942bbba1637b8ab60c38c3cd9fe894bd360da))
* maybe this will fix release please4 ([27c7242](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/27c724267acfab25052e1cd699b425bd55b25e53))
* maybe this will fix release please5 ([edc4139](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/edc41391478118f3c606bb88185f249cbd93b442))
* maybe this will fix release please6 ([a2319ca](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/a2319cab87930856924d1944c448a3801a6de76e))
* maybe this will fix release please7 ([c5ce385](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/c5ce385a773eb82ae6324b2de6a94bf528fefb29))
* maybe this will fix release please8 ([986a804](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/986a804fcadaa21063741f6d9edddcbabd26d0d9))
* never cancel release please action ([4e63a8d](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/4e63a8def36b9c1aae4215c43c02c934fdd80baf))
* update data sources ([8fcdd04](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/8fcdd049705ad0c410074dbb0600d9b8f91b445d))
* update Dockerfile to scratch image with cargo cheff ([#28](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/28)) ([70c098e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/70c098e1c2a15bf55f0e6bd2f00344ea35638497))
* update lock ([68fb684](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/68fb684b4ce1c88efae3adea4dd87a20e3360400))
* update lock ([dd00547](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/dd0054760ca5d24be3bc4a03713e7eae515bffe7))
* update lock ([fa70999](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/fa7099990afc76d756a38eaa191f49235f85718b))
* update version lock ([d514803](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/d514803a4be47766cc4a9aa5920311238673ab2c))
* update version lock ([9684ed5](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9684ed5a4236062a6837fc671f46d1d6fcb16cca))


### Miscellaneous

* **deps:** pin dependencies ([#4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/4)) ([c792a41](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/c792a41e73c2bf5ac98c155d3c2b45a6c86271fb))
* **deps:** update all non-major action updates ([#19](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/19)) ([04c8647](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/04c86470a85abaa71d78051b5f1aaa9c2f421e06))
* **deps:** update all non-major action updates ([#61](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/61)) ([056a4b4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/056a4b4a198b0f7ebea0122ec2db9aa4d11d568a))
* **deps:** update dependency tailwindcss to v4 ([#15](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/15)) ([33f1953](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/33f195308c4e616268b8db48767c41050f39767b))
* **deps:** update dependency tailwindcss to v4.2.0 ([#17](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/17)) ([011e117](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/011e117e2c86204f5c66e731ffe3dd2602210e8e))
* **deps:** update dependency tailwindcss to v4.2.1 ([#56](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/56)) ([df194e6](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/df194e68f02c467094e756ce1cf72cbc8aa25bba))
* **deps:** update github/codeql-action action to v4.32.4 ([#25](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/25)) ([42d5392](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/42d539266414510ce9a5406419375a17ac48cb8d))
* **deps:** update rust crate anyhow to v1.0.102 ([#22](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/22)) ([44f3289](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/44f3289809810643c38594ee4ff901c7d97d2597))
* **deps:** update rust crate axum to 0.8 ([#8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/8)) ([09c28cc](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/09c28cc6aab9c18cc893409f28b39772bf769ec3))
* **deps:** update rust crate clap to v4.5.59 ([#5](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/5)) ([67f0553](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/67f05537e08b430d449ba3d52670ec16a0472624))
* **deps:** update rust crate clap to v4.5.60 ([#21](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/21)) ([d50d3a5](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/d50d3a56bb69f4e47e8e9e3b6a43c51c849e4fa1))
* **deps:** update rust crate futures to v0.3.32 ([#6](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/6)) ([9ff13c4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9ff13c4fd458481912e9b5a32774ee9c28d2a606))
* **deps:** update rust crate indicatif to 0.18 ([#9](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/9)) ([2588a07](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/2588a07c416761cab00af4c5912a40b82e1bfa4d))
* **deps:** update rust crate smol_str to 0.3 ([#10](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/10)) ([5749ad1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/5749ad155b831e30ef18ca1e36349505fb3892b7))
* **deps:** update rust crate tower to 0.5 ([#12](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/12)) ([e3e66ce](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/e3e66ceb9eea647e638615b0b48c62f4dd93f3fe))
* **deps:** update rust crate tower-http to 0.6 ([#13](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/13)) ([43f4779](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/43f47792180a083a3f8608ac3b15f04746ac6d22))
* **deps:** update rust docker tag to v1.93 ([#14](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/14)) ([db7468e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/db7468e67329b0c34b6d857a757034b648ff793a))
* **deps:** update rust:1.93-slim docker digest to 7e6fa79 ([#46](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/46)) ([9ed9e1c](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9ed9e1c7bc72be03da04a7c46573c6493f54df5b))
* **deps:** update rust:1.93-slim docker digest to c0a38f5 ([#62](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/62)) ([84b3c6a](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/84b3c6ae18e31bb7d42d50701685059d0a391506))
* **deps:** update step-security/harden-runner action to v2.15.0 ([#47](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/47)) ([c2aeaa5](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/c2aeaa50253b62ddc4275e4fb867aced596d9c78))
* **deps:** update step-security/harden-runner action to v2.15.0 ([#59](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/59)) ([50be0cb](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/50be0cbed95eb779b196cd3a82375ebc7f471fee))
* **deps:** update to vactions-helm-update-chart-version-v1.5.1 ([#11](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/11)) ([f194175](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/f194175d3a3acf9b940a51683121641290a86891))
* **deps:** update to vactions-helm-update-chart-version-v1.5.2 ([#38](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/38)) ([e8254e3](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/e8254e3494049f573c91e573c9321bc62d401026))
* fix test failure ([61221b4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/61221b4ddae8210b2a85734891b399110af74d0b))
* **main:** release 0.2.0 ([#1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/1)) ([8a0bf21](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/8a0bf21f025c4cbc9969d6f2ac6a1734e33e8ff4))
* **main:** release 0.2.1 ([#2](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/2)) ([79ac80e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/79ac80e2193ca487958e5e25b891d4456fa028fe))
* **main:** release 0.3.0 ([#3](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/3)) ([c69f9a8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/c69f9a8cc2acadc2ceade9b0f5fe53033ee4ce29))
* **main:** release 0.3.1 ([#20](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/20)) ([4ac70b7](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/4ac70b7f9e5907a6fae2c93dfe2e7dca494092d9))
* **main:** release 0.3.2 ([#23](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/23)) ([e7d201d](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/e7d201d020a33f1043650dd36410fbeb36166e5b))
* **main:** release 0.4.0 ([#24](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/24)) ([25903e4](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/25903e4f4f5ad7699b0d95a12a000b0f58b43d2f))
* **main:** release 0.5.0 ([#27](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/27)) ([c7a2f0e](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/c7a2f0e75f1e33ad187dacd38411beb18d00a906))
* **main:** release 0.6.0 ([#29](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/29)) ([9d27294](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9d27294b44fa329e168783350ecc08c16dacb4c4))
* **main:** release 0.7.0 ([#33](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/33)) ([9e9c2da](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9e9c2daeb1685235a6e1d0c08464a695ce7fd566))
* **main:** release 0.7.1 ([#41](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/41)) ([e64b74c](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/e64b74ca8838b64dd339998730c25c928334c664))
* **main:** release 0.7.2 ([#43](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/43)) ([bfc92d8](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/bfc92d8c59d911d8f1ebf8269a4f49ae914e4849))
* **main:** release 0.8.0 ([#45](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/45)) ([eaef8a6](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/eaef8a63dbcc26ca64c0bd499967a379ec89009f))
* **main:** release 0.8.1 ([#51](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/51)) ([746f83d](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/746f83d2c39faa3a69920491365033853e1badaa))
* **main:** release 0.9.0 ([#53](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/53)) ([41ea9ac](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/41ea9ac6ede3878508e0209111ef934e1fe82bbd))
* update cargo lock ([a89386b](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/a89386b39c8abc99102ac3ea577d301c0bd109f0))
* update lock ([1ba448f](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/1ba448f7d5a94e6a44ffe7b9394acaad8c45b7a0))


### Dependencies

* **deps:** lock file maintenance ([#16](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/16)) ([9a24ae1](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/9a24ae146cd6782af4d336b8df5477b51f83727f))
* **deps:** lock file maintenance ([#35](https://github.com/TimSchoenle/mp-stats-legacy-viewer/issues/35)) ([834910f](https://github.com/TimSchoenle/mp-stats-legacy-viewer/commit/834910f245fe2a3ed1341871c95de5613ab0373b))

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
