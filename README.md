# Outdoor

Outdoor is a daemon that pipes current weather information from [OpenWeatherMap](https://openweathermap.org/)
 into ~~BigClown~~ [Hardwario](https://www.hardwario.com/) IoT stack. It complements home climatic sensors.

## Warning
This an educational project. It helps me to learn Rust while doing a project that makes sense. Most likely you know rust much better than I do.

## Getting Started

A recent cargo is required to compile the project. It uses async therefore circa a version 1.40 is necessary. Just run:
```bash
cargo build --release
```

### Installing

Compilation produces a mostly static binary (openssl may be required in runtime).
1. _Optionally_ strip debug information: `strip target/release/outdoor`
2. Copy `target/release/outdoor` to `/usr/local/bin/outdoor`
3. Install the systemd service file:
```bash
cp resources/outdoor.service /etc/systemd/system/outdoor.service
cp -r resources/outdoor.service.d/ /etc/systemd/system/outdoor.service.d
```
4. Configure the service in `/etc/systemd/system/outdoor.service.d/local.conf`. See all the options
 by running `/usr/local/bin/outdoor --help`.

5. Refresh systemd and start the service
```bash
systemctl daemon-reload
systemctl start outdoor.service
systemctl status outdoor.service
```

6. If everything works, allow `outdoor.service` to run on start
```bash
systemctl enable outdoor.service
```

## Running the tests

Some parts of the code are covered by tests. To check them, run: 
```bash
cargo test
```

## Cross compiling 
 - how to run on Turris Omnia?

1. Test with `cross test --target armv7-unknown-linux-musleabihf`.
2. Compile with `cross build --target armv7-unknown-linux-musleabihf --release`
3. Strip the binary:
```bash
docker run --rm -it \
    -v $PWD/target/armv7-unknown-linux-musleabihf/release/:/project \
    rustembedded/cross:armv7-unknown-linux-musleabihf-0.2.0 \
    /usr/local/arm-linux-musleabihf/bin/strip /project/outdoor
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details


## Contributing
You may submit pull requests, it is open source :smiley:

- Please respect the limited scope of this project. The use case is fairly limited. 
- Explain the reasoning behind the PR. It may not be obvious to novice rustaceans such as me.
- Please dedicate the pull request to single change. It makes no sense to change dependecies,
 do refactoring and change code style at once.
- Write tests for fragile pieces of code.
- Structure your commits, name them clearly.
- Run `rustfmt` where possible.
- Try to avoid creating new clippy warnings.
- Be nice :smiley: and have fun.

## Acknowledgments

* Thanks _Broderick Carlin_ for the [openweather crate](https://github.com/BroderickCarlin/openweather).
 As it has not been updated to async yet, I took the liberty of copying essential
 parts of the weather API client code for `outdoor`. 
* Thanks ~~Bigclown~~ Hardwario IoT platform for its open and well documented design. It allowed me to focus on the rust part of development.
* Thanks OpenWeather for the weather information and a free API tier.
