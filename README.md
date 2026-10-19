# Lyntrap

[![docs.rs](https://img.shields.io/docsrs/lyntrap?style=for-the-badge&label=DOCS%20BUILD)](https://docs.rs/lyntrap/latest/lyntrap)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ItsThatOneJack-Dev/lyntrap/ci.yml?style=for-the-badge&label=CODE%20BUILD)](https://github.com/ItsThatOneJack-Dev/lyntrap/actions/)

[![Crates.io Size](https://img.shields.io/crates/size/lyntrap?style=for-the-badge&label=CRATE%20SIZE)](https://crates.io/crates/lyntrap)
[![GitHub repo size](https://img.shields.io/github/repo-size/ItsThatOneJack-Dev/lyntrap?style=for-the-badge&label=REPO%20SIZE)](https://github.com/ItsThatOneJack-Dev/lyntrap)
[![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/ItsThatOneJack-Dev/lyntrap?style=for-the-badge&label=CODE%20SIZE)](https://github.com/ItsThatOneJack-Dev/lyntrap)

> [!WARNING]
> Lyntrap is deprecated as of version 0.1.2 on the 19th of October, 2026, and will no longer receive any updates.
> As part of this deprecation, I hereby release version 0.1.2 of Lyntrap under the MIT license, instead of the MPL license as was previously used.

> [!TIP]
> If you are looking for Lyntrap's functionality, I would recommend `lyntr-rs`, which is [available on crates.io](https://crates.io/crates/lyntr-rs), it behaves much more predictably and is a great deal more stable than Lyntrap has ever been.

---

> [!CAUTION]
> Lyntrap was designed solely to be used in Termynter, but during the planning phase I realised its potential use by others, and decided to release it as its own project, but with no guarantees.

> [!NOTE]
> It is known that Lyntrap fails on some endpoints, this has been determined to be due to issues parsing response bodies, no other information is known, as work on fixing it has been halted due to the deprecation of Lyntrap.

Lyntrap is a fully-fledged Rust API client for the social media [Lyntr](https://lyntr.gizmowizard.tech/)!

The name is a bit unusual, there are multiple "correct" (liguistic presciptivism can be bad) ways to say it, I (ItsThatOneJack) officially approve:

- "Lynt trap" (Unlike dryer lint's properties as a good firestarter, please don't start fires using Lyntrap, it would make me sad if you burnt down the Amazon rainforest)
- "Lyntr AP" (Advanced Placement, very fancy and educated)

Or, for you contrarians:

- "Lyn trap"
- "Lynt rap"

---

You can see the Lyntrap documentation on [docs.rs](https://docs.rs/lyntrap/latest/lyntrap)!

Happy Lynting!

## Synchronicity

Lyntrap can behave synchronously or asynchronously, depending on what you need! By default, it ships only able to behave asynchronously, however if you want synchronous behaviour, you can use it by simply installing it with the optional feature `blocking`.

## Testing

If you wish to run a test on Lyntrap, as of today (27th of September, 2026) there are two available, both testing the same thing but on different versions of Lyntrap.

You can use `live_api_async` to perform a full test on the system's ability to communicate with the Lyntr API asynchronously. Additionally, you can alternatively use `live_api_blocking` to perform a full test on the system's ability to communicate with the Lyntr API synchronously.

When testing either of the above, please set the `LYNTR_CLIENT_ID` and `LYNTR_CLIENT_SECRET` environment variables, this communicates the details to test with to the test script.
