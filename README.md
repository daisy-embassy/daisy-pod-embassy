# daisy-pod-embassy

**Not yet tested on real hardware**. I have only confirmed that it compiles successfully. If you decide to use it, please carefully verify all settings.I'm looking forward to your feedbacks!

- [daisy-seed](https://daisy.audio/hardware/Seed/)
- [daisy-pod](https://daisy.audio/product/Daisy-Pod/)
- [daisy-embassy](https://github.com/daisy-embassy/daisy-embassy)

## Run examples

```bash
# Please choose features based on which board you have.
cargo run --example=peripheral_demo --features=seed_1_2 --release
cargo run --example=peripheral_demo --features=seed_1_1 --release
```
