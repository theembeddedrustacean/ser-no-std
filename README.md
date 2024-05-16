# Simplified Embedded Rust: ESP Core Library Edition - Book Repository 🦀 

<p align="center">
<img src="https://media.beehiiv.com/cdn-cgi/image/fit=scale-down,format=auto,onerror=redirect,quality=80/uploads/asset/file/5ab65db3-4818-4fc3-942e-c18e795b65d4/2.png?t=1714653599" alt="BookCover" width="200"/>
</p>

Welcome to the _**Simplified Embedded Rust: ESP Core Library Edition**_ book repository. Here you will find all the book related resources. You can get a copy of the book [here](https://www.theembeddedrustacean.com/c/ser-no-std).

## 📝 Reporting Issues & Content Suggestions
If you find any text errors, typos, or formatting issues in the book, please [report a text error here](https://github.com/theembeddedrustacean/ser-no-std/issues/new?assignees=&labels=documentation&projects=&template=text-error.md&title=) so that it can be addressed in a later revision. 

If you find any code issues in the book, please [report a bug here](https://github.com/theembeddedrustacean/ser-no-std/issues/new?assignees=&labels=bug&projects=&template=bug_report.md&title=) so that it can be addressed in a later revision. 

You are also welcome to [suggest a feature here](https://github.com/theembeddedrustacean/ser-no-std/issues/new?assignees=&labels=enhancement&projects=&template=feature_request.md&title=) so it may be considered for content in the future.

## 🔗 Links to Book Examples
| Device   | Devkit | GitHub Links | Wokwi Links |
| -------- | ------ | ------------ | ----------- |
| ESP32-C3 | [ESP32-C3-DevKitM-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitm-1.html) | [ESP32-C3-DevKitM-1 Branch](#) | **CH 6 - GPIO**<br>[Blinky](https://wokwi.com/projects/391678247863083009)<br>[Button Controlled Blinking](https://wokwi.com/projects/391678361146008577)<br>[Button Press Counter](https://wokwi.com/projects/391678393219863553)<br>**CH 7 - ADC**<br>[Simple Voltmeter](https://wokwi.com/projects/391678430405517313)<br>[Temperature Sensing](https://wokwi.com/projects/391678465337780225)<br>**CH 8 - Timers & Counters**<br>[Timer Based Delay](https://wokwi.com/projects/391678495008295937)<br>[Real-Time Timer](https://wokwi.com/projects/391678635458788353)<br>**CH 9 - PWM**<br>[LED Fading](https://wokwi.com/projects/391678663139101697)<br>**CH 10 - Serial Communication**<br>[Console Printing](https://wokwi.com/projects/391678698454099969)<br>[Interacting with an I2C RTC](https://wokwi.com/projects/391678723554917377)<br>**CH 11 - The Embassy Framework**<br>[Blinky](https://wokwi.com/projects/391678752889882625)<br>[Real-time Timer](https://wokwi.com/projects/391678776862429185)<br>[LED Cycling](https://wokwi.com/projects/391678800985971713)<br>[UART Echo](https://wokwi.com/projects/391678832909874177) |
| ESP32-C3 | [ESP32-C3-DevKit-RUST-1](https://github.com/esp-rs/esp-rust-board/tree/v1.2) | 🚫 | 🚫 |
| ESP32-S2 | [ESP32-S2-DevKitM-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32s2/hw-reference/esp32s2/user-guide-devkitm-1-v1.html) | 🚫 | 🚫 |
| ESP32-S3 | [ESP32-S3-DevKitC-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/hw-reference/esp32s3/user-guide-devkitc-1.html)    | 🚫 | 🚫 |
| ESP32    | [ESP32-Dev-KitC](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/hw-reference/esp32/get-started-devkitc.html#get-started-esp32-devkitc-board-front) | 🚫 | 🚫 |

## 🔌 Wiring Templates for End of Chapter Questions
These are pre-wired templates to get you started with end of chapter questions. Questions that are not included would use exisiting book examples as templates. If you notice a template missing or would like to request one, feel free to submit a feature request. 

|Device  |Dev Kit  |CH 5 <br> GPIO|CH 6 <br> ADCs |CH7 <br> Timers & Counters |CH8 <br> PWM |CH9 <br> Serial Comms|CH10 <br> Embassy|
|--------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------|---------------------------------------------------|
|ESP32-C3|[ESP32-C3-DevKitM-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitm-1.html)                             |[Q5](https://wokwi.com/projects/397210832311932929)<br>[Q6](https://wokwi.com/projects/397210887966166017)<br>[Q7 & Q8](https://wokwi.com/projects/397210935209768961)|[Q4](https://wokwi.com/projects/397211014469548033)<br>[Q5](https://wokwi.com/projects/397211055233995777)<br>[Q6 & Q7](https://wokwi.com/projects/397702807168989185)|[Q1](https://wokwi.com/projects/397211229389919233)|[Q4](https://wokwi.com/projects/397211366137888769)<br>[Q5](https://wokwi.com/projects/397211392956273665)<br>[Q6](https://wokwi.com/projects/397211424007755777)<br>[Q7](https://wokwi.com/projects/397211461042421761)|🤷‍♂️            |[Q6](https://wokwi.com/projects/397211545848127489)|
|ESP32-C3|[ESP32-C3-DevKit-RUST-1](https://github.com/esp-rs/esp-rust-board/tree/v1.2)    |🚫 |🚫 |🚫 |🚫 |🚫 |🚫 |
|ESP32-S2|[ESP32-S2-DevKitM-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32s2/hw-reference/esp32s2/user-guide-devkitm-1-v1.html)   |🚫 |🚫 |🚫 |🚫 |🚫 |🚫 |
|ESP32-S3|[ESP32-S3-DevKitC-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/hw-reference/esp32s3/user-guide-devkitc-1.html)  |🚫 |🚫 |🚫 |🚫 |🚫 |🚫|
|ESP32   |[ESP32-Dev-KitC](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/hw-reference/esp32/get-started-devkitc.html#get-started-esp32-devkitc-board-front)|🚫|🚫|🚫|🚫|🚫|🚫|

## 🧑‍💻 Development Options

### 1. 🌐 Wokwi Web Interface (Recommended)
This is the recommended option for beginners as it is the quickest and easiest way to get started. Each code example has a corresponding project on [Wokwi](https://wokwi.com/), allowing you to run and modify the code directly from your web browser. Links to the examples are provided in the earlier section.

### 2. 🌐🛠️ Wokwi Web Interface with Hardware
If you prefer to work with physical hardware, Wokwi provides an approach to flash an external development board/device directly from the web interface. To do that you need to press F1 in the Wokwi code window to access the command palette. Afterward, you can select the "Flash Firmware to Device" option.

### 3. 🏡🛠️ Local Editor with Physical Hardware  
If you prefer to develop locally with physical hardware, you can clone the examples locally and set them up to run on an external development board. The software required entails installing the nightly toolchain with the rust src component, modifying the target for cross-compiling in addition to installing flashing tools to download code to the external hardware. This includes the following

#### a) **Install Rust** 🦀: 
If you do not have Rust installed already, follow the instructions on the [rustup
website](https://www.rust-lang.org/tools/install).

#### b) **Install the Nightly Toolchain with the `rust-src` Component**: 
Run the following command in a terminal window:

```shell
rustup toolchain install nightly --component rust-src
```

#### c) **Set the target**: 
Run the following command in a terminal window:

```shell
rustup target add riscv32imc -unknown -none -elf
```

#### d) **Install `espflash`**: 
Run the following command in a terminal window:
```shell
cargo install espflash
```

Afterward, the easiest way to flash an ELF binary, is to add `espflash` as your Cargo runner. This way, when enterning `cargo run`, the code would automaitcally perform the flashing after file generation. This is done by adding the following line to your `.cargo/config.toml` file:

```shell
[target.'cfg(any(target_arch = "riscv32", target_arch = "xtensa"))']
runner = "espflash flash --monitor"
```
#### e) **Install `cargo-generate`**:
When creating your own projects from scratch, it is highly recommended that you use `cargo-generate`. Through `cargo-generate` you can create new project templates pre-configured for any ESP device. Click on the link below for instructions to install and use `cargo-generate`.
To install `cargo-generate` run the following command:

```shell
cargo install cargo-generate
```
Afterward, to generate a `no-std` template run the following command:

```shell
cargo generate esp-rs/esp-template
```

### 4. 🏡🔮 Local Editor with Wokwi
If you prefer to develop locally with Wowki (no hardware), you can clone the examples locally and install the following extensions for the simulator:
- [VSCode Wokwi Extension](https://docs.wokwi.com/vscode/getting-started)
- [JetBrains Wokwi Plugin](https://plugins.jetbrains.com/plugin/23826-wokwi-simulator)

Afterward, you need to also configure your project for Wokwi using [these](https://docs.wokwi.com/vscode/project-config) instructions.

⚠️ Local development with Wokwi still requires the same setup as local editor with physical hardware since you will be compiling locally, however, flashing tools are not required since external hardware is not involved. Also through `cargo-generate` you can generate projects pre-configured for Wokwi skipping the second part after extension installation.

### ⛔️ Important Note: 
Options 2, 3, and 4 are not recommended for beginners due to the added complexity and, in some cases, required installations.

## 🧱 Hardware Component List (Optional)
This is a list of the components used in the different examples in the book. Acquiring these components is **OPTIONAL** and recommended only after you are comfortable with the material. You will only need these components if you are interested in doing physical hardware development at a later time (options 2 and 3 listed in the development options section earlier).

| Component                      | Documenation                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Purchase Links                                                                                                                               |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| Development Board (Choose one) | [ESP32-C3-DevKitM-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitm-1.html)<br>[ESP32-C3-DevKit-RUST-1](https://github.com/esp-rs/esp-rust-board/tree/v1.2) | ESP32-C3-DevKitM-1 ([AliExpress](https://www.aliexpress.com/item/3256803802784795.html?gps-id=pcStoreJustForYou&scm=1007.23125.137358.0&scm_id=1007.23125.137358.0&scm-url=1007.23125.137358.0&pvid=887074bd-9830-45ec-a9a0-e51a3b262eaf&_t=gps-id:pcStoreJustForYou,scm-url:1007.23125.137358.0,pvid:887074bd-9830-45ec-a9a0-e51a3b262eaf,tpp_buckets:668%232846%238108%231977&pdp_npi=4%40dis%21USD%218.00%218.00%21%21%218.00%218.00%21%402101c5a417149333693898144eafe8%2112000027657818087%21rec%21US%214083593659%21AB&spm=a2g0o.store_pc_home.smartJustForYou_2008854986518.1005003989099547))<br>ESP32-C3-DevKit-RUST-1 ([AliExpress](https://www.aliexpress.com/item/3256804232027536.html?spm=a2g0o.productlist.main.3.16a72kn92kn9EZ&algo_pvid=2e8dd822-5908-4691-82bb-2d41220563ec&algo_exp_id=2e8dd822-5908-4691-82bb-2d41220563ec-1&pdp_npi=4%40dis%21USD%2119.80%2119.80%21%21%2119.80%2119.80%21%402103200517149332747751850ee5b9%2112000029115522071%21sea%21US%214083593659%21AB&curPageLogUid=mZH5BE9nmh7P&utparam-url=scene%3Asearch%7Cquery_from%3A))|
| Female to Male Wires           | N/A                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | [AliExpress](https://s.click.aliexpress.com/e/_DcZBsT1)                                                                                                                                   |
| Prototyping Breadboard         | N/A                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | [AliExpress](https://s.click.aliexpress.com/e/_Dcw29Sj)                                                                                                                                   |
| LEDs                           | [Datasheet](https://components101.com/diodes/5mm-round-led)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_DkzxbBz)                                                                                                                                   |
| LED Bar                        | [Datasheet](https://components101.com/displays/led-bar-graph)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_Dd9Kx4n)                                                                                                                                   |
| Push Button                    | [Datasheet](https://components101.com/switches/push-button)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_DBC4OrZ)                                                                                                                                   |
| Potentiometer                  | [Datasheet](https://components101.com/resistors/potentiometer)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_DdlX2Hz)                                                                                                                                   |
| NTC Temperature Sensor                      | [Datasheet](https://components101.com/resistors/ntc-thermistor-10k)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_DDejccb)                                                                                                                                   |
| DS1307                         | [Datasheet](https://components101.com/ics/ds1307-i2c-real-time-clock-rtc)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | [AliExpress](https://s.click.aliexpress.com/e/_DEWxS7v)                                                                                                                                   |
