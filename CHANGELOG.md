# Changelog

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
