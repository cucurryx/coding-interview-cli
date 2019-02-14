## coding-inverviews CLI

### install

clone this repo:
```
git clone ...
```

install cargo for rust:
```
# for linux or macOS
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

run:
```
cargo run
```

### cmds

#### login or not login
There are two mode: login or not login. If you login, the submit history will be based on the remote history of you acount. And if not, it's based on your local log.

The default mode is not login.

```
# change to login mode
cinterview login

# then input username
# and input password
```
#### list
get current problems status
```
cinterview list
```

#### submit

```
# -e means exams mode
cinterview submit -e 12

# -t means test mode
cinterview submit -t 12
```

The difference between these two mode is whether you can get the error message if your code is not correct. Default it test mode.

