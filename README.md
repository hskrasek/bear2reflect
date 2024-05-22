> [!IMPORTANT]
>
> ## 🇵🇸 Support Palestine 🇵🇸
>
> In light of recent events in Gaza, I encourage everyone to educate themselves on the ongoing issues in Palestine and consider supporting the people there. Here are some resources and donation links:
>
> - [Decolonize Palestine](https://decolonizepalestine.com/) - An informative resource to better understand the situation in Palestine. Please take the time to read it.
> - [One Ummah - Gaza Emergency Appeal](https://donate.oneummah.org.uk/gazaemergencyappeal48427259) - A platform to provide direct donations to help the people in Gaza.
> - [Islamic Relief US - Palestine Appeal](https://islamic-relief.org/appeals/palestine-emergency-appeal/) - Another trusted platform to provide support for those affected in Palestine.
>
> Thank you for taking a moment to bring awareness and make a difference. 🇵🇸❤️

# Bear2Reflect

Bear2Reflect is a tool written in Rust to help you convert your notes from [Bear](https://bear.app/) to [Reflect](https://reflect.app).
This is my first foray into building a CLI tool in Rust, as I usually work in PHP, but I wanted to stretch my coding wings and do something different.

Bear2Reflect is compiled to run on both Intel macOS and Apple Silicon macOS. You can find the latest binaries in the [releases section](https://github.com/hskrasek/bear2reflect/releases).

## Currently Supported Features

- [x] Graph Selection
- [x] Converting Root and Nested tags into backlinks

## Planned Features

I have a few features in mind that I'd like to implement in the future, but at the moment I'm stopping at the bare minimum I personally need.
If you have any feature requests, please open an issue and I'll see what I can do.

> [!NOTE]
> Some of these planned features may not be possible due to limitations in the Reflect API

- [ ] Converting nested tags into backlinks and tags (e.g. `#tag/subtag` becomes `[[tag]] #subtag`, or maybe even `#tag [[subtag]]`) 
- [ ] Attaching files and other media in Bear notes to Reflect notes
  - This may not be possible due to limitations in the Reflect API
- [ ] Supporting the conversion of Bear's own backlinks into Reflect backlinks
  - I did not realize Bear added backlinks at some point. If/when this is implemented, the approach will include converting only backlinks, and leaving tags as is (or converting to whatever syntax Reflect needs)
