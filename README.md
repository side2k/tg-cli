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
