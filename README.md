# coding-interview CLI

My goal is to implement a CLI tool like [leetcode-cli](https://github.com/skygragon/leetcode-cli). And my teammate([@曾德勤
](https://github.com/Zendq1998)) will wrap it to a 
Virsual Studio Code extension like [this](https://github.com/jdneo/vscode-leetcode).

## install

clone this repo, install rust and cargo, then build or run.
```
# install cargo and rust for linux or macOS
curl -sSf https://static.rust-lang.org/rustup.sh | sh

git clone https://github.com/xiebei1108/coding-interview-cli
cd coding-interview-cli
cargo build --release
sudo cp ./target/release/cinterview /usr/local/bin
```

## commands

### login or not login
**Login mode is to be implemented**

There are two mode: login or not login. If you login, the submit history will be based on the remote history of you acount. And if not, it's based on your local log.

The default mode is not login.

```
# change to login mode
cinterview login

# then input username
# and input password
```

### init && list
```
# download problem details from network. And coding templates will be generated in the current directory.
cinterview init

# get current problems status
cinterview list
```

For example:
![./pics/1550304472530.jpg](./pics/1550304472530.jpg)

### submit 
```
# -e means exams mode
# the first arg after `submit` should be the suffix of the lanugage in which 
# you're going to submit the code.
# includes: cc, py, java, php, js, cs ...

cinterview submit -e cc 1 2 3

# -t means test mode
cinterview submit -t java 12
```

The difference between these two mode is whether you can get the error message if your code is not correct. Default it test mode.


### clean
Clean local problems data.
```
cinterview clean
```

## Todo
- [x] support all languages
- [ ] more readable output
- [ ] login
- [ ] generate submit code from test code automatically
- [ ] vscode extension
