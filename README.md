# Lyntrap

Lyntrap is a fully-fledged Rust API client for the social media [Lyntr](https://lyntr.gizmowizard.tech/)!

The name is a bit unusual, there are multiple "correct" (liguistic presciptivism can be bad) ways to say it, I (ItsThatOneJack) officially approve:

- "Lynt trap" (Unlike dryer lint's properties as a good firestarter, please don't start fires using Lyntrap, it would make me sad if you burnt down the Amazon rainforest)
- "Lyntr AP" (Advanced Placement, very fancy and educated)

Or, for you contrarians:

- "Lyn trap"
- "Lynt rap"

## Synchronicity

Lyntrap can behave synchronously or asynchronously, depending on what you need! By default, it ships only able to behave asynchronously, however if you want synchronous behaviour, you can use it by simply installing it with the optional feature `blocking`.

## Testing

If you wish to run a test on Lyntrap, as of today (27th of September, 2026) there are two available, both testing the same thing but on different versions of Lyntrap.

You can use `live_api_async` to perform a full test on the system's ability to communicate with the Lyntr API asynchronously.
You can use `live_api_blocking` to perform a full test on the system's ability to communicate with the Lyntr API synchronously.

When testing either of the above, please set the `LYNTR_CLIENT_ID` and `LYNTR_CLIENT_SECRET` environment variables, this communicates the details to test with to the test script.
