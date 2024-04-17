# tg-cli - a simple CLI tool for Telegram

Work in progress.
```
Usage: tg-cli [OPTIONS] --api-id <API_ID> --api-hash <API_HASH> <COMMAND>

Commands:
  login  Logs into the telegram account, saving session file
  help   Print this message or the help of the given subcommand(s)

Options:
  -s, --session-file <SESSION_FILE>  [default: .session]
      --api-id <API_ID>              [env: API_ID=12345678]
      --api-hash <API_HASH>          [env: API_HASH=abcde123456abcde]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Commands

### login

Log in and save session. `tg-cli` will ask for verification code and password, if 2-step verification is enabled.

Example:
```
$ export TGCLI_PASSWORD=mysuperpassword
$ tg-cli login --phone +79001234567
```

### list-dialogs

Put all dialog, their types (User/Group/Channel), ids and titles to stdout.

Example:
```
$ tg-cli list-dialogs

Listing 4 dialogs:
Group 1839823152 Old School Cruisers
Group 2223334444 Party makers
Channel 1234567899 Some interesting channel
User 111333 Telegram

```

Options:
  * `--filter <FILTER>` - list only dialogs that contain `FILTER` in their titles (case insensitive). If `FILTER` starts with `@` - dialogs with matching usernames will be shown


### msg \<dialog_id\> \<message\>

Example:
```
$ tg-cli msg 12345678 "hey there!"
```

Note: dialog_id can be obtained using list-dialogs command
