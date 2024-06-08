# commands-manager

commands-manager is a little soft writen in Rust allowing you to store, manage and run commands you always use.

At office I work with lot of virtual machines and servers, so I am using this soft to fastly open ssh sessions with Putty or FTP sessions with Filezilla to the machine I need to work on.

## Installation

If your operating system is a windows 64bit or a Mac with the new silicon chips (M1, M2, ...) you just have to download and run the executables files in the bin folder, otherwise you will have to build the project yourself.

To do so, you first have to make sure that you have a working installation of rust and cargo. You can find the installations instructions [here](https://doc.rust-lang.org/book/ch01-01-installation.html)

Once you have rust and cargo installed, simply download this repository, as a zip or by cloning it using git, navigate inside this folder inside a command line interpreter (cmd, iTerm, gnome-terminal, ...) and run this command :

```bash
cargo build --release
```

It can takes a few minutes to complete, once this is done you will have a new folder called "target", go in it, open the folder called "release" and inside should be your binary file ready to run

## Usage

Usage is really simple. You need to create a file called `Config.toml` in the same folder than your binary file.
You have an example provided with this repository, (here)[https://github.com/BastienLBCH/commands-manager/blob/master/Config.toml].

Fill it with the commands you want to have in the application.

> [!TIP]
> On windows I recommend to separate each argument in the list like this :
> ```toml
>server2 = ['config', 'example', '1-2']
> ```
>
> But on linux/unix systems I rocommend to have all your command in only one string :
> ```toml
>server1 = ['config example 1-1']
>```

Once your `Config.toml` file is ready, simply start your binary.

Sections are closed by default, simply click on a section to open or close it, then double click on a command to run it.


## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

MIT License

Copyright (c) [2024] [Labouche Bastien]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
